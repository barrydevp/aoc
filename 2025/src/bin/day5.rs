use std::fs;

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read input file")
}

type Range = (u64, u64);

fn main() {
    // let input = read_input("data/test_day5.txt");
    let input = read_input("data/day5.txt");
    let mut part1 = 0u64;
    let mut part2 = 0u64;

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let mut ranges: Vec<Range> = parts[0]
        .lines()
        .map(|line| {
            let nums = line.split('-').collect::<Vec<&str>>();
            (
                nums[0].parse::<u64>().unwrap(),
                nums[1].parse::<u64>().unwrap(),
            )
        })
        .collect();
    // ranges.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    ranges.sort(); // exactly the same as above

    let mut ids: Vec<u64> = parts[1]
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    ids.sort();

    // println!("{:?}", ranges);
    // println!("{:?}", ids);

    let mut i = 0;
    let mut j = 0;
    while i < ranges.len() && j < ids.len() {
        if ids[j] > ranges[i].1 {
            i += 1;
        } else if ids[j] < ranges[i].0 {
            j += 1;
        } else {
            // ids[j] is in ranges[i]
            part1 += 1;
            j += 1;
        }
    }

    let mut i = 0;
    while i < ranges.len() {
        let start = ranges[i].0;
        let mut j = i + 1;
        while j < ranges.len() {
            if ranges[j].0 > ranges[i].1 {
                break;
            }
            if ranges[j].1 >= ranges[i].1 {
                i = j;
            }
            j += 1;
            // if ranges[j].1 <= ranges[i].1 {
            //     j += 1;
            // } else if ranges[j].0 <= ranges[i].1 {
            //     i = j;
            //     j += 1;
            // } else {
            //     break;
            // }
        }
        let end = ranges[i].1;
        part2 += end - start + 1;
        // println!("{}, {}", start, end);
        i = j;
    }

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
