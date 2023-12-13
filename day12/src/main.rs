fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let arrangements_sum = get_arrangements_sum(&input);
    println!("Sum of all arrangements: {arrangements_sum}");
    
    // second task
    let unfolded_arrangements_sum = get_unfolded_arrangements_sum(&input);
    println!("Sum of all arrangements: {unfolded_arrangements_sum}");
}

fn get_arrangements_sum(input: &str) -> u64 {
    let data = input.split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            let input = line_iter.next().unwrap();
            let pattern = line_iter.next().unwrap()
                .split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (input, pattern)
        })
        .collect::<Vec<(&str, Vec<usize>)>>();
    let values = data.iter()
        .map(|(input, pattern)| get_arrangements(input, &pattern))
        .collect::<Vec<_>>();

    values.iter()
        .sum()

}

fn get_unfolded_arrangements_sum(input: &str) -> u64 {
    let data = input.split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            let input = line_iter.next().unwrap();
            let pattern = line_iter.next().unwrap()
                .split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (input.to_owned(), pattern)
        })
        .collect::<Vec<(String, Vec<usize>)>>();
    let data = data.iter()
        .map(|(input, pattern)| {
            let mut input = input.clone();
            input.push('?');
            let input = input.repeat(5).strip_suffix("?").unwrap().to_owned();
            let pattern = pattern.repeat(5);

            (input, pattern)
        })
        .collect::<Vec<(String, Vec<usize>)>>();
    let values = data.iter()
        .enumerate()
        .map(|(i, (input, pattern))| {
            println!("{} %", i as f32 / data.len() as f32);
            get_arrangements(input, &pattern)
        })
        .collect::<Vec<_>>();

    values.iter()
        .sum()

}

fn get_arrangements(input: &str, pattern: &[usize]) -> u64 {
    // .....#?##..?# => #?##..?#
    let input = input.trim_start_matches('.');
    match pattern.len() {
        0 if input.len() == 0 => return 1,
        0 => return input.find('#').is_none() as u64,
        _ if input.len() == 0 => return 0,
        _ => { },
    }

    // complete started blocks
    // #?##..?# => ..?#
    if input.chars().next().unwrap() == '#' {
        let mut input_iter = input.chars();
        for _ in 0..pattern[0] {
            match input_iter.next() {
                Some('#') | Some('?') => { },
                _ => return 0,
            };
        }
        match input_iter.next() {
            Some('.') | Some('?') => {
                let input = &input[(pattern[0] + 1)..];
                let pattern = &pattern[1..];

                return get_arrangements(input, pattern);
            },
            None => {
                let input = &input[pattern[0]..];
                let pattern = &pattern[1..];

                return get_arrangements(input, pattern);
            },
            _ => {
                return 0;
            },
        };
    }

    let input_new = input.replacen('?', "#", 1);

    return get_arrangements(&input_new, pattern) + get_arrangements(&input[1..], pattern);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = 
            "???.### 1,1,3\n\
             .??..??...?##. 1,1,3\n\
             ?#?#?#?#?#?#?#? 1,3,1,6\n\
             ????.#...#... 4,1,1\n\
             ????.######..#####. 1,6,5\n\
             ?###???????? 3,2,1\n";
        let arrangements_sum = 21;
        assert_eq!(get_arrangements_sum(&input), arrangements_sum);
    }

    #[test]
    fn test_example2() {
        assert_eq!(get_arrangements("???.###", &vec![1, 1, 3]), 1);
    }

    #[test]
    fn test_example3() {
        assert_eq!(get_arrangements("?###????????", &vec![3, 2, 1]), 10);
    }

    #[test]
    fn test_example4() {
        let input = 
            "???.### 1,1,3\n\
             .??..??...?##. 1,1,3\n\
             ?#?#?#?#?#?#?#? 1,3,1,6\n\
             ????.#...#... 4,1,1\n\
             ????.######..#####. 1,6,5\n\
             ?###???????? 3,2,1\n";
        let unfolded_arrangements_sum = 525152;
        assert_eq!(get_unfolded_arrangements_sum(&input), unfolded_arrangements_sum);
    }
}
