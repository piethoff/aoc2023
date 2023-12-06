fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let part_numbers = get_part_numbers(&input);
    let sum = sum_part_numbers(&part_numbers);

    println!("Sum of part numbers: {sum}");

    // second task
    let gear_ratios = get_gear_ratios(&input);
    let sum = sum_part_numbers(&gear_ratios);

    println!("Sum of gear ratios: {sum}");
}

fn get_part_numbers(input: &str) -> Vec<u32> {
    let lines = input.split('\n')
        .filter(|game| game.len() != 0)
        .collect::<Vec<&str>>();
    let mut digit_positions: Vec<Vec<usize>> = vec![vec!(); lines.len()];
    for line_i in 0..lines.len() {
        // get positions of symbols
        let mut sym_positions: Vec<usize> = vec!();
        let mut pos = 0;
        while let Some(sym_pos) = 
            lines[line_i][pos..].find(|c| c != '.' && !char::is_digit(c, 10) && !char::is_alphabetic(c)) {
                sym_positions.push(sym_pos + pos);
                pos += sym_pos + 1;
            }
        find_digit_positions(&mut digit_positions, &lines, &line_i, &sym_positions);
    }

    find_nums(&lines, &digit_positions)
}

fn get_gear_ratios(input: &str) -> Vec<u32> {
    let lines = input.split('\n')
        .filter(|game| game.len() != 0)
        .collect::<Vec<&str>>();
    let mut nums: Vec<u32> = vec!();
    let mut digit_positions: Vec<Vec<usize>> = vec![vec!(); lines.len()];
    for line_i in 0..lines.len() {
        // get positions of symbols
        let mut sym_positions: Vec<usize> = vec!();
        let mut pos = 0;
        while let Some(sym_pos) = 
            lines[line_i][pos..].find(|c| c == '*') {
                // process each '*' individually
                sym_positions.push(sym_pos + pos);
                find_digit_positions(&mut digit_positions, &lines, &line_i, &sym_positions);
                let nums_to_check = find_nums(&lines, &digit_positions);
                if nums_to_check.len() == 2 {
                    nums.push(nums_to_check[0] * nums_to_check[1]);
                }

                // reset
                digit_positions = vec![vec!(); lines.len()];
                sym_positions = vec!();

                pos += sym_pos + 1;
            }
    }
    nums
}

fn find_digit_positions(digit_positions: &mut Vec<Vec<usize>>, 
                      lines: &Vec<&str>, 
                      line_i: &usize, 
                      sym_positions: &Vec<usize>) {
    for sym_pos in sym_positions {
        for line_off in -1..=1 as i32 {
            for col_off in -1..=1 as i32 {
                if line_off == 0 && col_off == 0 {
                    continue;
                }
                let line_pos = *line_i as i32 + line_off;
                if line_pos < 0 || line_pos >= lines.len() as i32 {
                    continue;
                }
                let col_pos = *sym_pos as i32 + col_off;
                if col_pos < 0 || col_pos >= lines[0].len() as i32 {
                    continue;
                }

                // todo?
                let char_to_check = lines[line_pos as usize].chars().nth(col_pos as usize).unwrap();
                if char::is_digit(char_to_check, 10) {
                    digit_positions[line_pos as usize].push(col_pos as usize);
                }
            }
        }
    }
    for digit_position in digit_positions {
        digit_position.sort();
    }
}

fn find_nums(lines: &Vec<&str>, digit_positions: &Vec<Vec<usize>>) -> Vec<u32> {
    let mut nums: Vec<u32> = vec!();
    for (line, positions) in std::iter::zip(lines.iter(), digit_positions.iter()) {

        let mut pos_iter = positions.iter().peekable();
        while let Some(pos) = pos_iter.next() {
            let mut start: usize;
            let mut stop: usize;

            // find start of number
            start = *pos;
            'find_start: while let Some(prev_char) = line.chars().nth(start - 1) {
                if char::is_digit(prev_char, 10) {
                    start -= 1;
                    if start == 0 {
                        break 'find_start;
                    }
                } else {
                    break 'find_start;
                }
            }

            // find end of number
            stop = *pos;
            'find_stop: while let Some(next_char) = line.chars().nth(stop + 1) {
                if char::is_digit(next_char, 10) {
                    stop += 1;
                } else {
                    break 'find_stop;
                }
            }

            nums.push(line[start..=stop].parse::<u32>().unwrap());

            // forward pos_iter to end of found number
            'forward: while let Some(next_pos) = pos_iter.peek() {
                if **next_pos <= stop{
                    let _ = pos_iter.next();
                } else {
                    break 'forward;
                }
            }
        }
    }
    nums
}

fn sum_part_numbers(numbers: &Vec<u32>) -> u32 {
    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = "467..114..\n\
                     ...*......\n\
                     ..35..633.\n\
                     ......#...\n\
                     617*......\n\
                     .....+.58.\n\
                     ..592.....\n\
                     ......755.\n\
                     ...$.*....\n\
                     .664.598..";
        let sum = 4361;
        let part_numbers = get_part_numbers(input);
        assert_eq!(sum, sum_part_numbers(&part_numbers));
    }

    #[test]
    fn test_example2() {
        let input = "467..114..\n\
                     ...*......\n\
                     ..35..633.\n\
                     ......#...\n\
                     617*......\n\
                     .....+.58.\n\
                     ..592.....\n\
                     ......755.\n\
                     ...$.*....\n\
                     .664.598..";
        let sum = 467835;
        let gear_ratios = get_gear_ratios(&input);
        assert_eq!(sum, sum_part_numbers(&gear_ratios));
    }
}
