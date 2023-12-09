fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let steps = get_steps(&input);
    println!("Needed steps: {steps}");

    // second task
    let steps = get_steps_parallel(&input);
    println!("Needed parallel steps: {steps}");
}

fn parse_input(input: &str) -> (Vec<Node>, Vec<Direction>) {
    let mut lines_iter = input.split('\n');
    let directions = lines_iter.next().unwrap()
        .chars()
        .map(|c| c.try_into())
        .collect::<Result<Vec<Direction>, _>>().unwrap();
    
    let mut nodes = lines_iter.filter(|line| line.len() != 0)
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            let id = line_iter.next().unwrap();
            let _ = line_iter.next();
            let left = line_iter.next().unwrap()
                .trim_matches(|c| c == '(' || c == ')' || c == ',');
            let right = line_iter.next().unwrap()
                .trim_matches(|c| c == '(' || c == ')' || c == ',');
            let node = Node { id: id_from_str(id).unwrap(), 
                              left: NodeRef::ID(id_from_str(left).unwrap()),
                              right: NodeRef::ID(id_from_str(right).unwrap()) };
            // Rc::new(RefCell::new(node))
            node
        })
        .collect::<Vec<_>>();
    nodes.sort_unstable_by(|a, b| cmp_id(&a.id, &b.id));

    (nodes, directions)
}

fn get_steps(input: &str) -> u64 {

    let (mut nodes, directions) = parse_input(&input);
    let mut directions_iter = directions.iter().cycle();

    let mut step = 0u64;
    let mut curr_pos = 0usize;

    while curr_pos != nodes.len() - 1 {
        let direction = directions_iter.next().unwrap();
        let next_node = match direction {
            Direction::Left => { &nodes[curr_pos].left },
            Direction::Right => { &nodes[curr_pos].right },
        };
        curr_pos = match next_node {
            NodeRef::Pos(p) => *p,
            NodeRef::ID(id) => {
                // compute pos and update nodes
                let next_pos = nodes.binary_search_by(|node| cmp_id(&node.id, &id)).unwrap();

                match direction {
                    Direction::Left => { nodes[curr_pos].left = NodeRef::Pos(next_pos); },
                    Direction::Right => { nodes[curr_pos].right = NodeRef::Pos(next_pos); },
                }
                next_pos
            },
        };
        step += 1;
    }
    step
}

fn get_steps_parallel(input: &str) -> u64 {

    let (mut nodes, directions) = parse_input(&input);
    let mut directions_iter = directions.iter().enumerate().cycle();

    let mut step = 0u64;
    let curr_posses = nodes.iter().enumerate()
        .filter(|(_, node)| node.id[2] == 'A')
        .map(|(i, _)| i as usize)
        .collect::<Vec<_>>();

    // each staring node probably exerts some loop eventually
    //
    // for each start node note step at which they 
    // reach a finish node, also note direction state?
    // if a finish node with the same direction state 
    // is reached the cycle length and position is known
    //
    // compute "least common multiple" with offset?
    //
    // step through biggest cycle and check if all other cycles align

    let mut all_cycle_events: Vec<Vec<CycleEvent>> = vec![vec!(); curr_posses.len()];


    for (curr_pos, cycle_events) in std::iter::zip(curr_posses.iter(), all_cycle_events.iter_mut()) {
        let mut curr_pos = *curr_pos;
        'stepping: loop {
            let direction = directions_iter.next().unwrap();
            let next_node = match direction.1 {
                Direction::Left =>  { &nodes[curr_pos].left },
                Direction::Right => { &nodes[curr_pos].right },
            };
            curr_pos = match next_node {
                NodeRef::Pos(p) => *p,
                NodeRef::ID(id) => {
                    // compute pos and update nodes
                    let next_pos = nodes.binary_search_by(|node| cmp_id(&node.id, &id)).unwrap();

                    match direction.1 {
                        Direction::Left =>  { nodes[curr_pos].left = NodeRef::Pos(next_pos); },
                        Direction::Right => { nodes[curr_pos].right = NodeRef::Pos(next_pos); },
                    }
                    next_pos
                },
            };
            step += 1;

            // check for finish node
            if nodes[curr_pos].id[2] == 'Z' {
                let new_event = CycleEvent { pos: curr_pos, direction_state: direction.0, step };
                cycle_events.push(new_event);
                // check if current state is already in cycle_events
                if cycle_events.contains(&new_event) {
                    break 'stepping;
                } 
            }
        }
    }

    // from cycle_events generate list of target offsets and cycle length
    let mut target_lists = all_cycle_events.iter()
        .map(|cycle_events| {
            let cycle_start = cycle_events.iter()
                .filter(|event| event.pos == cycle_events[cycle_events.len() - 1].pos)
                .next().unwrap().step;
            let cycle_end = cycle_events[cycle_events.len() - 1].step;
            let mut result = Targets { cycle_len: cycle_end - cycle_start, offsets: vec!() };
            cycle_events.iter()
                .for_each(|event| result.offsets.push(event.step));
            let _ = result.offsets.pop();
            result
        })
        .collect::<Vec<_>>();
    target_lists.sort_unstable_by(|a, b| ((b.cycle_len as f64 / b.offsets.len() as f64).ceil() as u64).cmp(&((a.cycle_len as f64 / a.offsets.len() as f64).ceil() as u64)));

    // compute lcm with offset one after another
    // e.g. ... lcm(lcm(lcm(value1, value2), value3), value4), ...

    todo!()
    // 'stepping: loop {
    //     let finished = curr_posses.iter()
    //         .all(|pos| nodes[*pos].id[2] == 'Z');
    //     if finished {
    //         break 'stepping;
    //     }
    //     if step % 1000 == 0 {
    //         println!("{step}: {curr_posses:?}");
    //     }

    //     let direction = directions_iter.next().unwrap();
    //     curr_posses = curr_posses.iter()
    //         .map(|curr_pos| {
    //             let next_node = match direction {
    //                 Direction::Left => { &nodes[*curr_pos].left },
    //                 Direction::Right => { &nodes[*curr_pos].right },
    //             };
    //             match next_node {
    //                 NodeRef::Pos(p) => *p,
    //                 NodeRef::ID(id) => {
    //                     // compute pos and update nodes
    //                     let next_pos = nodes.binary_search_by(|node| cmp_id(&node.id, &id)).unwrap();

    //                     match direction {
    //                         Direction::Left => { nodes[*curr_pos].left = NodeRef::Pos(next_pos); },
    //                         Direction::Right => { nodes[*curr_pos].right = NodeRef::Pos(next_pos); },
    //                     }
    //                     next_pos
    //                 },
    //             }
    //         })
    //         .collect();
    //     step += 1;
    // }
    // step
}

struct Targets {
    offsets: Vec<u64>,
    cycle_len: u64,
}

#[derive(Clone, Copy)]
struct CycleEvent {
    pos: usize,
    direction_state: usize,
    step: u64,
}
impl PartialEq for CycleEvent {
    fn eq(&self, other: &Self) -> bool {
        if self.pos.eq(&other.pos) {
            return self.direction_state.eq(&other.direction_state);
        }
        return false;
    }
}
impl Eq for CycleEvent { }

#[derive(Debug)]
struct Node {
    id: [char; 3],
    // left:  [char; 3],
    // right: [char; 3],
    left:  NodeRef,
    right: NodeRef,
}

#[derive(Debug)]
enum NodeRef {
    ID([char; 3]),
    Pos(usize),
}

fn id_from_str(s: &str) -> Result<[char; 3], &'static str> {
    if s.len() != 3 {
        return Err("invalid string for node");
    }
    let mut id = ['0'; 3];
    let mut s_iter = s.chars();
    id[0] = s_iter.next().unwrap();
    id[1] = s_iter.next().unwrap();
    id[2] = s_iter.next().unwrap();

    Ok( id )
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
fn cmp_id(first: &[char; 3], second: &[char; 3]) -> std::cmp::Ordering {
    for i in (0..3).rev() {
        let ordering = first[i].cmp(&second[i]);
        match ordering {
            std::cmp::Ordering::Equal => { },
            _ => { return ordering; },
        }
    }
    return std::cmp::Ordering::Equal;
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(cmp_id(&self.id, &other.id))
    }
}
impl Eq for Node { }
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
impl TryFrom<char> for Direction {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("invalid char for direction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = 
            "RL\n\
             \n\
             AAA = (BBB, CCC)\n\
             ABC = (BBB, CCC)\n\
             BBB = (DDD, EEE)\n\
             CCC = (ZZZ, GGG)\n\
             DDD = (DDD, DDD)\n\
             EEE = (EEE, EEE)\n\
             GGG = (GGG, GGG)\n\
             ZZZ = (ZZZ, ZZZ)\n";
        let steps = 2;
        assert_eq!(get_steps(&input), steps);
    }

    #[test]
    fn test_example2() {
        let input = 
            "LLR\n\
             \n\
             AAA = (BBB, BBB)\n\
             ABC = (BBB, CCC)\n\
             BBB = (AAA, ZZZ)\n\
             ZZZ = (ZZZ, ZZZ)\n";
        let steps = 6;
        assert_eq!(get_steps(&input), steps);
    }

    #[test]
    fn test_example3() {
        let input = 
            "LR\n\
             \n\
             11A = (11B, XXX)\n\
             11B = (XXX, 11Z)\n\
             11Z = (11B, XXX)\n\
             22A = (22B, XXX)\n\
             22B = (22C, 22C)\n\
             22C = (22Z, 22Z)\n\
             22Z = (22B, 22B)\n\
             XXX = (XXX, XXX)\n";
        let steps = 6;
        assert_eq!(get_steps_parallel(&input), steps);
    }
}
