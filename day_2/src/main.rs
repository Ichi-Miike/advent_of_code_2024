use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //  Set the filename
    let filename: &str = "input.txt";

    //  Clear the screen before we get started
    std::process::Command::new("clear").status().unwrap();

    //  Get the list of reports from the file and initialize the 'safe_report_count' variable
    let reports: Vec<String> = get_line_array(filename);
    let mut safe_report_count: i32 = 0;

    //  Loop through the reports pulled from the file
    for report in reports {
        //  Split the report into an array and initialize an idnexer to iterate through the elements
        let report_array: Vec<&str> = report.split(" ").collect();
        let result = check_if_report_is_safe(report_array.clone());
        let mut suffix: String = String::new();

        if result.0 {
            suffix = String::from("safe");
            safe_report_count += 1;

        }
        else {
                 if apply_dampener(report_array, result.1 as usize) {
                    suffix = String::from("safe (dampened)");
                    safe_report_count += 1;
                 }
                 else {
                    suffix = String::from("not safe (failed dampening)");
                 }
        }

        println!("{} -> {}", report, suffix);

//        println!("");
    }

    println!("");
    println!("Part 1 Answer: {}", safe_report_count);
    println!("");
}

fn apply_dampener(arr: Vec<&str>, index: usize) -> bool {
    println!("Dampening.... Index {}-{}", index, index + 1);
    let mut test_a: Vec<&str> = arr.clone();
    let mut test_b: Vec<&str> = arr.clone();

    test_a.remove(index);
    test_b.remove(index + 1);

    // println!("Origin -> {}", arr.join(" "));
    // println!("Test A -> {}", test_a.join(" "));
    // println!("Test B -> {}", test_b.join(" "));

    if check_if_report_is_safe(test_a).0 {
        return true;
    }

    if check_if_report_is_safe(test_b).0 {
        return true;
    }

    return false;
}

fn check_if_report_is_safe(arr: Vec<&str>) -> (bool, i32) {
    //  Get the report trend (Increasing, Decreasing or NoChange)
    let report_trend: ReportTrend = get_report_trend(arr[0], arr[1]);

    //  Initialize the array index
    let mut i: usize = 0;

    //  Iterate through the elements of the report array
    while i < arr.len() - 1 {
        //  Get the current / next elements
        let val_0: i32 = parse_number(arr[i]);
        let val_1: i32 = parse_number(arr[i + 1]);

        let safety_check = compare_elements(val_0, val_1, report_trend);

        if !safety_check {
            return (false, i as i32);
        }

        //  Update the index counter to check the next pair of elements
        i += 1;
    }

    return (true, -1);
}

fn compare_elements(val_0: i32, val_1: i32, report_trend: ReportTrend) -> bool {
    match report_trend {
        ReportTrend::NoChange => {
            //println!("\t{}-{} _> fail", val_0, val_1);
            return false;
        }

        ReportTrend::Increasing => {
            if val_0 < val_1 && val_1 - val_0 <= 3 {
                //println!("\t{}-{} -> ok", val_0, val_1);
                return true;
            }
        }

        ReportTrend::Decreasing => {
            if val_0 > val_1 && (val_0 - val_1) <= 3 {
                //println!("\t{}-{} -> ok", val_0, val_1);
                return true;
            }
        }
    }
    //println!("\t{}-{} -> fail", val_0, val_1);
    return false;
}

fn get_report_trend(val_0: &str, val_1: &str) -> ReportTrend {
    if parse_number(val_0) < parse_number(val_1) {
        return ReportTrend::Increasing;
    }
    if parse_number(val_0) > parse_number(val_1) {
        return ReportTrend::Decreasing;
    } else {
        return ReportTrend::NoChange;
    }
}

fn parse_number(val: &str) -> i32 {
    return val.parse::<i32>().unwrap();
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

#[derive(Copy, Clone)]
enum ReportTrend {
    Increasing,
    Decreasing,
    NoChange,
}
