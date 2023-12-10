fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let sum = get_sum_extrapolated(&input);
    println!("Sum of extrapolated data points: {sum}");
    
    // second task
    let sum = get_sum_prepolated(&input);
    println!("Sum of prepolated data points: {sum}");
}

fn get_sum_extrapolated(input: &str) -> i64 {
    input.split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        // .map(|data| extrapolate(&data))
        .map(|mut data| extrapolate_inplace(data.as_mut_slice()))
        .sum()
}

fn get_sum_prepolated(input: &str) -> i64 {
    input.split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        // .map(|data| extrapolate(&data))
        .map(|mut data| {
            data.reverse();
            extrapolate_inplace(data.as_mut_slice())
        })
        .sum()
}

#[allow(dead_code)]
fn extrapolate(data: &Vec<i64>) -> i64 {
    let all_zero = data.iter()
        .all(|value| *value == 0);
    if all_zero {
        return 0;
    }

    let next_row = data.windows(2)
        .map(|ab| ab[1] - ab[0])
        .collect::<Vec<i64>>();
    return data.last().unwrap() + extrapolate(&next_row);
}

fn extrapolate_inplace(data: &mut [i64]) -> i64 {
    let all_zero = data.iter()
        .all(|value| *value == 0);
    if all_zero {
        return 0;
    }

    let datalen = data.len() - 1;
    for i in 0..datalen {
        data[i] = data[i + 1] - data[i];
    }

    let last_value = data.last().unwrap().clone();
    return last_value + extrapolate_inplace(&mut data[..datalen]);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = 
            "0 3 6 9 12 15\n\
             1 3 6 10 15 21\n\
             10 13 16 21 30 45\n";
        let sum_extrapolated = 114;
        assert_eq!(get_sum_extrapolated(&input), sum_extrapolated);
    }

    #[test]
    fn test_example2() {
        let input = 
            "0 3 6 9 12 15\n\
             1 3 6 10 15 21\n\
             10 13 16 21 30 45\n";
        let sum_prepolated = 2;
        assert_eq!(get_sum_prepolated(&input), sum_prepolated);
    }
}
