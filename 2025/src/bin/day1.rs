use std::fs;

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read input file")
}

fn main() {
    // let input = read_input("data/test_day1.txt");
    let input = read_input("data/day1.txt");
    let dials = input.lines();

    let mut pwd = 0i64;
    let mut pos = 50;

    dials.for_each(|dial| {
        let dial_ch: Vec<char> = dial.chars().collect();
        let direction = dial_ch[0];
        let steps = dial_ch[1..]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        if direction == 'R' {
            if steps >= (100 - pos) {
                pwd += 1 + ((steps - 100 + pos) / 100) as i64;
                pos = (pos + steps) % 100;
            } else {
                pos += steps;
            }
        } else if direction == 'L' {
            if steps >= pos {
                pwd += 1 + ((steps - pos) / 100) as i64;
                if pos == 0 {
                    pwd -= 1;
                }
                pos = (pos - steps).rem_euclid(100);
            } else {
                pos -= steps;
            }
        }

        // println!("Dialing {}  {}  {}", direction, steps, pwd);

        // pos = pos.rem_euclid(100);
        //
        // if pos == 0 {
        //     pwd += 1;
        // }

        // println!("Dialing {}  {}  {}", direction, steps, pwd);
    });

    println!("The password is: {}", pwd);
}
