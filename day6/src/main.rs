use std::fs;
use std::collections::VecDeque;

fn main() {
    let file_path = "day6.txt";
    let contents = fs::read_to_string(file_path).expect("cant read");

    let mut buffer: VecDeque<char> = VecDeque::new();
    let mut res = 14;

    for letter in contents.chars() {
        if buffer.len() < 14 {
            buffer.push_back(letter);
        }
        else {
            if is_marker(&buffer) {
               println!("{:?}", res);
               return;
            }
            buffer.pop_front();
            buffer.push_back(letter);
            res += 1;
        }
    }
    println!("{:?}", is_marker(&buffer));
    print!("{:?}", buffer);
}

fn is_marker(buf: &VecDeque<char>) -> bool {
    for l in buf {
        let mut counter = 0; 
        for rest in buf {
            if l == rest {
                counter += 1;
            }
        }
        if counter > 1 {
            return false;
        }
    }
    true
}
