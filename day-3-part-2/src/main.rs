use std::{str::Split, ops::Range, collections::HashSet, fs::read_to_string, path::Path};
use regex::Match;

fn main() {
    let input = read_to_string(Path::new(
        "/Users/darioalessandro/Documents/advent-of-code-2023/day-3/src/input.txt",
    ))
    .unwrap();
    let actual_part_numbers =find_part_numbers(&input);


    let assertions = vec!((84, 1), (566, 2), (585, 4), (965, 4), (353, 5), (280, 2), (75, 3), (869, 2), (798, 1), (145, 1), (629, 1), (579, 2), (837, 2), (2, 1), (658, 2), (869, 2));
    for (part_number, matches) in assertions {
        let result : Vec<_> = actual_part_numbers.iter().filter(|part| part.1 == part_number).collect();
        assert_eq!(result.len(), matches, "part number {}", part_number);
    }

    let actual_part_numbers: Vec<u32> = actual_part_numbers.iter().map(|a| {
        a.1
    }).collect();


    // add then together
    let actual_sum: u32 = actual_part_numbers.iter().sum();
    println!("sum {}", actual_sum);
}

type SymbolLocation = (usize, usize, char);

fn find_part_in_row(current_row: &Vec<char>, y_p: usize) -> u32 {
    // search to the left to find the begining of the part number
    let mut y_p = y_p;
    while y_p >= 1 {
        if current_row.get(y_p - 1).unwrap().is_digit(10) {
            if y_p > 0 {
                y_p=y_p-1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    let mut part_number = String::new();
    while  y_p < current_row.len() - 1 {
        let char = current_row.get(y_p).unwrap();
        if char.is_digit(10) {
            part_number.push(*char);
            y_p = y_p + 1;
        } else {
            break;
        }
    }
    part_number.parse::<u32>().unwrap_or(0u32)
}

fn find_parts_around_symbol(rows: &Split<char>, x: usize, y: usize) -> Vec<(SymbolLocation, u32)> {
    let mut symbol_coordinates = Vec::new();
    let rows: Vec<Vec<char>> = rows.clone().map(|row| {
        row.chars().collect()
    }).collect();
    // lets start from the top left
    // range for y is -1, 0, +1
    // range for x is -1, 0, +1
    let range_x: Range<i16> = -1..2;
    let range_y: Range<i16> = -1..2;
    for xx in range_x.clone() {
        for yy in range_y.clone() {
            if xx == 0 && yy == 0 {
                // this is the symbol location, skip
                continue;
            }
            let x_p = x as i16 + xx;
            let y_p = y as i16 + yy;

            let x_p = if x_p.is_negative()  {
                0usize
            } else {
                x_p as usize
            };
            let y_p = if y_p.is_negative() {
                0usize
            } else {
                y_p as usize
            };


            if x_p as usize > rows.len() - 1 {
                continue;
            }
            if y_p as usize > rows.len() - 1 {
                continue;
            }
            // visit coordinate and test for digit
            let current_row = rows.get(x_p).unwrap();
            let visited_caracter = current_row.get(y_p).unwrap();
            // We found a part_number
            if visited_caracter.is_digit(10) {
                let parsed_part_number: u32 = find_part_in_row(&current_row, y_p);
                let symbol_location = (x, y, ' ');
                symbol_coordinates.push((symbol_location, parsed_part_number));
            }
        }
    }

    symbol_coordinates   
}

fn find_part_numbers(input: &str) -> Vec<(SymbolLocation,u32)> {
    // 1. Find coordinates of symbols
    // If you find a symbol, then look for a digit (part number) on coordinates
    // (x, y) search in (x-1, y+1), (x, y+1), (x+1, y+1), ... (x+1, y-1) (8 adjacent positions)
    // if you find a digit, then try to parse the part number
    // if you find a digit at (s, b) s+1, s-1
    let mut found_symbols_coordinates = Vec::<SymbolLocation>::new();
    let mut part_numbers = Vec::<(SymbolLocation, u32)>::new();
    let rows = input.split('\n');
    for (x, row) in rows.clone().enumerate() {
        // find all numbers in a row.
        for (y, char) in row.chars().enumerate() {
            if !char.is_digit(10) && char != '.' {
                // found symbol
                found_symbols_coordinates.push((x, y, char));
                // find part numbers around coordinates
                part_numbers.append(&mut find_parts_around_symbol(&rows, x, y));
            }
        }
    }
    // remove duplicate part numbers, meaning, we found the same number around the same symbol
    // sort them
    part_numbers.sort_by(|a, b| {
       a.1.cmp(&b.1) 
    });

    // remove dups
    let mut unique_parts= HashSet::new();

    for part in part_numbers.iter() {
        unique_parts.insert(*part);
    }
    unique_parts.into_iter().collect()
}


 
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use regex::{Regex, Match};

    use super::*;

    #[test]
    fn test_find_all_numbers_in_row() {
        let input = "...731..484....875..#....................*243...681.................135..191.162.688%.........29.....*............688..........437......*..";

        let number_regex = Regex::new(r"([0-9])+").unwrap();
        let matches: Vec<_> = number_regex.find_iter(input).collect();

        println!("matches {:?}", matches);

        let symbol_regex = Regex::new("([^0-9,.])+").unwrap();
        let matches: Vec<_> = symbol_regex.find_iter(input).collect();
        println!("symbols {:?}", matches);
    }

    #[test]
    fn part_1() {
        let input = read_to_string(Path::new(
            "/Users/darioalessandro/Documents/advent-of-code-2023/day-3/src/input.txt",
        ))
        .unwrap();

        let number_regex = Regex::new(r"([0-9])+").unwrap();
        let symbol_regex = Regex::new("([^0-9,.])+").unwrap();
        
        let mut all_symbols: Vec<(Match, usize)> = Vec::new();
        let mut all_codes: Vec<(Match, usize)> = Vec::new();
        for (row, input) in input.split('\n').enumerate() {
            // build symbols and part numbers matrix.
            let mut codes: Vec<_> = number_regex.find_iter(&input).map(|m| {
                (m, row)
            }).collect();
            // println!("matches {:?}", codes);

            let mut symbols: Vec<_> = symbol_regex.find_iter(input).map(|m| {
                (m, row)
            }).collect();
            // println!("symbols {:?}", symbols);
            all_symbols.append(&mut symbols);
            all_codes.append(&mut codes);
        }
    
        // if there's a symbol near each code, then add it to the list of final opcodes
        let mut list_of_final_codes: HashMap<String, Match> = HashMap::new();
        // iterate though all codes and see if there is a symbol nearby
        for (code, code_row) in all_codes {
            for (symbol, symbol_row) in all_symbols.clone() {
                // if symbol near code, then add it to list_of_final_codes
                // 1. check that symbol is in adjacent row
                if  code.as_str() == "5" {
                    println!("print symbols {:?}", symbol);
                }
                if symbol_row.abs_diff(code_row) <= 1 {
                    //  345
                    //  .*.

                    // 345
                    //*....
                    // extend range of the symbol to cover diagonal matches
                    let mut symbol_range_start = symbol.range().start;
                    if symbol.range().start > 0 {
                        symbol_range_start = symbol_range_start - 1;
                    }
                    let symbol_range_end = symbol.range().start+1;
                    let code_range = code.range();
                    if code_range.contains(&symbol_range_start) || code_range.contains(&symbol.range().start) || code_range.contains(&symbol_range_end) {
                        let unique_string = format!("start:{} end: {} row: {} value: {}", code.range().start, code.range().end, code_row, symbol.as_str());
                        list_of_final_codes.insert(unique_string, code);
                    }
                }
            }
        }
        // println!("list of final codes {:?}", list_of_final_codes.len());

        // add them all!
        let mut regex_solution: Vec<u32> = list_of_final_codes.values().map(|val| val.as_str().parse::<u32>().unwrap()).collect();
        regex_solution.sort();
        let mut old_fasion_solution: Vec<u32> =find_part_numbers(&input).iter().map(|old| old.1).collect();      
        old_fasion_solution.sort();

        println!("regex {:?}", regex_solution);
        let sum: u32 = regex_solution.iter().sum();
        println!("sum {}", sum);

    }

    #[test]
    fn part_2() {
        let input = read_to_string(Path::new(
            "/Users/darioalessandro/Documents/advent-of-code-2023/day-3/src/input.txt",
        ))
        .unwrap();

        let number_regex = Regex::new(r"([0-9])+").unwrap();
        let symbol_regex = Regex::new("([^0-9,.])+").unwrap();
        
        let mut all_symbols: Vec<(Match, usize)> = Vec::new();
        let mut all_codes: Vec<(Match, usize)> = Vec::new();
        for (row, input) in input.split('\n').enumerate() {
            // build symbols and part numbers matrix.
            let mut codes: Vec<_> = number_regex.find_iter(&input).map(|m| {
                (m, row)
            }).collect();
            // println!("matches {:?}", codes);

            let mut symbols: Vec<_> = symbol_regex.find_iter(input).map(|m| {
                (m, row)
            }).collect();
            // println!("symbols {:?}", symbols);
            all_symbols.append(&mut symbols);
            all_codes.append(&mut codes);
        }
    
        // if there's a symbol near each code, then add it to the list of final opcodes
        let mut list_of_final_codes: HashMap<String, Match> = HashMap::new();
        let mut gears: Vec<Vec<(String, Match)>> = Vec::new();
        // iterate though all codes and see if there is a symbol nearby
        for (symbol, symbol_row) in all_symbols.clone() {
            let mut list_of_new_discovered_codes :Vec<(String, Match)> = Vec::new();
            for (code, code_row) in all_codes.clone() {
                if symbol_row.abs_diff(code_row) <= 1 {
                    let mut symbol_range_start = symbol.range().start;
                    if symbol.range().start > 0 {
                        symbol_range_start = symbol_range_start - 1;
                    }
                    let symbol_range_end = symbol.range().start+1;
                    let code_range = code.range();
                    if code_range.contains(&symbol_range_start) || code_range.contains(&symbol.range().start) || code_range.contains(&symbol_range_end) {
                        let unique_string = format!("start:{} end: {} row: {} value: {}", code.range().start, code.range().end, code_row, symbol.as_str());
                        list_of_new_discovered_codes.push((unique_string, code));
                    }
                }
            }
            // if we found a gear, then multiply it
            if symbol.as_str() == "*" && list_of_new_discovered_codes.len() == 2 {
                gears.push(list_of_new_discovered_codes.clone());
            }
            
            // regardless insert all codes into list of final codes.
            for (string, code) in list_of_new_discovered_codes {
                list_of_final_codes.insert(string, code);
            }
        }

        // add them all!
        let mut regex_solution: Vec<u32> = list_of_final_codes.values().map(|val| val.as_str().parse::<u32>().unwrap()).collect();
        regex_solution.sort();
        let mut old_fasion_solution: Vec<u32> =find_part_numbers(&input).iter().map(|old| old.1).collect();      
        old_fasion_solution.sort();

        // println!("regex {:?}", regex_solution);
        let sum: u32 = regex_solution.iter().sum();
        println!("sum {}", sum);
        assert_eq!(520135, sum);

        println!("all them gears {:?}", gears);
        let gears_sum = gears.iter().fold(0u32, |acc, gear| {
            let product: u32 = gear.iter().map(|(key, match_result)| {
                let number: u32 =  match_result.as_str().parse().unwrap();
                number
            }).product();
            acc + product
        });

        println!("gears sum {}", gears_sum);

        // multip
    }
}
