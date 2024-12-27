use std::fs;
use regex::Regex;

fn main() {
    //  Set the filename
    let filename = "input.txt";
    let verbose: bool = false;

    //  Clear the screen before we get started
    std::process::Command::new("clear").status().unwrap();

    //  Get the input as define the grid
    let input_text: String = read_input_file(filename);

    if verbose {
        println!("{}", input_text);
        println!("");
    }

    let horizontal_count: i32 = get_horizontatal_count(&input_text);
    let vertical_count: i32 = get_vertical_count(&input_text);
    let diagonal_count_1: i32 = get_diagonal_count_asc(&input_text);
    let diagonal_count_2: i32 = get_diagonal_count_desc(&input_text);


    println!("H - {}", horizontal_count);
    println!("V - {}", vertical_count);
    println!("D1 - {}", diagonal_count_1);
    println!("D2 - {}", diagonal_count_2);
    println!("Total: {}", horizontal_count + vertical_count + diagonal_count_1 + diagonal_count_2);
    println!("");

    println!("Part 2");
    let x_mas: i32 = get_x_mas(&input_text);

    println!("X-MAS: {}", x_mas);
}


fn get_horizontatal_count(input: &str) -> i32 {
    println!("Getting Horizontal Count");

    let forwards: i32 = input.matches("XMAS").count() as i32;
    let backwards: i32 = input.matches("SAMX").count() as i32;

    return forwards + backwards;
}


fn get_vertical_count(input: &str) -> i32 {
    println!("Getting Vertical Count");

    let input_lines: Vec<&str> = get_array_of_lines(input);

    let mut r: usize = 0;     //  row index
    let mut c: usize = 0;     //  column index

    let rows: usize = input_lines.len();
    let cols: usize = input_lines[0].len();

    let mut forwards: i32 = 0;
    let mut backwards: i32 = 0;

    while r < rows - 3 {
        c = 0;
        while c < cols{
            if input_lines[r as usize].chars().nth(c).unwrap() == 'X'
                && input_lines[r + 1].chars().nth(c).unwrap() == 'M'
                && input_lines[r + 2].chars().nth(c).unwrap() == 'A'
                && input_lines[r + 3].chars().nth(c).unwrap() == 'S'
            {
                forwards += 1;
            }

            if input_lines[r as usize].chars().nth(c).unwrap() == 'S'
                && input_lines[r + 1].chars().nth(c).unwrap() == 'A'
                && input_lines[r + 2].chars().nth(c).unwrap() == 'M'
                && input_lines[r + 3].chars().nth(c).unwrap() == 'X'
            {
                backwards += 1;
            }
            c += 1;
        }
        r += 1;
    }

    return forwards + backwards;
}


fn get_diagonal_count_asc(input: &str) -> i32 {
    println!("Getting Diagonal Count (ascending)");


    let input_lines: Vec<&str> = get_array_of_lines(input);

    let mut r: usize = 3;     //  row index
    let mut c: usize = 0;     //  column index

    let rows: usize = input_lines.len();
    let cols: usize = input_lines[0].len();

    let mut forwards: i32 = 0;
    let mut backwards: i32 = 0;

    while r < rows {
        c = 0;
        while c < cols - 3 {
            if input_lines[r as usize].chars().nth(c).unwrap() == 'X'
                && input_lines[r - 1].chars().nth(c + 1).unwrap() == 'M'
                && input_lines[r - 2].chars().nth(c + 2).unwrap() == 'A'
                && input_lines[r - 3].chars().nth(c + 3).unwrap() == 'S'
            {
                forwards += 1;
            }

            if input_lines[r as usize].chars().nth(c).unwrap() == 'S'
                && input_lines[r - 1].chars().nth(c + 1).unwrap() == 'A'
                && input_lines[r - 2].chars().nth(c + 2).unwrap() == 'M'
                && input_lines[r - 3].chars().nth(c + 3).unwrap() == 'X'
            {
                backwards += 1;
            }
            c += 1;
        }
        r += 1;
    }

    return forwards + backwards;
}


fn get_diagonal_count_desc(input: &str) -> i32 {
    println!("Getting Diagonal Count (descending)");

    let input_lines: Vec<&str> = get_array_of_lines(input);

    let mut r: usize = 0;     //  row index
    let mut c: usize = 3;     //  column index

    let rows: usize = input_lines.len();
    let cols: usize = input_lines[0].len();

    let mut forwards: i32 = 0;
    let mut backwards: i32 = 0;

    while r < rows - 3 {
        c = 0;
        while c < cols - 3 {
            if input_lines[r as usize].chars().nth(c).unwrap() == 'X'
                && input_lines[r + 1].chars().nth(c + 1).unwrap() == 'M'
                && input_lines[r + 2].chars().nth(c + 2).unwrap() == 'A'
                && input_lines[r + 3].chars().nth(c + 3).unwrap() == 'S'
            {
                forwards += 1;
            }

            if input_lines[r as usize].chars().nth(c).unwrap() == 'S'
                && input_lines[r + 1].chars().nth(c + 1).unwrap() == 'A'
                && input_lines[r + 2].chars().nth(c + 2).unwrap() == 'M'
                && input_lines[r + 3].chars().nth(c + 3).unwrap() == 'X'
            {
                backwards += 1;
            }
            c += 1;
        }
        r += 1;
    }

    return forwards + backwards;
}


fn get_x_mas(input: &str) -> i32 {

    //  Define th patterns we are looking for
    let patterns: Vec<(i32, Regex, Regex, Regex)> = vec![
        (0, Regex::new(r"M.M").unwrap(), Regex::new(r".A.").unwrap(), Regex::new(r"S.S").unwrap()),
        (1, Regex::new(r"M.S").unwrap(), Regex::new(r".A.").unwrap(), Regex::new(r"M.S").unwrap()),
        (2, Regex::new(r"S.M").unwrap(), Regex::new(r".A.").unwrap(), Regex::new(r"S.M").unwrap()),
        (3, Regex::new(r"S.S").unwrap(), Regex::new(r".A.").unwrap(), Regex::new(r"M.M").unwrap())
    ];

    let mut r: usize = 0;     //  row index
    let mut c: usize = 0;     //  column index

    let rows: usize = input.lines().count();
    let cols: usize = input.lines().nth(0).unwrap().chars().count();

    println!("Rows: {}... Cols: {}... {}", rows, cols, rows * cols);

    let mut count: i32 = 0;

    while r < rows - 2 {
        let line_1: &str = input.lines().nth(r).unwrap();
        let line_2: &str = input.lines().nth(r + 1).unwrap();
        let line_3: &str = input.lines().nth(r + 2).unwrap();

        c = 0;
        while c < cols - 2 {
            for pattern in patterns.iter() {
                let test_1: &str = &line_1[c..c + 3];
                let test_2: &str = &line_2[c..c + 3];
                let test_3: &str = &line_3[c..c + 3];

                if pattern.1.is_match(test_1)
                    && pattern.2.is_match(test_2)
                    && pattern.3.is_match(test_3)
                {
                    count += 1;
                }
            }
            c += 1;
        }
        r += 1;
    }

    return count;
}

fn get_array_of_lines(input: &str) -> Vec<&str> {
    return input.split("\n").collect::<Vec<&str>>();
}


fn read_input_file(filename: &str) -> String {
    let file_contents = fs::read_to_string(filename).expect("Failed to read file");

    return file_contents;
}
