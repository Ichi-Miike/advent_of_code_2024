use std::{fs, process::Output};
use colored::Colorize;

fn main() {
    let filename: &str = "input.txt";
    let verbose: bool = true;

    //  Get the input data
    let input: String = read_input_file(filename);

    //  Clear the screen before we get started
    std::process::Command::new("clear").status().unwrap();

    if verbose {
        println!("{}", input);
        print_spacer(2);
    }

    let separator: usize = find_separator(&input);

    let rules_0: Vec<&str> = get_rules(&input[..separator], 0);
    let rules_1: Vec<&str> = get_rules(&input[..separator], 1);

    //println!("Ordered List: {:?}", get_ordered_list(rules_0.clone(), rules_1.clone()));


    print_spacer(2);
    let data: Vec<&str> = get_values(&input[separator..]);
    let mut valid_data: Vec<&str> = Vec::new();
    let mut invalid_data: Vec<&str> = Vec::new();

    for item in data {
        let mut data_item_is_ok: bool = true;
        let values: Vec<&str> = item.split(",").collect();
        let mut i: usize = 0;
        while i < values.len() {
            let rules_1_instances: Vec<usize> = get_rules_1_instances(&rules_1, values[i]);
            if rules_1_instances.len() > 0 {
                for instance in rules_1_instances {
                    let preceder = rules_0[instance];
                    if values.contains(&preceder) {
                        let position = values.iter().position(|&x| x == preceder).unwrap();
                        if position > i {
                            data_item_is_ok = false;
                            break;
                        }
                    }
                }
            }
            i += 1;
        }
        if data_item_is_ok {
            valid_data.push(item);
        } else {
            invalid_data.push(item);
        }
    }

    if invalid_data.len() > 0 {
        fix_invalid_data2(invalid_data, rules_0, rules_1);
    }

}


fn print_results(title: &str, input: Vec<&str>, verbose: bool) -> i32 {
    println!("{}", title);

    let mut result: i32 = 0;
    for res in input {
        if verbose {
            println!("{:?} -> {}", res, get_middle_element_from_array(res).yellow());
        }
        result += parse_number(get_middle_element_from_array(res));
    }
    println!("Result: {}", result.to_string().green().bold());
    print_spacer(2);

    return result;
}




fn fix_invalid_data(input: Vec<&str>, rules_0: Vec<&str>, rules_1: Vec<&str>) -> Vec<String> {
    println!("Fixing invalid data...");
    println!("Invalid Data: {:?}", input);
    print_spacer(1);
    println!("Distinct Values: {:?}", de_duplicate_array(input.clone()));


    let mut output: Vec<String> = Vec::new();

    for item in input {
        let mut fixed_data: Vec<&str> = Vec::new();

        let values: Vec<&str> = item.split(",").collect();
        let mut i: usize = 0;
        while i < values.len() {
            let value: &str = values[i];
            //println!("Value: {}", value);
            let rules_1_instances: Vec<usize> = get_rules_1_instances(&rules_1, value);
            if rules_1_instances.len() == 0 {
                fixed_data.push(value);
            } else {
                for p in rules_1_instances {
                    let preceder = rules_0[p];
                    if item.contains(preceder) {
                        fixed_data.push(preceder);
                    }
                }
                fixed_data.push(value);
            }
            i += 1;
        }
        output.push(de_duplicate_array(fixed_data).join(","));
    }
    return output;
}



fn get_distinct_values(input: Vec<&str>) -> Vec<&str> {
    println!("Getting distinct values...");

    let output: Vec<&str>;
    let mut tmp_output: Vec<&str> = Vec::new();

    for item in input {
        let data: Vec<&str> = item.split(",").collect();
        for el in data {
            tmp_output.push(el);
        }
    }

    output = de_duplicate_array(tmp_output);

    println!("Output: {:?}", output);
    print_spacer(2);

    return output;
}


fn sort_values(input: &Vec<String>, rules_0: &Vec<&str>, rules_1: &Vec<&str>) -> (bool, Vec<String>) {
    println!("Sorting Values... {:?}", input);

    let mut output: Vec<String> = Vec::new();
    let mut tmp_output: Vec<String> = Vec::new();

    let mut  updated: bool = false;

    let mut value_counter: usize = 0;
    while value_counter < input.len() {
        
        let mut rule_matched: bool = false;
        let val_a: String = input[value_counter].to_string();
        let val_b: String = match value_counter == input.len() - 1 {
            false => input[value_counter + 1].to_string(),
            true => input[value_counter].to_string()
        };

        let mut rules_counter: usize = 0;
        while rules_counter < rules_0.len() && !rule_matched {
            if rules_0[rules_counter] == val_a && rules_1[rules_counter] == val_b {
                tmp_output.push(val_a.to_string());
                rule_matched = true;
            }
            else if rules_0[rules_counter] == val_b && rules_1[rules_counter] == val_a {
                tmp_output.push(val_b.to_string());
                tmp_output.push(val_a.to_string());
                rule_matched = true;
                updated = true;
                value_counter += 1;
            }
            rules_counter += 1;
        }
        if !rule_matched {
            tmp_output.push(val_a.to_string());
        }

        value_counter += 1;
    }

    output = tmp_output;

    return (updated, output);
}


fn fix_invalid_data2(input: Vec<&str>, rules_0: Vec<&str>, rules_1: Vec<&str>) {
    println!("Fixing invalid data...");
    println!("Invalid Data: {:?}", input);
    print_spacer(1);

    let mut total: i32 = 0;

    for item in input {
        let data: Vec<&str> = item.split(",").collect();
        let mut tmp_data: Vec<String> = Vec::new();
        for d in data {
            tmp_data.push(d.to_string());
        }

        let mut list_updated: bool = true;
        while list_updated {
            let result:(bool, Vec<String>) = sort_values(&tmp_data, &rules_0, &rules_1);
            list_updated = result.0;
            tmp_data = result.1;
        }
        println!("Sorted List: {:?}", tmp_data);

        total += parse_number(get_middle_element_from_array(tmp_data.join(",").as_ref()));
    }

    println!("Fixed Total: {}", total);



}


fn de_duplicate_array(input: Vec<&str>) -> Vec<&str> {
    let mut output: Vec<&str> = Vec::new();

    for item in input {
        if !output.contains(&item) {
            output.push(item);
        }
    }

    return output;
}


fn print_spacer(no_of_lines: i32) {
    let mut i: i32 = 0;
    while i < no_of_lines {
        println!("");
        i += 1;
    }
}


fn parse_number(input: &str) -> i32 {
    return input.parse::<i32>().unwrap();
}


fn get_rules_1_instances(rules: &Vec<&str>, value: &str) -> Vec<usize> {
    let mut instances: Vec<usize> = Vec::new();

    let mut i: usize = 0;
    while i < rules.len() {
        if rules[i] == value {
            instances.push(i);
        }
        i += 1;
    }

    return instances;
}


fn find_separator(input: &str) -> usize {
    return input.find("\n\n").unwrap();
}


fn get_rules(input: &str, position: i32) -> Vec<&str> {
    let mut rules: Vec<&str> = Vec::new();

    for line in input.lines() {
        let value_1: &str = line.split("|").nth(0).unwrap();
        let value_2: &str = line.split("|").nth(1).unwrap();

        match position {
            0 => rules.push(value_1),
            1 => rules.push(value_2),
            _ => panic!("Invalid position")
        }
    }
    return rules;
}



fn get_values (input: &str) -> Vec<&str> {
    return input.trim().split("\n").collect::<Vec<&str>>();
}



fn get_middle_element_from_array(input: &str) -> &str {

    let arr = input.split(",").collect::<Vec<&str>>();

    let middle: usize = arr.len() / 2;

    return arr[middle];
}



fn read_input_file(filename: &str) -> String {
    let file_contents = fs::read_to_string(filename).expect("Failed to read file");

    return file_contents;
}