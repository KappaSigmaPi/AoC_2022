use std::fs;

fn main() {
    let file_path = "day1.txt";
    let contents = fs::read_to_string(file_path) 
        .expect("Can't read file");

    let mut most_calories1 = 0;
    let mut most_calories2 = 0;
    let mut most_calories3 = 0;
    let mut current_calories = 0;

    for line in contents.lines() {
        if line.is_empty() {
            if current_calories > most_calories1 {
                most_calories3 = most_calories2;
                most_calories2 = most_calories1;
                most_calories1 = current_calories;
            }
            else if current_calories > most_calories2 {
                most_calories3 = most_calories2;
                most_calories2 = current_calories;
            }
            else if current_calories > most_calories3 {
                most_calories3 = current_calories;
            }
            current_calories = 0;
        }
        else {
            current_calories += line.parse::<i32>().unwrap();
        }
    }
    println!("{}", most_calories1 + most_calories2 + most_calories3);
}
