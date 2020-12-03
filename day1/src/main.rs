use lazy_static::lazy_static;

fn main() {
    lazy_static! {
        static ref FILE_STRING: String =
            std::fs::read_to_string("/home/vid/Projekti/AoC/day1/src/input.txt").unwrap();
    }
    // let lines = FILE_STRING.lines();
    let lines = FILE_STRING.lines();
    let mut parsed_lines = Vec::new();
    for line in lines {
        parsed_lines.push(line.parse::<i32>().unwrap())
    }
    // 1st part
    for value in &parsed_lines {
        let target = 2020 - value;
        if parsed_lines.iter().any(|&i| i == target) {
            println!("{} x {} = {}", value, target, value * target);
            break;
        }
    }
    // 2nd part
    // for value in &parsed_lines {
    //     let moving_target = 2020 - value;
    //     for sub_value in &parsed_lines {
    //         let second_target = moving_target - sub_value;
    //         for subvalue in &parsed_lines {
    //             let target = second_target - subvalue;
    //             if parsed_lines.iter().any(|&i| i == target) {
    //                 println!(
    //                     "{} x {} x {} = {}",
    //                     value,
    //                     subvalue,
    //                     target,
    //                     value + moving_target + second_target
    //                 );
    //                 break;
    //             }
    //         }
    //     }
    // }
    parsed_lines.sort();
    let lenght = parsed_lines.len();
    println!("{}", lenght);

    for i in 0..(lenght - 2) {
        let mut bottom = i + 1;
        let mut top = lenght - 1;
        while bottom < top {
            if parsed_lines[i] + parsed_lines[bottom] + parsed_lines[top] == 2020 {
                println!(
                    "{} x {} x {} = {}",
                    parsed_lines[i],
                    parsed_lines[bottom],
                    parsed_lines[top],
                    parsed_lines[i] * parsed_lines[bottom] * parsed_lines[top]
                );
                break;
            } else if parsed_lines[i] + parsed_lines[bottom] + parsed_lines[top] < 2020 {
                bottom += 1;
            } else {
                top -= 1;
            }
        }
    }

    // let values = lines.map(|line| line.parse::<i32>().unwrap());

    // println!("{:?}", parsed_lines);
}
