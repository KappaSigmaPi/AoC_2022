#![feature(iter_array_chunks)]

use std::fs;
use std::collections::HashSet;

fn main() {
    let file_path = "day3.txt";
    let contents = fs::read_to_string(file_path);

    let mut points = 0;


    for [line1, line2, line3] in contents.expect("can't parse").lines().array_chunks() {
        //println!("{} {} {}", line1, line2, line3);

        let mut first_set: HashSet<char> = HashSet::new();
        for letter in line1.chars() {
            first_set.insert(letter);
        }

        let mut second_set: HashSet<char> = HashSet::new();
        for letter in line2.chars() {
            second_set.insert(letter);
        }

        let mut third_set: HashSet<char> = HashSet::new();
        for letter in line3.chars() {
            third_set.insert(letter);
        }

        for x in first_set.intersection(&second_set) {
            if third_set.contains(x) {
                match x {
                    'A' ..= 'Z' => {
                        // x.encode_utf8(val);
                        points += (*x as u32) - 38;
                    },
                    'a' ..= 'z' => {
                        //x.encode_utf8(val);
                        points += (*x as u32) - 96;
                    }
                    _ => println!("no item")
                }
            }
        }
    }
    // for line in contents.expect("can't parse").lines() {
    //     println!("{line}");
    //     let first_part = &line[0..(line.len()/2)];
    //     println!("{}", first_part);
    //     let second_part = &line[line.len()/2..line.len()];
    //     println!("{}", second_part);


    //     let mut first_set: HashSet<char> = HashSet::new();
    //     for letter in first_part.chars() {
    //         first_set.insert(letter);
    //     }
    //     let mut second_set: HashSet<char> = HashSet::new();
    //     for letter in second_part.chars() {
    //         second_set.insert(letter);
    //     }
    //     for x in second_set.intersection(&first_set) {
    //         if *x == 'a' {
    //            println!("{} - {}", x, (*x as u32) - 96);
    //         }
    //         if *x == 'L' {
    //            println!("{} - {}", x, (*x as u32) - 38);
    //         }
    //         match x {
    //             'A' ..= 'Z' => {
    //                 // x.encode_utf8(val);
    //                 points += (*x as u32) - 38;
    //             },
    //             'a' ..= 'z' => {
    //                 //x.encode_utf8(val);
    //                 points += (*x as u32) - 96;
    //             }
    //             _ => println!("no item")
    //         }
    //     }
    // }
        println!("{}", points)
}

