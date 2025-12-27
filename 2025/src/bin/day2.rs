use std::fs;

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read input file")
}

fn sum_range(start: u64, nstart: u32, end: u64, n: u32, k: u32) -> u64 {
    let max_num = 10u64.pow(n) - 1;
    let max_part = 10u64.pow(k) - 1;
    let step = max_num / max_part;

    let lower = if nstart == n {
        start.next_multiple_of(step).max(start)
    } else {
        step * 10u64.pow(k - 1)
    };

    let upper = end.min(max_num);

    if lower <= upper {
        let n = (upper - lower) / step + 1;
        return step * (lower / step * 2 + n - 1) * n / 2;
    }

    0
}

fn main() {
    // let input = read_input("data/test_day2.txt");
    let input = read_input("data/day2.txt");
    let mut part1 = 0u64;
    let mut part2 = 0u64;

    input.trim().split(',').for_each(|range| {
        let pairs = range.split('-').collect::<Vec<&str>>();
        let nstart = pairs[0].len();
        let nend = pairs[1].len();
        let start: u64 = pairs[0].parse().unwrap();
        let end: u64 = pairs[1].parse().unwrap();

        // Part 1
        for n in nstart..=nend {
            if n % 2 != 0 {
                continue;
            }

            part1 += sum_range(start, nstart as u32, end, n as u32, (n / 2) as u32);
        }

        // Part 2, double counting when the pattern of k digits is a factor of higher h digits
        // e.g., k = 2, h = 4, pattern = (12)(12)(12)(12) -> (1212)(1212)
        // also, obviously all patterns contains patterns of k=1
        for n in nstart..=nend {
            let mut counted = vec![];
            'outer: for k in (1..=n / 2).rev() {
                if n % k != 0 {
                    continue;
                }

                for h in counted.iter() {
                    if h % k == 0 {
                        continue 'outer;
                    }
                }

                let sum = sum_range(start, nstart as u32, end, n as u32, k as u32);
                if sum > 0 {
                    part2 += sum;
                    counted.push(k);
                }
            }
            if counted.len() > 1 {
                part2 -= sum_range(start, nstart as u32, end, n as u32, 1);
            }
        }

        // println!("Start: {}, End: {}", start, end);
    });
    // for ranges in ranges {
    //     let pairs = ranges.split('-').collect::<Vec<&str>>();
    //     let start: i32 = pairs[0].parse().unwrap();
    //     let end: i32 = pairs[1].parse().unwrap();
    //     println!("Start: {}, End: {}", start, end);
    // }

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
