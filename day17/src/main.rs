fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    
    // second task
}

// counter example to plain dijkstra (e.g. not valuing past directions in node):
// o 1 1 1 1 
// 1 1 1 9 x
//
// optimal cost: 6
// o 1 ^ > >
// V > > 9 V
//
// plain dijkstra cost: 12
// o 1 2 3  .
// 1 2 3 12 x
//
// Node (3, 0) is marked as distance of 3 but is also a dead end as the path cannot 
// continue a fourth time right. The only option to reach the goal x is now via 
// the node of cost 9. The correct detour would mark Node (3, 0) as distance 6, 
// which is higher but has the advantage of continuing farther to the right and 
// avoid the cost 9 node.

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| line.chars()
             .map(|c| {
                 format!("{c}").parse::<u8>().unwrap()
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
            "2413432311323\n\
             3215453535623\n\
             3255245654254\n\
             3446585845452\n\
             4546657867536\n\
             1438598798454\n\
             4457876987766\n\
             3637877979653\n\
             4654967986887\n\
             4564679986453\n\
             1224686865563\n\
             2546548887735\n\
             4322674655533\n";
        let heat_loss = get_min_heat_loss(&input);
        assert_eq!(heat_loss, 102);
    }
}
