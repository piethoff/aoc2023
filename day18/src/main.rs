fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let volume = get_volume(&input);
    println!("Volume is {volume}");
    
    // second task
    let volume = get_volume_hex(&input);
    println!("Volume from hex is {volume}");
}

fn get_circumference(points: &Vec<Point>) -> i64 {
    points.windows(2)
        .map(|points| {
            (points[1].position.0 - points[0].position.0).abs() 
          + (points[1].position.1 - points[0].position.1).abs() 
        })
    .sum()
}

fn parse_points_hex(input: &str) -> Vec<Point> {
    let mut current_position = (0, 0);
    let mut points = input.trim()
        .split('\n')
        .map(|line| {
            let mut line_iter = line.split(' ').skip(2);

            let hex_str = line_iter.next().unwrap();
            let distance = i64::from_str_radix(&hex_str[2..7], 16).unwrap();
            let direction = match &hex_str[7..8] {
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                _ => "x",
            };

            let offset = match direction {
                "R" => (distance, 0),
                "L" => (-distance, 0),
                "U" => (0, -distance),
                "D" => (0, distance),
                _ => (0, 0),
            };

            current_position.0 += offset.0;
            current_position.1 += offset.1;

            Point { position: current_position.clone() }
        })
    .collect::<Vec<_>>();
    points.push(points[0].clone());
    points
}

fn parse_points(input: &str) -> Vec<Point> {
    let mut current_position = (0, 0);
    let mut points = input.trim()
        .split('\n')
        .map(|line| {
            let mut line_iter = line.split(' ');
            let direction = line_iter.next().unwrap();
            let distance = line_iter.next()
                .unwrap()
                .parse::<i64>()
                .unwrap();

            let offset = match direction {
                "R" => (distance, 0),
                "L" => (-distance, 0),
                "U" => (0, -distance),
                "D" => (0, distance),
                _ => (0, 0),
            };

            current_position.0 += offset.0;
            current_position.1 += offset.1;

            Point { position: current_position.clone() }
        })
    .collect::<Vec<_>>();
    points.push(points[0].clone());
    points
}

fn compute_volume(points: &Vec<Point>) -> i64 {
    points.windows(2)
        .map(|points| {
            // cross product
            points[0].position.0 * points[1].position.1 
          - points[0].position.1 * points[1].position.0
        })
    .sum::<i64>() / 2
}

fn get_volume_hex(input: &str) -> i64 {

    let points = parse_points_hex(input);

    let circumference = get_circumference(&points);
    let volume = compute_volume(&points);

    volume + circumference / 2 + 1
}

fn get_volume(input: &str) -> i64 {

    let points = parse_points(input);

    let circumference = get_circumference(&points);
    let volume = compute_volume(&points);

    volume + circumference / 2 + 1
}

#[derive(Clone, Debug)]
struct Point {
    position: (i64, i64),
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = 
            "R 6 (#70c710)\n\
             D 5 (#0dc571)\n\
             L 2 (#5713f0)\n\
             D 2 (#d2c081)\n\
             R 2 (#59c680)\n\
             D 2 (#411b91)\n\
             L 5 (#8ceee2)\n\
             U 2 (#caa173)\n\
             L 1 (#1b58a2)\n\
             U 2 (#caa171)\n\
             R 2 (#7807d2)\n\
             U 3 (#a77fa3)\n\
             L 2 (#015232)\n\
             U 2 (#7a21e3)\n";
        let volume = get_volume(&input);
        assert_eq!(volume, 62);
    }

    #[test]
    fn test_example2() {
        let input = 
            "R 6 (#70c710)\n\
             D 2 (#0dc571)\n\
             L 6 (#5713f0)\n\
             U 2 (#d2c081)\n";
        let volume = get_volume(&input);
        assert_eq!(volume, 7 * 3);
    }

    #[test]
    fn test_example3() {
        // #########
        // #########
        // #########
        // #########
        // #########
        // #########
        //     #####
        //     #####
        //     #####
        //     #####
        let input = 
            "R 8 (#70c710)\n\
             D 8 (#0dc571)\n\
             L 4 (#5713f0)\n\
             U 4 (#d2c081)\n\
             L 4 (#5713f0)\n\
             U 4 (#d2c081)\n";
        let volume = get_volume(&input);
        assert_eq!(volume, 9*9 - 4*4);
    }

    #[test]
    fn test_example4() {
        let input = 
            "R 6 (#70c710)\n\
             D 5 (#0dc571)\n\
             L 2 (#5713f0)\n\
             D 2 (#d2c081)\n\
             R 2 (#59c680)\n\
             D 2 (#411b91)\n\
             L 5 (#8ceee2)\n\
             U 2 (#caa173)\n\
             L 1 (#1b58a2)\n\
             U 2 (#caa171)\n\
             R 2 (#7807d2)\n\
             U 3 (#a77fa3)\n\
             L 2 (#015232)\n\
             U 2 (#7a21e3)\n";
        let volume = get_volume_hex(&input);
        assert_eq!(volume, 952408144115);
    }
}
