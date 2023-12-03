use std::{str::Split, ops::Range, collections::HashSet, fs::read_to_string, path::Path};

fn main() {
    let input = read_to_string(Path::new(
        "/Users/darioalessandro/Documents/advent-of-code-2023/day-3/src/input.txt",
    ))
    .unwrap();
    let actual_part_numbers =find_part_numbers(&input);

    // query them 798
    let assertions = vec!((353, 5), (280, 2), (75, 3), (869, 2), (798, 1), (145, 1), (629, 1), (579, 2), (837, 2), (2, 1), (658, 2), (869, 2));
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
    use super::*;

    #[test]
    fn test_number_parser() {
        let input :Vec<char> = "467..114..".chars().collect();
        assert_eq!(467, find_part_in_row(&input, 0));
        for i in 5..7 {
            assert_eq!(114, find_part_in_row(&input, i));
        }
    }

    #[test]
    fn test_find_numbers() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let mut expected_part_numbers: Vec<u32> = vec!(467, 35, 633, 617, 592, 755, 664, 598);
        let mut actual_part_numbers: Vec<u32> = find_part_numbers(&input).iter().map(|a| {
            a.1
        }).collect();
        actual_part_numbers.sort();
        expected_part_numbers.sort();
        assert_eq!(expected_part_numbers, actual_part_numbers);
        
        // add then together
        let expected_sum: u32 = 4361;
        let actual_sum: u32 = actual_part_numbers.iter().sum();

        assert_eq!(expected_sum, actual_sum);

    }

}
