use std::fs;
use std::collections::VecDeque;

fn main() {

    let file_path = "day5.txt";
    let conctents = fs::read_to_string(file_path).unwrap();

    //manually setting the stack ... i know ...
    let stack1: Vec<char> = vec!['T', 'D', 'W', 'Z', 'V', 'P'];
    let stack2: Vec<char> = vec!['L', 'S', 'W', 'V', 'F', 'J', 'D'];
    let stack3: Vec<char> = vec!['Z', 'M', 'L', 'S', 'V', 'T', 'B', 'H'];
    let stack4: Vec<char> = vec!['R', 'S', 'J'];
    let stack5: Vec<char> = vec!['C', 'Z', 'B', 'G', 'F', 'M', 'L', 'W'];
    let stack6: Vec<char> = vec!['Q', 'W', 'V', 'H', 'Z', 'R', 'G', 'B'];
    let stack7: Vec<char> = vec!['V', 'J', 'P', 'C', 'B', 'D', 'N'];
    let stack8: Vec<char> = vec!['P', 'T', 'B', 'Q'];
    let stack9: Vec<char> = vec!['H', 'B', 'Z', 'R', 'C'];

    let mut stacks: Vec<Vec<char>> = vec![stack1, stack2, stack3, stack4, stack5, stack6, stack7, stack8, stack9];

    for line in conctents.lines().skip(10) {
        let numbers_str: Vec<&str> = line.split(' ').collect();
        let numbers_need: Vec<usize> = vec![numbers_str[1].parse::<usize>().unwrap(),
                                           numbers_str[3].parse::<usize>().unwrap(),
                                           numbers_str[5].parse::<usize>().unwrap()];
        //let numbers: Vec<u32> = numbers_neeed.iter().map(|e| e.parse)
        //println!("{} {} {}", numbers_need[0], numbers_need[1], numbers_need[2]);

        let mut buf: VecDeque<char> = VecDeque::new(); 
        for _ in 1..=numbers_need[0] {
            let val = stacks[numbers_need[1] - 1].pop().unwrap();
            buf.push_back(val);    
        }
        for _ in 1..=buf.len() {
            let val = buf.pop_back().unwrap();
            stacks[numbers_need[2] - 1].push(val);
        }
    }
    for mut s in stacks {
        match s.pop() {
            Some(character) => print!("{}", character),
            None => print!(" "),
        }
    }
    println!();
}
