fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let max_steps = get_max_steps(&input);
    println!("Maximum steps away from 'S': {max_steps}");

    // second task
    let tiles_inside = get_tiles_inside(&input);
    println!("Tiles inside loop: {tiles_inside}");
}

fn get_max_steps(input: &str) -> u64 {
    let mut pipe_map = input.split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| {
            line.chars()
                .map(|c| PipeTile::Unvisited(c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let final_positions = mark_loop(&mut pipe_map);

    // println!("final_positions: {final_positions:?}");
    // println!("pipe_map: ");
    // for line in pipe_map.iter() {
    //     for tile in line {
    //         match tile {
    //             PipeTile::Unvisited(c) => print!(" {c}"),
    //             PipeTile::Vistied(n) => print!(" {n}"),
    //         }
    //     }
    //     println!("");
    // }

    final_positions.iter()
        .map(|pos| pipe_map[pos[0]][pos[1]].clone())
        .map(|tile| match tile {
            PipeTile::Unvisited(_) => 0,
            PipeTile::Vistied(n) => n,
        })
        .max().unwrap()
}

fn mark_loop(pipe_map: &mut Vec<Vec<PipeTile>>) -> Vec<[usize; 2]> {
    let mut curr_positions: Vec<[usize; 2]> = vec!();
    let mut final_positions: Vec<[usize; 2]> = vec!();

    // find start 'S'
    pipe_map.iter().enumerate()
        .for_each(|(i, line)| {
            line.iter().enumerate()
                .filter(|(_, tile)| **tile == PipeTile::Unvisited('S'))
                .for_each(|(j, _)| curr_positions.push([i, j]));
        });

    let mut current_step = 0u64;
    while curr_positions.len() != 0 {

        let mut new_positions: Vec<[usize; 2]> = vec!();
        
        for pos in curr_positions.iter() {
            let mut connected = false;
            let current_tile = pipe_map[pos[0]][pos[1]];

            // check left
            if pos[1] > 0 {
                let curr_connectable = 
                    current_tile == PipeTile::Unvisited('S')
                    || current_tile == PipeTile::Unvisited('J')
                    || current_tile == PipeTile::Unvisited('-')
                    || current_tile == PipeTile::Unvisited('7');
                let new_tile = pipe_map[pos[0]][pos[1] - 1];

                let new_connectable = 
                    new_tile == PipeTile::Unvisited('L')
                    || new_tile == PipeTile::Unvisited('-') 
                    || new_tile == PipeTile::Unvisited('F');

                if curr_connectable && new_connectable {
                    new_positions.push([pos[0], pos[1] - 1]);
                    connected = true;
                }
            }

            // check right
            if pos[1] < pipe_map[pos[0]].len() - 1 {
                let curr_connectable = 
                    current_tile == PipeTile::Unvisited('S')
                    || current_tile == PipeTile::Unvisited('L')
                    || current_tile == PipeTile::Unvisited('-')
                    || current_tile == PipeTile::Unvisited('F');
                let new_tile = pipe_map[pos[0]][pos[1] + 1];

                let new_connectable = 
                    new_tile == PipeTile::Unvisited('J')
                    || new_tile == PipeTile::Unvisited('-') 
                    || new_tile == PipeTile::Unvisited('7');

                if curr_connectable && new_connectable {
                    new_positions.push([pos[0], pos[1] + 1]);
                    connected = true;
                }
            }

            // check up
            if pos[0] > 0 {
                let curr_connectable = 
                    current_tile == PipeTile::Unvisited('S')
                    || current_tile == PipeTile::Unvisited('J')
                    || current_tile == PipeTile::Unvisited('|')
                    || current_tile == PipeTile::Unvisited('L');
                let new_tile = pipe_map[pos[0] - 1][pos[1]];

                let new_connectable = 
                    new_tile == PipeTile::Unvisited('7')
                    || new_tile == PipeTile::Unvisited('|') 
                    || new_tile == PipeTile::Unvisited('F');

                if curr_connectable && new_connectable {
                    new_positions.push([pos[0] - 1, pos[1]]);
                    connected = true;
                }
            }

            // check down
            if pos[0] < pipe_map.len() - 1 {
                let curr_connectable = 
                    current_tile == PipeTile::Unvisited('S')
                    || current_tile == PipeTile::Unvisited('7')
                    || current_tile == PipeTile::Unvisited('|')
                    || current_tile == PipeTile::Unvisited('F');
                let new_tile = pipe_map[pos[0] + 1][pos[1]];

                let new_connectable = 
                    new_tile == PipeTile::Unvisited('J')
                    || new_tile == PipeTile::Unvisited('|') 
                    || new_tile == PipeTile::Unvisited('L');

                if curr_connectable && new_connectable {
                    new_positions.push([pos[0] + 1, pos[1]]);
                    connected = true;
                }
            }

            if !connected {
                final_positions.push(*pos);
            }
        }

        for pos in curr_positions.iter() {
            pipe_map[pos[0]][pos[1]] = PipeTile::Vistied(current_step);
        }
        curr_positions = new_positions;
        current_step += 1;

    }
    final_positions
}

fn get_tiles_inside(input: &str) -> u64 {

    let mut pipe_map = input.split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| {
            line.chars()
                .map(|c| PipeTile::Unvisited(c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    
    // keep only pipes that belong to the loop
    let mut marked_map = pipe_map.clone();
    let _ = mark_loop(&mut marked_map);
    std::iter::zip(pipe_map.iter_mut(), marked_map.iter())
        .for_each(|(pipe_line, marked_line)| {
            std::iter::zip(pipe_line.iter_mut(), marked_line.iter())
                .for_each(|(tile, marked_tile)| {
                    match marked_tile {
                        PipeTile::Unvisited(_) => *tile = PipeTile::Unvisited('.'),
                        _ => { },
                    }
                })
        });
    // 'S' has to be replaced by the correct pipe that completes the loop
    let mut starts: Vec<[usize; 2]> = vec!();
    pipe_map.iter().enumerate()
        .for_each(|(i, line)| {
            line.iter().enumerate()
                .filter(|(_, tile)| **tile == PipeTile::Unvisited('S'))
                .for_each(|(j, _)| starts.push([i, j]));
        });
    for pos in starts {
        // check left
        let mut left = false;
        if pos[1] > 0 {
            let new_tile = pipe_map[pos[0]][pos[1] - 1];

            left = 
                new_tile == PipeTile::Unvisited('L')
                || new_tile == PipeTile::Unvisited('-') 
                || new_tile == PipeTile::Unvisited('F');
        }

        // check right
        let mut right = false;
        if pos[1] < pipe_map[pos[0]].len() - 1 {
            let new_tile = pipe_map[pos[0]][pos[1] + 1];

            right = 
                new_tile == PipeTile::Unvisited('J')
                || new_tile == PipeTile::Unvisited('-') 
                || new_tile == PipeTile::Unvisited('7');
        }

        // check up
        let mut up = false;
        if pos[0] > 0 {
            let new_tile = pipe_map[pos[0] - 1][pos[1]];

            up = 
                new_tile == PipeTile::Unvisited('7')
                || new_tile == PipeTile::Unvisited('|') 
                || new_tile == PipeTile::Unvisited('F');
        }

        // check down
        let mut down = false;
        if pos[0] < pipe_map.len() - 1 {
            let new_tile = pipe_map[pos[0] + 1][pos[1]];

            down = 
                new_tile == PipeTile::Unvisited('J')
                || new_tile == PipeTile::Unvisited('|') 
                || new_tile == PipeTile::Unvisited('L');
        }

        pipe_map[pos[0]][pos[1]] = match (left, right, up, down) {
            (true, true, _, _) => PipeTile::Unvisited('-'),
            (true, _, true, _) => PipeTile::Unvisited('J'),
            (true, _, _, true) => PipeTile::Unvisited('7'),
            (_, true, true, _) => PipeTile::Unvisited('L'),
            (_, true, _, true) => PipeTile::Unvisited('F'),
            (_, _, true, true) => PipeTile::Unvisited('|'),
            _ => PipeTile::Unvisited('S'),
        };
    }

    // imagine the center of a tile to be slightly in the top
    // such that a knife cutting leftwards from a tile cuts through
    // 'J', '|', 'L' but not '7', 'F', '-' 
    //
    // if the knife cuts through an even number of pipes
    // the original tile is outside, else it is inside
    let mut tiles_inside = 0u64;

    for line in pipe_map.iter_mut() {
        let mut loop_pokes = 0u64;

        for tile in line.iter_mut() {
            match tile {
                PipeTile::Unvisited('J') 
                    | PipeTile::Unvisited('|')
                    | PipeTile::Unvisited('L') => loop_pokes += 1,
                PipeTile::Unvisited('.') => {
                    *tile = PipeTile::Vistied(loop_pokes);
                    tiles_inside += loop_pokes % 2;
                },
                _ => { },
            };
        }
    }
    // println!("pipe_map: ");
    // for line in pipe_map.iter() {
    //     for tile in line {
    //         match tile {
    //             PipeTile::Unvisited(c) => print!(" {c}"),
    //             PipeTile::Vistied(n) => print!(" {n}"),
    //         }
    //     }
    //     println!("");
    // }

    tiles_inside
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum PipeTile {
    Unvisited(char),
    Vistied(u64),
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = 
            ".....\n\
             .S-7.\n\
             .|.|.\n\
             .L-J.\n\
             .....\n";
        let max_steps = 4;
        assert_eq!(get_max_steps(&input), max_steps);
    }

    #[test]
    fn test_example2() {
        let input = 
            "..F7.\n\
             .FJ|.\n\
             SJ.L7\n\
             |F--J\n\
             LJ...\n";
        let max_steps = 8;
        assert_eq!(get_max_steps(&input), max_steps);
    }

    #[test]
    fn test_example3() {
        let input = 
            ".F----7F7F7F7F-7....\n\
             .|F--7||||||||FJ....\n\
             .||.FJ||||||||L7....\n\
             FJL7L7LJLJ||LJ.L-7..\n\
             L--J.L7...LJS7F-7L7.\n\
             ....F-J..F7FJ|L7L7L7\n\
             ....L7.F7||L7|.L7L7|\n\
             .....|FJLJ|FJ|F7|.LJ\n\
             ....FJL-7.||.||||...\n\
             ....L---J.LJ.LJLJ...\n";
        let tiles_inside = 8;
        assert_eq!(get_tiles_inside(&input), tiles_inside);
    }

    #[test]
    fn test_example4() {
        let input = 
            "FF7FSF7F7F7F7F7F---7\n\
             L|LJ||||||||||||F--J\n\
             FL-7LJLJ||||||LJL-77\n\
             F--JF--7||LJLJ7F7FJ-\n\
             L---JF-JLJ.||-FJLJJ7\n\
             |F|F-JF---7F7-L7L|7|\n\
             |FFJF7L7F-JF7|JL---7\n\
             7-L-JL7||F7|L7F-7F7|\n\
             L.L7LFJ|||||FJL7||LJ\n\
             L7JLJL-JLJLJL--JLJ.L\n";
        let tiles_inside = 10;
        assert_eq!(get_tiles_inside(&input), tiles_inside);
    }
}
