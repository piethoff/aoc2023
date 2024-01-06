fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let north_load = get_north_load(&input);
    println!("North load: {north_load}");
}

fn get_cycled_north_load(input: &str) -> u64 {
    let mut input_chars = split_input(input);

    let mut old_input_chars = input_chars.clone();
    slide_cycle(&mut input_chars);

    // detect cycle by hashing after each slide
    // extrapolate cycle to 1_000_000_000
    // hash map of (chars, num_slides)
    for i in 0..1_000_000_000 {
        slide_cycle(&mut input_chars);
        if i % 10_000 == 0 {
            println!("{}:", i);
            input_chars.iter()
                .for_each(|line| {
                    line.iter()
                        .for_each(|c| print!("{c}"));
                    println!("");
                });
        }
    }

    calc_north_load(&input_chars)
}

fn get_north_load(input: &str) -> u64 {
    let mut input_chars = split_input(input);
    slide_north(&mut input_chars);

    calc_north_load(&input_chars)
}

fn calc_north_load(input: &Vec<Vec<char>>) -> u64 {
    input.iter()
        .rev()
        .enumerate()
        .map(|(weight, row)| {
            row.iter()
                .filter(|c| **c == 'O')
                .count() as u64 * (weight + 1) as u64
        })
        .sum()
}

fn slide_cycle(input: &mut Vec<Vec<char>>) {
    slide_north(input);
    slide_west(input);
    slide_south(input);
    slide_east(input);
}

fn slide_north(input: &mut Vec<Vec<char>>) {
    for col in 0..input[0].len() {
        let mut curr_top = 0;
        'rows: for row in 0..input.len() {
            while input[curr_top][col] != '.' {
                curr_top += 1;
                if curr_top >= input.len() {
                    break 'rows;
                }
            }
            if row <= curr_top {
                continue 'rows;
            }
            match input[row][col] {
                '.' => { },
                '#' => curr_top = row,
                'O' => {
                    input[curr_top][col] = 'O';
                    input[row][col] = '.';
                },
                _ => { },
            };
        }
    }
}

fn slide_south(input: &mut Vec<Vec<char>>) {
    for col in 0..input[0].len() {
        let mut curr_top = input[0].len() - 1;
        'rows: for row in (0..input.len()).rev() {
            while input[curr_top][col] != '.' {
                curr_top -= 1;
                if curr_top <= 0 {
                    break 'rows;
                }
            }
            if row >= curr_top {
                continue 'rows;
            }
            match input[row][col] {
                '.' => { },
                '#' => curr_top = row,
                'O' => {
                    input[curr_top][col] = 'O';
                    input[row][col] = '.';
                },
                _ => { },
            };
        }
    }
}

fn slide_east(input: &mut Vec<Vec<char>>) {
    for row in 0..input.len() {
        let mut curr_left = input[row].len() - 1;
        'columns: for col in (0..input[row].len()).rev() {
            while input[row][curr_left] != '.' {
                curr_left -= 1;
                if curr_left <= 0 {
                    break 'columns;
                }
            }
            if col >= curr_left {
                continue 'columns;
            }
            match input[row][col] {
                '.' => { },
                '#' => curr_left = col,
                'O' => {
                    input[row][curr_left] = 'O';
                    input[row][col] = '.';
                },
                _ => { },
            };
        }
    }
}

fn slide_west(input: &mut Vec<Vec<char>>) {
    for row in 0..input.len() {
        let mut curr_left = 0;
        'columns: for col in 0..input[row].len() {
            while input[row][curr_left] != '.' {
                curr_left += 1;
                if curr_left >= input[row].len() {
                    break 'columns;
                }
            }
            if col <= curr_left {
                continue 'columns;
            }
            match input[row][col] {
                '.' => { },
                '#' => curr_left = col,
                'O' => {
                    input[row][curr_left] = 'O';
                    input[row][col] = '.';
                },
                _ => { },
            };
        }
    }
}

fn split_input(input: &str) -> Vec<Vec<char>> {
    input.split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = 
            "O....#....\n\
             O.OO#....#\n\
             .....##...\n\
             OO.#O....O\n\
             .O.....O#.\n\
             O.#..O.#.#\n\
             ..O..#O..O\n\
             .......O..\n\
             #....###..\n\
             #OO..#....\n";
        let north_load = 136;
        assert_eq!(get_north_load(&input), north_load);
    }

    #[test]
    fn test_example2() {
        let input = 
            "O....#....\n\
             O.OO#....#\n\
             .....##...\n\
             OO.#O....O\n\
             .O.....O#.\n\
             O.#..O.#.#\n\
             ..O..#O..O\n\
             .......O..\n\
             #....###..\n\
             #OO..#....\n";

        let output = 
             "OOOO.#.O..\n\
              OO..#....#\n\
              OO..O##..O\n\
              O..#.OO...\n\
              ........#.\n\
              ..#....#.#\n\
              ..O..#.O.O\n\
              ..O.......\n\
              #....###..\n\
              #....#....\n";

        let mut input_chars = split_input(input);
        slide_north(&mut input_chars);

        let output_chars = split_input(output);

        assert_eq!(input_chars, output_chars);
    }

    #[test]
    fn test_example3() {
        let input = 
            "O....#....\n\
             O.OO#....#\n\
             .....##...\n\
             OO.#O....O\n\
             .O.....O#.\n\
             O.#..O.#.#\n\
             ..O..#O..O\n\
             .......O..\n\
             #....###..\n\
             #OO..#....\n";

        let one_cycle = 
             ".....#....\n\
              ....#...O#\n\
              ...OO##...\n\
              .OO#......\n\
              .....OOO#.\n\
              .O#...O#.#\n\
              ....O#....\n\
              ......OOOO\n\
              #...O###..\n\
              #..OO#....\n";

        let two_cycle =
            ".....#....\n\
             ....#...O#\n\
             .....##...\n\
             ..O#......\n\
             .....OOO#.\n\
             .O#...O#.#\n\
             ....O#...O\n\
             .......OOO\n\
             #..OO###..\n\
             #.OOO#...O\n";
             
        let three_cycle =
            ".....#....\n\
             ....#...O#\n\
             .....##...\n\
             ..O#......\n\
             .....OOO#.\n\
             .O#...O#.#\n\
             ....O#...O\n\
             .......OOO\n\
             #...O###.O\n\
             #.OOO#...O\n";

        let mut input_chars = split_input(input);

        slide_cycle(&mut input_chars);
        let one_cycle_chars = split_input(one_cycle);
        assert_eq!(input_chars, one_cycle_chars);

        slide_cycle(&mut input_chars);
        let two_cycle_chars = split_input(two_cycle);
        assert_eq!(input_chars, two_cycle_chars);

        slide_cycle(&mut input_chars);
        let three_cycle_chars = split_input(three_cycle);
        assert_eq!(input_chars, three_cycle_chars);
    }

    #[test]
    fn test_example5() {
        let input = 
            "O....#....\n\
             O.OO#....#\n\
             .....##...\n\
             OO.#O....O\n\
             .O.....O#.\n\
             O.#..O.#.#\n\
             ..O..#O..O\n\
             .......O..\n\
             #....###..\n\
             #OO..#....\n";
        let cycled_north_load = 64;
        assert_eq!(get_cycled_north_load(&input), cycled_north_load);
    }
}
