use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: &str = "input.txt";

    let line_array = get_line_array(filename);
    let mut list_1: Vec<String> = Vec::new();
    let mut list_2: Vec<String> = Vec::new();

    for line in line_array {
        let split_line: Vec<&str> = line.split("   ").collect();
        list_1.push(split_line[0].to_string());
        list_2.push(split_line[1].to_string());
    }

    list_1.sort();
    list_2.sort();

    let mut total: i32 = 0;

    for i in 0..list_1.len() {
        let num_1 = parse_number(&list_1[i]);
        let num_2 = parse_number(&list_2[i]);

        total += (num_1 - num_2).abs();
    }
    println!("Part 1 Answer: {}", total);
    println!("");

    ////////////////////////////////////////////////////////////////////////////////////////////
    // Part 2
    ////////////////////////////////////////////////////////////////////////////////////////////

    total = 0;
    for item in list_1 {
        let count = list_2.iter().filter(|&x| x == &item).count();
        total += parse_number(&item) * (count as i32);

        if count > 0 {
            println!("Value: {}... Occurrences: {}", item, count);
        }
    }
    println!("Part 2 Answer: {}", total);
    println!("");
}


fn parse_number(input: &str) -> i32 {
    return input.parse::<i32>().unwrap();
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn get_line_array(filename: &str) -> Vec<String> {
    let mut arr = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                arr.push(ip);
            }
        }
    }
    return arr;
}


// fn get_line_array(line_count: usize) -> Vec<String> {
//     let mut line_array: Vec<String> = vec![String::new(); line_count];
//
//     if let Ok(lines) = read_lines("input.txt") {
//         for (index, line) in lines.enumerate() {
//             if let Ok(ip) = line {
//                 line_array[index] = ip;
//             }
//         }
//     }
//
//     return line_array;
// }