use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let sum_distances = get_sum_distances(&input, 2u32);
    println!("Sum of expanded distances: {sum_distances}");

    // second task
    let sum_distances = get_sum_distances(&input, 1_000_000u32);
    println!("Sum of expaaaaanded distances: {sum_distances}");
}

fn get_sum_distances(input: &str, expansion_factor: u32) -> i64{
    let parsed_input = input.split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();

    let (columns, rows) = expand(&parsed_input);

    let positions = parsed_input.iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(j, _)| [i.clone(), j])
                .collect::<Vec<[usize; 2]>>()
        })
    .flatten()
    .map(|[row_pos, col_pos]| {
        let col_expansions = columns.iter()
            .filter(|col| **col < col_pos)
            .count();
        let row_expansions = rows.iter()
            .filter(|row| **row < row_pos)
            .count();
        [row_pos as i64 + row_expansions as i64 * (expansion_factor - 1) as i64,
         col_pos as i64 + col_expansions as i64 * (expansion_factor - 1) as i64]
    })
    .collect::<Vec<[i64; 2]>>();

    calc_sum_distances(positions)
}

fn calc_sum_distances(positions: Vec<[i64; 2]>) -> i64 {
    positions.iter()
        .combinations(2)
        .map(|positions| {
            (positions[1][0] - positions[0][0]).abs() 
                + (positions[1][1] - positions[0][1]).abs()
        })
        .sum()
}

fn expand(input: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let mut columns: Vec<usize> = vec!();
    // expand columns
    for i in 0..input[0].len() {
        let insert_column = input.iter()
            .map(|line| {
                line.chars()
                    .skip(i)
                    .next()
                    .unwrap()
            })
        .all(|c| c == '.');
        if insert_column {
            columns.push(i);
        }
    }

    let mut rows: Vec<usize> = vec!();
    // expand rows
    for i in 0..input.len() {
        let insert_row = input[i].chars().all(|c| c == '.');
        if insert_row {
            rows.push(i);
        }
    }

    (columns, rows)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = 
            "...#......\n\
             .......#..\n\
             #.........\n\
             ..........\n\
             ......#...\n\
             .#........\n\
             .........#\n\
             ..........\n\
             .......#..\n\
             #...#.....\n";
        let sum_distances = 374;
        assert_eq!(get_sum_distances(&input, 2u32), sum_distances);
    }
}
