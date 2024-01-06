
fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let hash_result = hash_sum(&input);
    println!("Hash of input is: {hash_result}");
    
    // second task
    let focussing_power = read_to_hashmap(&input).focussing_power();
    println!("Focussing power is: {focussing_power}");
}

fn read_to_hashmap(input: &str) -> HashMap {
    let mut hash_map = HashMap::new(256);

    input.trim()
        .split(',')
        .for_each(|lens_str| {
            if let Some(p) = lens_str.find('=') {
                let lens = Lens { label: lens_str[..p].to_owned(), 
                                  focal_len: lens_str[p+1..].parse().unwrap() };
                hash_map.insert_lens(lens);
            } else {
                let lens = Lens { label: lens_str[..lens_str.len()-1].to_owned(), 
                                  focal_len: 0 };
                hash_map.remove_lens(lens);
            }
        });
    hash_map
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_len: u8,
}

#[derive(Debug)]
struct HashMap {
    data: Vec<Vec<Lens>>,
}
impl HashMap {
    fn new(len: usize) -> Self {
        HashMap { data: vec![vec![]; len] }
    }

    fn insert_lens(&mut self, lens: Lens) {
        let box_num = hash(&lens.label) as usize;
        let position_in_box = self.data[box_num].iter()
            .position(|l| l.label == lens.label);
        match position_in_box {
            Some(p) => self.data[box_num][p] = lens,
            None => self.data[box_num].push(lens),
        };
    }

    fn remove_lens(&mut self, lens: Lens) {
        let box_num = hash(&lens.label) as usize;
        let position_in_box = self.data[box_num].iter()
            .position(|l| l.label == lens.label);
        match position_in_box {
            Some(p) => _ = self.data[box_num].remove(p), 
            None => { },
        };
    }

    fn focussing_power(&self) -> u64 {
        self.data.iter()
            .enumerate()
            .map(|(i, box_data)| {
                box_data.iter()
                    .enumerate()
                    .map(|(j, lens)| {
                        (i as u64 + 1) * (j as u64 + 1) * lens.focal_len as u64
                    })
                .sum::<u64>()
            })
        .sum()
    }
}

fn hash_sum(input: &str) -> u64 {
    input.trim()
        .split(',')
        .map(|segment| hash(segment) as u64)
        .sum()
}

fn hash(input: &str) -> u8 {
    // result += char
    // result *= 17
    // result % 256

    let mut result: u8 = 0;
    input.chars()
        .for_each(|c| {
            result = result.wrapping_add(c as u8);
            result = result.wrapping_mul(17);
        });
    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = "HASH";
        let hash_result = 52;
        assert_eq!(hash(&input), hash_result);
    }

    #[test]
    fn test_example2() {
        let input = "rn";
        let hash_result = 0;
        assert_eq!(hash(&input), hash_result);
    }

    #[test]
    fn test_example3() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let hash_map = read_to_hashmap(&input);

        let focussing_power = hash_map.focussing_power();
        assert_eq!(focussing_power, 145);
    }
}
