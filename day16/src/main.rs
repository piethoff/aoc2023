fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let num_energized = get_energized(&input);
    println!("Number of energized tiles: {num_energized}");
    
    // second task
    let max_num_energized = get_max_energized(&input);
    println!("Maximal number of energized tiles: {max_num_energized}");
}

fn get_max_energized(input: &str) -> u64 {
    let mut energized_tiles: Vec<u64> = vec!();

    let tiles = parse_input(input);

    for i in 0..tiles.len() {
        let beam = Beam { position: (i as isize, 0), direction: Direction::East };
        energized_tiles.push(compute_beam(tiles.clone(), beam));

        let beam = Beam { position: (i as isize, (tiles[0].len() - 1) as isize), direction: Direction::West };
        energized_tiles.push(compute_beam(tiles.clone(), beam));
    }

    for j in 0..tiles[0].len() {
        let beam = Beam { position: (0, j as isize), direction: Direction::South };
        energized_tiles.push(compute_beam(tiles.clone(), beam));

        let beam = Beam { position: ((tiles.len() - 1) as isize, j as isize), direction: Direction::North };
        energized_tiles.push(compute_beam(tiles.clone(), beam));
    }

    *energized_tiles.iter()
        .max()
        .unwrap()
}

fn get_energized(input: &str) -> u64 {
    let tiles = parse_input(input);
    compute_beam(tiles, Beam { position: (0, 0), direction: Direction::East })
}

fn compute_beam(mut tiles: Vec<Vec<Tile>>, beam: Beam) -> u64 {
    let mut beams: Vec<Beam> = vec![beam];

    while beams.len() != 0 {
        advance_beams(&mut beams, &mut tiles);
    }

    tiles.iter()
        .map(|row| row.iter())
        .flatten()
        .filter(|tile| tile.energized)
        .count() as u64
}

fn advance_beams(beams: &mut Vec<Beam>, tiles: &mut Vec<Vec<Tile>>) {

    // energize tiles below beams
    for beam in beams.iter() {
        tiles[beam.position.0 as usize]
             [beam.position.1 as usize].energized = true;
    }

    // mirror/split beams
    let mut beams_to_append: Vec<Beam> = vec!();

    for beam in beams.iter_mut() {
        match tiles[beam.position.0 as usize]
                   [beam.position.1 as usize].mirror {
            Some(Mirror::LeftMirror) => {
                match beam.direction {
                    Direction::North => {
                        beam.direction = Direction::West;
                    },
                    Direction::East => {
                        beam.direction = Direction::South;
                    },
                    Direction::South => {
                        beam.direction = Direction::East;
                    },
                    Direction::West => {
                        beam.direction = Direction::North;
                    },
                }
            },
            Some(Mirror::RightMirror) => {
                match beam.direction {
                    Direction::North => {
                        beam.direction = Direction::East;
                    },
                    Direction::East => {
                        beam.direction = Direction::North;
                    },
                    Direction::South => {
                        beam.direction = Direction::West;
                    },
                    Direction::West => {
                        beam.direction = Direction::South;
                    },
                }
            },
            Some(Mirror::VerticalSplitter) => {
                match beam.direction {
                    Direction::East | Direction::West => {
                        let mut new_beam = beam.clone();
                        new_beam.direction = Direction::South;
                        beam.direction = Direction::North;
                        beams_to_append.push(beam.clone());
                        beams_to_append.push(new_beam);

                        tiles[beam.position.0 as usize]
                             [beam.position.1 as usize].mirror = 
                             Some(Mirror::UsedVerticalSplitter);
                    },
                    Direction::North | Direction::South => { },
                }
            },
            Some(Mirror::HorizontalSplitter) => {
                match beam.direction {
                    Direction::North | Direction::South => {
                        let mut new_beam = beam.clone();
                        new_beam.direction = Direction::West;
                        beam.direction = Direction::East;
                        beams_to_append.push(beam.clone());
                        beams_to_append.push(new_beam);

                        tiles[beam.position.0 as usize]
                             [beam.position.1 as usize].mirror = 
                             Some(Mirror::UsedHorizontalSplitter);
                    },
                    Direction::East | Direction::West => { },
                }
            },
            Some(Mirror::UsedVerticalSplitter) => { },
            Some(Mirror::UsedHorizontalSplitter) => { },
            None => { },
        }
    }

    // drop all beams on top of used splitters in splitting direction
    beams.retain(|beam| {
        let current_tile = &tiles[beam.position.0 as usize]
                                 [beam.position.1 as usize];
        match &current_tile.mirror {
            Some(Mirror::UsedVerticalSplitter)   | 
            Some(Mirror::UsedHorizontalSplitter) => {
                return false;
            }
            _ => { },
        }
        return true;
    });

    // append new beams
    beams.append(&mut beams_to_append);


    // transport beams
    for beam in beams.iter_mut() {
        match beam.direction {
            Direction::North => {
                beam.position.0 -= 1;
            },
            Direction::East => {
                beam.position.1 += 1;
            },
            Direction::South => {
                beam.position.0 += 1;
            },
            Direction::West => {
                beam.position.1 -= 1;
            },
        }
    }

    // drop all beams outside bounds
    beams.retain(|beam| {
        if beam.position.0 as usize >= tiles[0].len() {
            return false;
        }
        if beam.position.1 as usize >= tiles.len() {
            return false;
        }
        return true;
    });

}

#[derive(Clone, Debug)]
struct Beam {
    position: (isize, isize),
    direction: Direction,
}

#[derive(Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
struct Tile {
    mirror: Option<Mirror>,
    energized: bool,
}

#[derive(Clone)]
enum Mirror {
    LeftMirror,
    RightMirror,
    VerticalSplitter,
    HorizontalSplitter,
    // make splitters one time use to avoid cycles
    UsedVerticalSplitter,
    UsedHorizontalSplitter,
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input.split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| line.chars()
             .map(|c| {
                 let mirror = match c {
                     '\\' => Some(Mirror::LeftMirror),
                     '/' => Some(Mirror::RightMirror),
                     '|' => Some(Mirror::VerticalSplitter),
                     '-' => Some(Mirror::HorizontalSplitter),
                     '.' | _ => None
                 };
                 Tile { mirror, energized: false }
             })
             .collect::<Vec<_>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = 
            ".|...\\....\n\
             |.-.\\.....\n\
             .....|-...\n\
             ........|.\n\
             ..........\n\
             .........\\\n\
             ..../.\\\\..\n\
             .-.-/..|..\n\
             .|....-|.\\\n\
             ..//.|....\n";
        let num_energized = get_energized(&input);
        assert_eq!(num_energized, 46);
    }

    #[test]
    fn test_example2() {
        let input = 
            ".|...\\....\n\
             |.-.\\.....\n\
             .....|-...\n\
             ........|.\n\
             ..........\n\
             .........\\\n\
             ..../.\\\\..\n\
             .-.-/..|..\n\
             .|....-|.\\\n\
             ..//.|....\n";
        let num_energized = get_max_energized(&input);
        assert_eq!(num_energized, 51);
    }
}
