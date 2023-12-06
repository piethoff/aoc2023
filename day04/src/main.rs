fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let sum = point_sum(&input); 
    println!("Sum of points: {sum}");

    // second task
    let sum = copied_num_cards(&input);
    println!("Sum of copied points: {sum}");
}

fn split_input(input: &str) -> Vec<&str> {
    input.split('\n')
        .filter(|game| game.len() != 0)
        .collect::<Vec<&str>>()
}

fn num_winning(line: &str) -> u32 {
    let mut sections = line.split(':')
        .nth(1)
        .unwrap()
        .split('|');
    let winning_nums = sections.next().unwrap();
    let my_nums = sections.next().unwrap();

    let mut my_nums_table = [0u32; 100];

    my_nums.split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .for_each(|num| {
            my_nums_table[num] += 1;
        });

    let mut num_winning = 0u32;
    winning_nums.split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .for_each(|num| {
            if my_nums_table[num] > 0 {
                num_winning += 1;
            }
        });

    num_winning
}

fn points_from_num_winning(num_winning: u32) -> u32 {
    if num_winning > 0 {
        return 1 << (num_winning - 1);
    }
    return 0;
}

fn point_sum(input: &str) -> u32 {
    let lines = split_input(input);
    lines.iter()
        .map(|line| num_winning(line))
        .map(|num| points_from_num_winning(num))
        .sum()
}

fn copied_num_cards(input: &str) -> u32 {
    let lines = split_input(input);
    let nums_winning = lines.iter()
        .map(|line| num_winning(line))
        .collect::<Vec<_>>();
    // how many copies of each card
    let mut weights = vec![1u32; nums_winning.len()];
    nums_winning.iter()
        .enumerate()
        .for_each(|(card_i, num)| {
            for weights_i in card_i+1..card_i+1+*num as usize {
                if weights_i < weights.len() {
                    weights[weights_i] += weights[card_i];
                }
            }
        });
    weights.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let sum = 13;
        assert_eq!(point_sum(&input), sum); 
    }

    #[test]
    fn test_example2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let sum = 30;
        assert_eq!(copied_num_cards(&input), sum);
    }
}
