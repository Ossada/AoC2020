use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    lazy_static! {
        static ref FILE_STRING: String =
            std::fs::read_to_string("/home/vid/Projekti/AoC/day2/input.txt").unwrap();
    }

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<x>\d+)-(?P<y>\d+) (?P<character>.+): (?P<password>\w+)").unwrap();
    }
    let mut non_valid = 0;
    let mut valid2 = 0;

    for line in FILE_STRING.lines() {
        let parsed = RE.captures(line).unwrap();
        let bottom: usize = parsed["x"].parse().unwrap();
        let top: usize = parsed["y"].parse().unwrap();
        let character: char = parsed["character"].parse().unwrap();
        let pass: String = parsed["password"].parse().unwrap();

        non_valid += check_validity(bottom, top, character, &pass);
        valid2 += check_validity2(bottom, top, character, &pass);
    }
    println!("Valid {}", non_valid);
    println!("Valid second {}", valid2)
}

fn check_validity(x: usize, y: usize, character: char, pass: &str) -> i16 {
    let times = pass.matches(character).count();
    // println!("{}-{} {}: {}, times: {}", x, y, character, pass, times);
    if times >= x && times <= y {
        // let passed = "VALID";
        1
    } else {
        // let passed = "INVALID";
        // println!(
        //     "{}-{} {}: {}, times: {}, {}",
        //     x, y, character, pass, times, passed
        // );
        0
    }
}

fn check_validity2(x: usize, y: usize, character: char, pass: &str) -> i16 {
    let first = pass.chars().nth(x - 1).unwrap();
    let second = pass.chars().nth(y - 1).unwrap();
    if first == character && second != character || first != character && second == character {
        1
    } else {
        0
    }
}
