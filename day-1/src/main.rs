use std::{collections::HashSet, fs::read_to_string, path::Path};

fn main() {
    // 1. read the input file
    let input_file = read_to_string(Path::new(
        "/Users/darioalessandro/Documents/advent-of-code-2023/day-1/src/input.txt",
    ))
    .unwrap();

    // 2. parse the input file into the decoded calibration vector
    let decoded_vector = decode_calibration_vector(&input_file);

    // 3. sum all values
    let sum: u32 = decoded_vector.iter().sum();

    // 4. print 54985
    println!("sum {}", sum);
}

pub fn str_to_u32(input: &str) -> Option<u32> {
    match input {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

pub fn find_all_numbers_in_string(input: &str) -> Vec<(usize, u32)> {
    let all_numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut left_numbers: Vec<_> = all_numbers
        .iter()
        .filter_map(|number| {
            input.find(number).map(|index| {
                let found_number = str_to_u32(number).unwrap();
                (index, found_number)
            })
        })
        .collect();

    let mut right_numbers: Vec<_> = all_numbers
        .iter()
        .filter_map(|number| {
            input.rfind(number).map(|index| {
                let found_number = str_to_u32(number).unwrap();
                (index, found_number)
            })
        })
        .collect();
    // combine all found numbers and sort by index
    left_numbers.append(&mut right_numbers);
    // remove dups
    let result: HashSet<(usize, u32)> = left_numbers.into_iter().collect();
    result.into_iter().collect::<Vec<_>>()
}

fn find_all_digits_in_string(input: &str) -> Vec<(usize, u32)> {
    input
        .chars()
        .enumerate()
        .filter_map(|(index, char)| char.to_digit(10).map(|digit| (index, digit)))
        .collect()
}

pub fn decode_calibration_vector(input: &str) -> Vec<u32> {
    // split string using \n
    let input = input.split('\n');
    input
        .filter_map(|row| {
            let mut all_digits = find_all_digits_in_string(&row);
            let mut all_string_digits = find_all_numbers_in_string(&row);
            // combine both arrays
            all_digits.append(&mut all_string_digits);
            // sort them by index
            all_digits.sort_by(|a, b| a.0.cmp(&b.0));

            let first_digit = all_digits.first();
            let last_digit = all_digits.last();

            match (first_digit, last_digit) {
                (Some(first_digit), Some(last_digit)) => Some(first_digit.1 * 10 + last_digit.1),
                _ => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_sample_calibration() {
        let calibration = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let exepected_vec: Vec<u32> = vec![12, 38, 15, 77];

        let actual_vec = decode_calibration_vector(calibration);
        // Assert that vectors are equal
        assert_eq!(actual_vec, exepected_vec);

        // add all the values together
        let expected_sum: u32 = exepected_vec.iter().sum();
        let actual_sum: u32 = actual_vec.iter().sum();
        assert_eq!(expected_sum, actual_sum);
    }

    #[test]
    fn test_digits_as_strings() {
        let input = r"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let expected_calibration_vector = vec![29, 83, 13, 24, 42, 14, 76];

        // Adding these together produces 281.)
        let expected_sum: u32 = 281;

        let actual_calibration_vector = decode_calibration_vector(&input);

        assert_eq!(expected_calibration_vector, actual_calibration_vector);

        // checksum
        assert_eq!(expected_sum, actual_calibration_vector.iter().sum());
    }

    #[test]
    fn test_basic_find_usage() {
        let input: Vec<&str> = r"two1nine
eightwothree"
            .split('\n')
            .collect();

        let expected_output = vec![vec![(0, 2), (4, 9)], vec![(0, 8), (4, 2), (7, 3)]];

        println!("rows {:?}", &input);
        let actual_output: Vec<_> = input
            .iter()
            .map(|row| find_all_numbers_in_string(&row))
            .collect();

        assert_eq!(expected_output, actual_output);
    }
    #[test]
    fn test_corrupted_row() {
        let input = r"sdfsdf";
        let output = decode_calibration_vector(&input);
        assert_eq!(Vec::<u32>::new(), output);
    }
}
