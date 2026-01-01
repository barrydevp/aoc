use std::{cmp::min, fs};

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read input file")
}

fn main() {
    let input = read_input("data/test_day3.txt");
    // let input = read_input("data/day3.txt");
    let mut part1 = 0u64;
    let mut part2 = 0u64;
    let mut part2_1 = 0u64;

    input.lines().for_each(|bank| {
        let nums: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut max_i = 0;

        let mut i = 1;
        while i < nums.len() - 1 {
            if nums[i] > nums[max_i] {
                max_i = i;
            }
            i += 1;
        }

        let mut max_val = nums[max_i + 1];
        i = max_i + 1;
        while i < nums.len() {
            if nums[i] > max_val {
                max_val = nums[i];
            }
            i += 1;
        }
        part1 += (nums[max_i] * 10 + max_val) as u64;

        let mut max_at = vec![0; nums.len() + 1];

        for i in (0..nums.len()).rev() {
            let end = min(12, nums.len() - i);
            for j in (1..=end).rev() {
                if max_at[j] < 10u64.pow(j as u32 - 1) {
                    max_at[j] = max_at[j - 1] + 10u64.pow(j as u32 - 1) * (nums[i] as u64);
                } else {
                    let cur = max_at[j - 1] + 10u64.pow(j as u32 - 1) * (nums[i] as u64);
                    if cur > max_at[j] {
                        max_at[j] = cur;
                    }
                }

                // println!("num: {}, i: {}, max_at[{}]: {}", bank, i, j, max_at[j]);
            }
        }
        // println!("in: {}, max_at: {:?}", bank, max_at[12]);

        part2 += max_at[12] as u64;

        let mut max_all = 0u64;
        let mut last_i = 0;
        for k in (0..12).rev() {
            let mut max_cur = 11;
            let start_i = last_i;
            for i in start_i..(nums.len() - k) {
                if max_cur == 11 || nums[i] > max_cur {
                    max_cur = nums[i];
                    last_i = i;
                }
            }
            max_all += max_cur as u64 * 10u64.pow(k as u32);
            last_i += 1;
        }
        // println!("in: {}, max_at: {:?}", bank, max_all);
        part2_1 += max_all;
    });

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
    println!("Part2_1: {}", part2_1);
}
