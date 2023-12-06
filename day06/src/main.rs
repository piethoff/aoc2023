fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let possibilities = get_num_possibilities(&input);
    println!("Number of possibilities is {possibilities}");

    // second task
    let possibilities = get_squashed_possibilities(&input);
    println!("Number of squashed possibilities is {possibilities}");
}

fn get_num_possibilities(input: &str) -> u64 {
    let mut input_iter = input.split('\n');
    let times = input_iter.next().unwrap()
        .split(':')
        .skip(1)
        .next().unwrap()
        .split_whitespace()
        .map(|time| time.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let distances = input_iter.next().unwrap()
        .split(':')
        .skip(1)
        .next().unwrap()
        .split_whitespace()
        .map(|time| time.parse::<u64>().unwrap() + 1u64)
        .collect::<Vec<_>>();

    let possibilities = std::iter::zip(times.iter(), distances.iter())
        .map(|(t, record)| {
            (*t as f64 / 2f64 - ((*t as f64 / 2f64).powi(2) - (*record) as f64).sqrt(),
             *t as f64 / 2f64 + ((*t as f64 / 2f64).powi(2) - (*record) as f64).sqrt())
        })
        .map(|(lower, upper)| (upper.floor() as i64 - lower.ceil() as i64 + 1i64) as u64)
        .product();

    return possibilities;

    // distance = v * t_drive
    // t_drive = t - t_button
    // v = a * t_button
    // a = 1 mm/ms /s
    //
    // => distance(t_button) = (a * t_button) * (t - t_button)
    // => distance(t_button) = a * ( - t_button^2 + t*t_button )
    // distance(t_button) = record
    // => a * ( - t_button^2 + t*t_button ) = record
    // => t_button^2 - t*t_button + record/a = 0
    // t_button = t/2 +- sqrt(t^2/4 - record/a)
    //
    // t = 7, record = 9
    // => t_button = 3.5 +- sqrt(12.25 - 9)
    //             = 3.5 +- 1.8
    //    t_button = 1.7 or t_button = 5.3
    //    t_button in 2..=5 => 4 possibilities
}

fn get_squashed_possibilities(input: &str) -> u64 {
    let mut input_iter = input.split('\n');
    let time = input_iter.next().unwrap()
        .split(':')
        .skip(1)
        .next().unwrap()
        .replace(" ", "")
        .parse::<u64>().unwrap();

    let distance = input_iter.next().unwrap()
        .split(':')
        .skip(1)
        .next().unwrap()
        .replace(" ", "")
        .parse::<u64>().unwrap();
    println!("time: {time}, distance: {distance}");

    let possibilities =
            (time as f64 / 2f64 - ((time as f64 / 2f64).powi(2) - (distance) as f64).sqrt(),
             time as f64 / 2f64 + ((time as f64 / 2f64).powi(2) - (distance) as f64).sqrt());
    let possibilities = (possibilities.1.floor() as i64 - possibilities.0.ceil() as i64 + 1i64) as u64;

    return possibilities;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = 
            "Time:      7  15   30\n\
             Distance:  9  40  200";
        let num_possibilities = 288;
        assert_eq!(get_num_possibilities(&input), num_possibilities);
    }

    #[test]
    fn test_example2() {
        let input = 
            "Time:      7  15   30\n\
             Distance:  9  40  200";
        let num_possibilities = 71503;
        assert_eq!(get_squashed_possibilities(&input), num_possibilities);
    }
}
