use std::fs;

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read input file")
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

type Point = (usize, usize);

fn main() {
    // let input = read_input("data/test_day4.txt");
    let input = read_input("data/day4.txt");
    let mut part1 = 0u64;
    let mut part2 = 0u64;

    let grids: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut grids_mod = grids.clone();
    let mut to_remove: Vec<Point> = vec![];
    let mut grids_track: Vec<Vec<i32>> = vec![vec![0; grids[0].len()]; grids.len()];

    // println!("{:?}", grids);

    for row in 0..grids.len() {
        for col in 0..grids[0].len() {
            if grids[row][col] == '.' {
                grids_track[row][col] = -1;
                continue;
            }

            let roll_around = DIRECTIONS
                .iter()
                .map(|(dr, dc)| {
                    let new_row = row as isize + dr;
                    let new_col = col as isize + dc;
                    if new_row < 0
                        || new_row >= grids.len() as isize
                        || new_col < 0
                        || new_col >= grids[0].len() as isize
                    {
                        return false;
                    }
                    grids[new_row as usize][new_col as usize] == '@'
                })
                .filter(|&x| x)
                .count();

            if roll_around < 4 {
                part1 += 1;
                grids_mod[row][col] = 'x';
                to_remove.push((row, col));
            }
            grids_track[row][col] = roll_around as i32;
        }
    }

    while let Some((row, col)) = to_remove.pop() {
        part2 += 1;
        grids_track[row][col] = -1;
        DIRECTIONS.iter().for_each(|(dr, dc)| {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            if new_row < 0
                || new_row >= grids.len() as isize
                || new_col < 0
                || new_col >= grids[0].len() as isize
            {
                return;
            }
            let (nr, nc) = (new_row as usize, new_col as usize);
            if grids_track[nr][nc] == -1 {
                return;
            }
            if grids_track[nr][nc] == 4 {
                grids_mod[nr][nc] = 'x';
                to_remove.push((nr, nc));
            }
            grids_track[nr][nc] -= 1;
        });
    }

    // grids_mod.iter().for_each(|row| {
    //     row.iter().for_each(|&c| print!("{}", c));
    //     println!();
    // });

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
