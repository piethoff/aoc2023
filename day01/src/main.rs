use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap();
    let input: Vec<&str> = input.split('\n')
        .filter(|s| s.len() != 0)
        .collect();

    // first task
    let calibrations = get_calibrations(&input);
    let sum = sum_calibrations(&calibrations);

    println!("First task: Sum of Calibrations is {sum}");

    // second task
    let calibrations = get_word_calibrations(&input);
    let sum = sum_calibrations(&calibrations);

    println!("Second task: Sum of Calibrations is {sum}");
}

fn sum_calibrations(input: &Vec<u32>) -> u32 {
    input.iter()
        .sum()
}

fn get_calibrations(input: &Vec<&str>) -> Vec<u32> {
    input.iter()
        .map(|s| {
            let first = s.chars()
                .find(|&c| '0' <= c && c <= '9')
                .unwrap()
                .to_digit(10)
                .unwrap();
            let last = s.chars()
                .rev()
                .find(|&c| '0' <= c && c <= '9')
                .unwrap()
                .to_digit(10)
                .unwrap();
            first * 10 + last
        })
        .collect()
}

fn get_word_calibrations(input: &Vec<&str>) -> Vec<u32> {
    input.iter()
        .map(|s| {
            let first_last = find_first_last_number(s);
            (first_last[0] * 10 + first_last[1]) as u32
        })
        .collect()
}

fn find_first_last_number(input: &str) -> [usize; 2] {
    let mut first_occurrance: [Option<usize>; 10] = [None; 10];
    let mut last_occurrance:  [Option<usize>; 10] = [None; 10];
    let num_words = ["zero", "one",
                     "two", "three", 
                     "four", "five", 
                     "six", "seven", 
                     "eight", "nine"];
    for i in 0..=9 {
        let first_word = input.find(num_words[i]);
        let first_digit = input.find(('0' as u8 + i as u8) as char);

        first_occurrance[i] = match (first_word, first_digit) {
            (Some(pos_w), Some(pos_d)) => Some(std::cmp::min(pos_w, pos_d)),
            (None, Some(pos_d)) => Some(pos_d),
            (Some(pos_w), None) => Some(pos_w),
            _ => None,
        };

        let last_word = input.rfind(num_words[i]);
        let last_digit = input.rfind(('0' as u8 + i as u8) as char);

        last_occurrance[i] = match (last_word, last_digit) {
            (Some(pos_w), Some(pos_d)) => Some(std::cmp::max(pos_w, pos_d)),
            (None, Some(pos_d)) => Some(pos_d),
            (Some(pos_w), None) => Some(pos_w),
            _ => None,
        };
    }

    let first_number = first_occurrance.iter()
        .enumerate()
        .filter_map(|(n, pos)| {
            if let Some(pos) = pos {
                return Some((n, pos));
            }
            return None;
        })
        .min_by_key(|(_n, pos)| pos.clone())
        .unwrap().0;

    let last_number = last_occurrance.iter()
        .enumerate()
        .filter_map(|(n, pos)| {
            if let Some(pos) = pos {
                return Some((n, pos));
            }
            return None;
        })
        .max_by_key(|(_n, pos)| pos.clone())
        .unwrap().0;

    [first_number, last_number]
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ];
        let solution = vec![12, 38, 15, 77];

        let result = get_calibrations(&input);
        assert_eq!(solution, result);

        let solution_sum = 142;
        assert_eq!(solution_sum, sum_calibrations(&result));
    }

    #[test]
    fn test_words() {
        let input = vec![
            "two1nine",
            "eightwothree", 
            "abcone2threexyz", 
            "xtwone3four", 
            "4nineeightseven2", 
            "zoneight234", 
            "7pqrstsixteen", 
        ];
        let solution = vec![29, 83, 13, 24, 42, 14, 76];

        let result = get_word_calibrations(&input);
        assert_eq!(solution, result);

        let solution_sum = 281;
        assert_eq!(solution_sum, sum_calibrations(&result));
    }
}
