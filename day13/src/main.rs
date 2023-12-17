use core::panic;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let reflections_sum = get_reflections_sum(&input);
    println!("Reflections sum: {reflections_sum}");

    // second task
    let unsmuged_reflections_sum = get_unsmudged_reflections_sum(&input);
    println!("Unsmudged reflections sum: {unsmuged_reflections_sum}");
}

fn get_unsmudged_reflections_sum(input: &str) -> u64 {
    let blocks = input.split("\n\n")
        .filter(|block| block.len() != 0)
        .collect::<Vec<_>>();

    blocks.iter()
        .map(|block| {
            let column_smudge = find_columns_smudge(block);
            let row_smudge = find_row_smudge(block);
            match (column_smudge, row_smudge) {
                (Some((col, _)), None) => col as u64,
                (None, Some((row, _))) => (row * 100) as u64,
                _ => panic!("no or multiple smudges found"),
            }
        })
        .sum::<u64>()
}

fn get_reflections_sum(input: &str) -> u64 {
    input.split("\n\n")
        .map(|block| {
            let num = get_columns_reflections(block) 
                + 100 * get_row_reflections(block);
            num
        })
    .sum()
}

fn get_columns_reflections(input: &str) -> u64 {
    let data = input.split('\n')
        .filter(|line| line.len() != 0)
        .collect::<Vec<_>>();
    (1..(data[0].len())).into_iter()
        .filter(|mirror_col| {
            data.iter()
                .all(|row| {
                    std::iter::zip(
                        row.chars().skip(*mirror_col), 
                        row.chars().rev().skip(row.len() - mirror_col)
                        )
                        .all(|(a, b)| a == b)
                })
        })
        .sum::<usize>() as u64
}

fn get_row_reflections(input: &str) -> u64 {
    let data = input.split('\n')
        .filter(|line| line.len() != 0)
        .collect::<Vec<_>>();
    (1..(data.len())).into_iter()
        .filter(|mirror_row| {
            std::iter::zip(
                data.iter().skip(*mirror_row), 
                data.iter().rev().skip(data.len() - mirror_row)
                )
                .all(|(a, b)| a == b)
        })
        .sum::<usize>() as u64
}

fn find_columns_smudge(input: &str) -> Option<(usize, (usize, usize))> {
    let data = input.split('\n')
        .filter(|line| line.len() != 0)
        .collect::<Vec<_>>();
    let smudges = (1..(data[0].len())).into_iter()
        .map(|mirror_col| {
            data.iter().enumerate()
                .map(|(row_i, row)| {
                    let diff_positions = std::iter::zip(
                        row.chars().enumerate().skip(mirror_col), 
                        row.chars().rev().skip(row.len() - mirror_col)
                        )
                        .filter(|((_, a), b)| a != b)
                        .map(move |((col_i, _), _)| (mirror_col, (row_i, col_i)));
                    diff_positions
                })
            .flatten()
            .collect::<Vec<(usize, (usize, usize))>>()
        })
        .filter(|diff_positions| diff_positions.len() == 1)
        .map(|diff_positions| diff_positions[0])
        .collect::<Vec<(usize, (usize, usize))>>();

    if smudges.len() == 1 {
        return Some(smudges[0]);
    } else {
        return None;
    }
}

fn find_row_smudge(input: &str) -> Option<(usize, (usize, usize))> {
    let data = input.split('\n')
        .filter(|line| line.len() != 0)
        .collect::<Vec<_>>();
    let smudges = (1..(data.len())).into_iter()
        .map(|mirror_row| {
            std::iter::zip(
                data.iter().enumerate().skip(mirror_row), 
                data.iter().rev().skip(data.len() - mirror_row)
                )
                .map(|((row_i, a), b)| {
                    let diff_positions = std::iter::zip(a.chars(), b.chars()).enumerate()
                        .filter(|(_, (a, b))| a != b)
                        .map(move |(col_i, _)| (mirror_row, (row_i, col_i)));
                    diff_positions
                })
            .flatten()
            .collect::<Vec<(usize, (usize, usize))>>()
        })
        .filter(|diff_positions| diff_positions.len() == 1)
        .map(|diff_positions| diff_positions[0])
        .collect::<Vec<(usize, (usize, usize))>>();

    if smudges.len() == 1 {
        return Some(smudges[0]);
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = 
            "#.##..##.\n\
             ..#.##.#.\n\
             ##......#\n\
             ##......#\n\
             ..#.##.#.\n\
             ..##..##.\n\
             #.#.##.#.\n";
        let reflections_sum = 5;
        assert_eq!(get_reflections_sum(&input), reflections_sum);
    }

    #[test]
    fn test_example2() {
        let input = 
            "#...##..#\n\
             #....#..#\n\
             ..##..###\n\
             #####.##.\n\
             #####.##.\n\
             ..##..###\n\
             #....#..#\n";
        let reflections_sum = 400;
        assert_eq!(get_reflections_sum(&input), reflections_sum);
    }

    #[test]
    fn test_example3() {
        let input = 
            "#.##..##.\n\
             ..#.##.#.\n\
             ##......#\n\
             ##......#\n\
             ..#.##.#.\n\
             ..##..##.\n\
             #.#.##.#.\n\
             \n\
             #...##..#\n\
             #....#..#\n\
             ..##..###\n\
             #####.##.\n\
             #####.##.\n\
             ..##..###\n\
             #....#..#\n";
        let reflections_sum = 405;
        assert_eq!(get_reflections_sum(&input), reflections_sum);
    }

    #[test]
    fn test_example4() {
        let input = 
            "#....#\n\
             #.####\n\
             ..##..\n\
             ######\n\
             ######\n\
             ..##..\n\
             #....#\n";
        let smudge_pos = (1, 4);
        assert_eq!(find_columns_smudge(&input), Some(smudge_pos));
    }

    #[test]
    fn test_example5() {
        let input = 
            "#....#\n\
             ..##..\n\
             ######\n\
             ##.###\n\
             ..##..\n\
             #....#\n";
        let smudge_pos = (3, 2);
        assert_eq!(find_row_smudge(&input), Some(smudge_pos));
    }
}
