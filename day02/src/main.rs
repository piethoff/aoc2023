fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();
    let requirement = Cubes {red: 12, green: 13, blue: 14};
    let sum = sum_possible_games(&input, &requirement);

    println!("First task: Sum of IDs: {sum}");

    let power = games_power_sum(&input);
    println!("Second Task: Sum of Power of all Games: {power}");
}

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

fn sum_possible_games(games: &str, requirement: &Cubes) -> u32 {
    games.split('\n')
        .filter(|game| game.len() != 0)
        .map(|game| {
            let mut game_iter = game.split(':');
            let game_id = game_iter.next()
                .unwrap()[5..]
                .parse::<u32>()
                .unwrap();
            let game_nums = game_iter.next().unwrap();
            (game_id, game_min_total(game_nums))
        })
        .map(|(id, min_cubes)| id * is_game_possible(&min_cubes, requirement) as u32)
        .sum()
}

fn games_power_sum(games: &str) -> u32 {
    games.split('\n')
        .filter(|game| game.len() != 0)
        .map(|game| {
            let mut game_iter = game.split(':');
            let _ = game_iter.next();
            game_iter.next().unwrap()
        })
        .map(|game| game_min_total(game))
        .map(|min_cubes| min_cubes.red * min_cubes.blue * min_cubes.green)
        .sum()
}

fn game_min_total(game: &str) -> Cubes {
    // game: "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    let mut total_cubes = Cubes {red: 0, green: 0, blue: 0};
    // draw: " 1 red, 2 green, 6 blue"
    for draw in game.split(';') {
        // color: "1 red"
        for color in draw.split(',') {
            let mut color_iter = color.split_whitespace();
            let num = color_iter.next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let color = color_iter.next().unwrap();
            match color {
                "red" if total_cubes.red < num => total_cubes.red = num,
                "blue" if total_cubes.blue < num => total_cubes.blue = num,
                "green" if total_cubes.green < num => total_cubes.green = num,
                _ => {},
            }
        }
    }
    total_cubes
}

fn is_game_possible(game_total: &Cubes, requirement: &Cubes) -> bool {
    game_total.red <= requirement.red &&
    game_total.blue <= requirement.blue &&
    game_total.green <= requirement.green
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let requirement = Cubes {red: 12, green: 13, blue: 14};
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let sum = sum_possible_games(input, &requirement);
        assert_eq!(sum, 8);
    }
}
