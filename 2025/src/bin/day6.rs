use std::fs;

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read input file")
}

fn main() {
    // let input = read_input("data/test_day6.txt");
    let input = read_input("data/day6.txt");
    // let mut part1 = 0u64;
    let mut part2 = 0u64;

    let rows: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    // println!("{:?}", rows);

    let mut results = vec![0; rows[0].len()];
    let ops = rows
        .last()
        .cloned()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, &r)| {
            match r {
                "*" => {
                    results[i] = 1;
                }
                "+" => {
                    results[i] = 0;
                }
                _ => {}
            };
            r
        })
        .collect::<Vec<&str>>();
    // println!("{:?}", ops);

    for row in rows.iter().take(rows.len() - 1) {
        for (i, num_str) in row.iter().enumerate() {
            let num = num_str.parse::<u64>().unwrap();
            match ops[i] {
                "*" => {
                    // println!("Multiplying {} to results[{}] = {}", num, i, results[i]);
                    results[i] *= num;
                }
                "+" => {
                    // println!("Adding {} to results[{}] = {}", num, i, results[i]);
                    results[i] += num;
                }
                _ => {}
            }
        }
    }

    // println!("Results: {:?}", results);

    let part1: u64 = results.iter().sum();

    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
    let width = grid[0].len();
    let height = grid.len();
    // println!("{:?}", grid);

    let mut right = width;
    for left in (0..width).rev().filter(|&i| grid[height - 1][i] != b' ') {
        let nums = (left..right).map(|j| {
            (0..height - 1).fold(0, |acc, i| {
                let digit = grid[i][j];
                if digit == b' ' {
                    acc
                } else {
                    acc * 10 + u64::from(digit - b'0')
                }
            })
        });

        if grid[height - 1][left] == b'*' {
            part2 += nums.product::<u64>();
        } else {
            part2 += nums.sum::<u64>();
        }
        if left == 0 {
            break;
        }
        right = left - 1;
    }

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
