use std::fs;

fn main() {
    let file_path = "day2.txt";
    let contents = fs::read_to_string(file_path);

    let mut points_one = 0;
    let mut points = 0;

    for line in contents.expect("cant parse").lines() {
        let enemy = line.chars().nth(0).unwrap();
        let me = line.chars().nth(2).unwrap();

        match me {
            'Z' => points += 3,
            'Y' => points += 2,
            'X' => points += 1,  
            _ => (),
        }
        match (enemy, me) {
            ('A', 'Y') => points_one += 1 + 3,
            ('A', 'X') => points_one += 3 + 0,
            ('A', 'Z') => points_one += 2 + 6,
            ('B', 'Y') => points_one += 2 + 3,
            ('B', 'X') => points_one += 1 + 0,
            ('B', 'Z') => points_one += 3 + 6,
            ('C', 'Y') => points_one += 3 + 3,
            ('C', 'X') => points_one += 2 + 0,
            ('C', 'Z') => points_one += 1 + 6,
            _ => (),
        }

        if (me == 'X' && enemy == 'A') || (me == 'Z' && enemy == 'C') || (me == 'Y' && enemy == 'B'){
            points += 3;
        }
        else if (me == 'Y' && enemy == 'A') || (me == 'X' && enemy == 'C') || (me == 'Z' && enemy == 'B') {
            points += 6;
        }
    }
    println!("{}", points);
    println!("{}", points_one);
}
