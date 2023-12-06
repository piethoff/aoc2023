fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let locations = get_locations(&input);
    let min_location = locations.iter().min().unwrap();
    println!("Minimum location is {min_location}");

    // second task
    let locations = get_locations_from_ranges(&input);
    let min_location = get_min_from_ranges(&locations);
    println!("Minimum location is {min_location}");
}

fn get_locations(input: &str) -> Vec<i64> {
    let maps = get_maps(input);
    let seeds = get_seeds(input);

    apply_maps(maps, seeds)
}

fn get_maps(input: &str) -> Vec<Map> {
    input.split("\n\n")
        .skip(1) // skip seeds
        .map(|raw_map| {
            raw_map.split('\n')
                .skip(1) // skip map header
                .collect::<Vec<_>>()
        })
        .map(|lines| build_map(lines))
        .collect()
}

fn build_map(lines: Vec<&str>) -> Map {
    let mut parsed_lines = lines.iter()
        .filter(|line| line.len() != 0)
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            let mut row = Row::default();
            row.dest = line_iter.next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            row.src = line_iter.next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            row.len = line_iter.next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            row
        })
        .collect::<Vec<_>>();

    parsed_lines.sort_unstable_by_key(|line| line.src);

    let mut map = Map::default();
    let mut current_src = 0i64;
    for i in 0..parsed_lines.len() {
        // check for "default" mapping
        if parsed_lines[i].src > current_src {
            map.ranges.push(OffsetRange {
                start: current_src,
                offset: 0,
            });
        }
        map.ranges.push(OffsetRange {
            start: parsed_lines[i].src,
            offset: parsed_lines[i].dest as i64 - parsed_lines[i].src as i64,
        });
        current_src = parsed_lines[i].src + parsed_lines[i].len;
    }
    // trailing "default" mapping
    map.ranges.push(OffsetRange {
        start: current_src,
        offset: 0,
    });
    map
}

fn get_seeds(input: &str) -> Vec<i64> {
    let mut result = input.split('\n')
        .next() // get first line only
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    result.sort_unstable();
    result
}

fn apply_maps(maps: Vec<Map>, mut seeds: Vec<i64>) -> Vec<i64> {
    for i in 0..maps.len() {
        seeds = apply_map(&maps[i], seeds);
    }
    seeds
}

fn apply_map(map: &Map, input: Vec<i64>) -> Vec<i64> {
    let mut map_pos = 0i64;
    let mut result = input.into_iter()
        .map(|input| {
            'map_search: loop {
                if map_pos + 1 >= map.ranges.len() as i64 {
                    break 'map_search;
                }
                if input >= map.ranges[map_pos as usize + 1].start {
                    map_pos += 1;
                } else {
                    break 'map_search;
                }
            }
            input as i64 + map.ranges[map_pos as usize].offset
        })
        .collect::<Vec<_>>();
    result.sort_unstable();
    result
}

fn get_min_from_ranges(ranges: &Vec<Range>) -> i64 {
    ranges.iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

fn get_locations_from_ranges(input: &str) -> Vec<Range> {
    let maps = get_maps(input);
    let seed_ranges = get_seed_ranges(input);

    apply_maps_to_ranges(maps, seed_ranges)
}

fn get_seed_ranges(input: &str) -> Vec<Range> {
    input.split('\n')
        .next() // get first line only
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|range| Range { start: range[0], stop: range[0] + range[1] - 1 } )
        .collect()
}

fn apply_maps_to_ranges(maps: Vec<Map>, seeds: Vec<Range>) -> Vec<Range> {
    let mut result = seeds.clone();
    for i in 0..maps.len() {
        result = apply_map_to_ranges(&maps[i], &result);
    }
    result
}

fn apply_map_to_ranges(map: &Map, ranges: &Vec<Range>) -> Vec<Range> {
    let mut result: Vec<Range> = vec!();
    for range in ranges {
        result.append(&mut apply_map_to_range(map, &range));
    }
    result
}

fn apply_map_to_range(map: &Map, range: &Range) -> Vec<Range> {
    let mut result: Vec<Range> = vec!();
    let mut range_left = range.clone();

    'map_search: for map_pos in 0..map.ranges.len() {
        if map_pos + 1 >= map.ranges.len() {
            result.push(Range { 
                start: range_left.start + map.ranges[map_pos].offset, 
                stop:  range_left.stop  + map.ranges[map_pos].offset });
            break 'map_search;
        }
        if range_left.start < map.ranges[map_pos + 1].start {
            if range_left.stop < map.ranges[map_pos + 1].start {
                result.push(Range { 
                    start: range_left.start + map.ranges[map_pos].offset,  
                    stop:  range_left.stop  + map.ranges[map_pos].offset });
                break 'map_search;
            } else {
                result.push(Range { 
                    start: range_left.start + map.ranges[map_pos].offset,   
                    stop: map.ranges[map_pos + 1].start - 1 + map.ranges[map_pos].offset });
                range_left.start = map.ranges[map_pos + 1].start;
            }
        }
    }
    result
}

#[derive(Default)]
struct Row {
    dest: i64,
    src: i64,
    len: i64,
}

#[derive(Default)]
struct Map {
    ranges: Vec<OffsetRange>,
}

struct OffsetRange {
    start: i64,
    offset: i64,
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: i64,
    stop: i64,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = 
            "seeds: 79 14 55 13\n\
             \n\
             seed-to-soil map:\n\
             50 98 2\n\
             52 50 48\n\
             \n\
             soil-to-fertilizer map:\n\
             0 15 37\n\
             37 52 2\n\
             39 0 15\n\
             \n\
             fertilizer-to-water map:\n\
             49 53 8\n\
             0 11 42\n\
             42 0 7\n\
             57 7 4\n\
             \n\
             water-to-light map:\n\
             88 18 7\n\
             18 25 70\n\
             \n\
             light-to-temperature map:\n\
             45 77 23\n\
             81 45 19\n\
             68 64 13\n\
             \n\
             temperature-to-humidity map:\n\
             0 69 1\n\
             1 0 69\n\
             \n\
             humidity-to-location map:\n\
             60 56 37\n\
             56 93 4";
        let min_location = 35;
        let locations = get_locations(&input);
        assert_eq!(*locations.iter().min().unwrap(), min_location);
    }

    #[test]
    fn test_example2() {
        let input = 
            "seeds: 79 14 55 13\n\
             \n\
             seed-to-soil map:\n\
             50 98 2\n\
             52 50 48\n\
             \n\
             soil-to-fertilizer map:\n\
             0 15 37\n\
             37 52 2\n\
             39 0 15\n\
             \n\
             fertilizer-to-water map:\n\
             49 53 8\n\
             0 11 42\n\
             42 0 7\n\
             57 7 4\n\
             \n\
             water-to-light map:\n\
             88 18 7\n\
             18 25 70\n\
             \n\
             light-to-temperature map:\n\
             45 77 23\n\
             81 45 19\n\
             68 64 13\n\
             \n\
             temperature-to-humidity map:\n\
             0 69 1\n\
             1 0 69\n\
             \n\
             humidity-to-location map:\n\
             60 56 37\n\
             56 93 4";
        let min_location = 46;
        let locations = get_locations_from_ranges(&input);
        assert_eq!(get_min_from_ranges(&locations), min_location);
    }
}
