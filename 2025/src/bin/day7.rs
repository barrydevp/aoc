use std::fs;

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read input file")
}

fn main() {
    // let input = read_input("data/test_day7.txt");
    let input = read_input("data/day7.txt");
    let mut part1 = 0u64;
    // let mut part2 = 0u64;

    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
    let width = grid[0].len();
    let height = grid.len();
    // println!("{:?}", grid);
    let mut track: Vec<bool> = grid[0].iter().map(|&b| b == b'S').collect();
    let mut track2: [Vec<u64>; 2] = [
        track.iter().map(|&b| if b { 1 } else { 0 }).collect(),
        vec![0; width],
    ];

    let mut idx = 0;
    for i in 1..height {
        // println!("{:?}", track2);
        let mut atleast_one = false;
        for j in 0..width {
            if grid[i][j] == b'^' && track[j] {
                atleast_one = true;
                part1 += 1;
                track[j] = false;
                if j > 0 {
                    track[j - 1] = true;
                    track2[(idx + 1) % 2][j - 1] += track2[idx][j];
                    // println!(
                    //     "track2[{}][{}] += {} => {}",
                    //     idx + 1,
                    //     j - 1,
                    //     track2[idx][j],
                    //     track2[(idx + 1) % 2][j - 1]
                    // );
                }
                if j + 1 < width {
                    track[j + 1] = true;
                    track2[(idx + 1) % 2][j + 1] += track2[idx][j];
                }
            }
        }
        if atleast_one {
            for j in 0..width {
                track2[(idx + 1) % 2][j] += if grid[i][j] != b'^' {
                    track2[idx][j]
                } else {
                    0
                };
            }
            track2[idx].fill(0);
            idx = (idx + 1) % 2;
            // println!("{:?}", track2[idx]);
        }
    }

    // println!("{:?}", track2);
    let part2: u64 = track2[idx].iter().sum();

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
