use std::fs;

fn main() {

    let file_path = "day4.txt";
    let contents = fs::read_to_string(file_path).expect("can't read input");
        
    let mut counter = 0;
    let mut counter1 = 0;

    for line in contents.lines() {
        let intervals1: Vec<&str> = line.split(&[',','-']).collect();
        let intervals: Vec<u32> = intervals1.iter().map(|e| e.parse::<u32>().unwrap()).collect();

        if (intervals[1] >= intervals[2]) && (intervals[1] <= intervals[3]) ||
            (intervals[0] >= intervals[2]) && (intervals[0] <= intervals[3])||
            (intervals[0] <= intervals[2] && intervals[1] >= intervals[3]) {
                counter1 += 1;
            }

        if intervals[0] <= intervals[2] && intervals[1] >= intervals[3] { 
            counter += 1; 
            println!("{}: {} {} {} {}",counter , intervals[0],intervals[1], intervals[2], intervals[3]);        
        }
        else if intervals[0] >= intervals[2] && intervals[1] <= intervals[3] {
            counter += 1; 
            println!("{}: {} {} {} {}",counter , intervals[0],intervals[1], intervals[2], intervals[3]); 
        }
    }
    println!("{}", counter);
    println!("{}", counter1);

}
