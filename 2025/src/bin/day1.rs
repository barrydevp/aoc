use std::fs;

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read input file")
}

fn main() {
    // let input = read_input("data/test_day1.txt");
    let input = read_input("data/day1.txt");
    let dials = input.lines();

    let mut pwd1 = 0i32;
    let mut pwd2 = 0i32;
    let mut pos = 50i32;

    dials.for_each(|dial| {
        let dial_ch: Vec<char> = dial.chars().collect();
        let direction = dial_ch[0];
        let steps = dial_ch[1..]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        if direction == 'R' {
            pwd2 += (pos + steps) / 100;
            pos += steps;
        } else if direction == 'L' {
            // we rotate left, the normal circle logic goes backwards, which mean 0 1 2 3 becomes 0 99 98 97
            // so that our pos is now in reverse => we take new pos = (100 - pos)
            // also need to mod by 100 because we could be at pos = 0
            pwd2 += ((100 - pos) % 100 + steps) / 100;
            pos -= steps;
        }

        // println!("Dialing {}  {}  {}", direction, steps, pwd);

        pos = pos.rem_euclid(100);

        if pos == 0 {
            pwd1 += 1;
        }

        // println!("Dialing {}  {}  {}", direction, steps, pwd);
    });

    println!("First pwd is: {}", pwd1);
    println!("Second pwd is: {}", pwd2);
}
