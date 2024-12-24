use std::fs;
use regex::Regex;

fn main() {
    //  Set the filename
    let filename: &str = "input.txt";

    //  Set the regex patterns
    let regex_p1: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let regex_p2: Regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    //  Clear the screen before we get started
    std::process::Command::new("clear").status().unwrap();

    //  Initialize variables / get the input
    let input = read_file_as_string(filename);
    let mut total_p1: i32 = 0;
    let mut total_p2: i32 = 0;
    let mut mul_enabled: bool = true;


    //  Part 1
    total_p1 = regex_p1.captures_iter(&input)
        .map(|cap| cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap())
        .sum();

    println!("Part 1 total: {}", total_p1);


    //  Part 2
    for cap in regex_p2.captures_iter(&input) {
        if let Some(mul_cap) = cap.get(1) {
            if mul_enabled {
                let x: i32 = cap[1].parse().unwrap();
                let y: i32 = cap[2].parse().unwrap();
                total_p2 += x * y;
            }
        } else if cap.get(0).map_or("", |m| m.as_str()) == "do()" {
            mul_enabled = true;
        } else if cap.get(0).map_or("", |m| m.as_str()) == "don't()" {
            mul_enabled = false;
        }
    }

    println!("Part 2 total: {}", total_p2);
}



//  Returns the contents of a file system object as a string
fn read_file_as_string(filename: &str) -> String {
    let file_contents = fs::read_to_string(filename).expect("Failed to read file");

    return file_contents;
}
