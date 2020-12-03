use lazy_static::lazy_static;

fn main() {
    lazy_static! {
        static ref FILE_STRING: String =
            std::fs::read_to_string("/home/vid/Projekti/AoC/day3/input.txt").unwrap();
    }
    let mut lines = FILE_STRING.lines();
    lines.next();
    lines.next();
    let mut trees = 0;
    let mut index = 1;
    let mut flag = 1;
    for line in lines {
        let lenght = line.len();

        if flag == 0 {
            flag = 1;
            continue;
        }
        if index > lenght - 1 {
            index = index - lenght;
            // println!("{}", index);
        }
        let path = line.chars().nth(index).unwrap();
        //println!("{}, {}, {}", index, lenght, path);
        if path == '#' {
            println!("{}", line);
            trees += 1;
        }
        index += 1;

        flag = 0;
    }
    println!("{}", trees);
}
