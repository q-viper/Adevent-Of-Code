use std::{array, fs};
use std::collections::HashMap;

fn read_data(file_path: &str) -> Result<(String, String), String> {
    // Read the file
    let contents = fs::read_to_string(file_path)
        .map_err(|e| format!("Error reading the file: {}", e))?;

    // Split the content into two parts
    let mut parts = contents.split("Split From Here");

    let part1 = parts
        .next()
        .ok_or("Error: Could not find part1 in the file.".to_string())?
        .trim()
        .to_string();

    let part2 = parts
        .next()
        .ok_or("Error: Could not find part2 in the file.".to_string())?
        .trim()
        .to_string();

    // Return the two parts
    Ok((part1, part2))
}

fn day1_solve(file_path: &str, data_part: &str){
    // Read the data from the file
    let (part1, part2) =  match read_data(file_path) {
        Ok((part1, part2)) => (part1, part2),
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // Initialize vectors for test and train data
    let mut test_data: Vec<(i32, i32)> = Vec::new();
    let mut train_data: Vec<(i32, i32)> = Vec::new();

    // Parse part1 into test_data
    for line in part1.lines() {
        let mut parts = line.split_whitespace(); // Split the line by spaces
        if let (Some(x), Some(y)) = (parts.next(), parts.next()) {
            if let (Ok(x), Ok(y)) = (x.parse::<i32>(), y.parse::<i32>()) {
                test_data.push((x, y));
            } else {
                eprintln!("Warning: Could not parse line in part1: '{line}'");
            }
        } else {
            eprintln!("Warning: Malformed line in part1: '{line}'");
        }
    }

    // Parse part2 into train_data
    for line in part2.lines() {
        let mut parts = line.split_whitespace(); // Split the line by spaces
        if let (Some(x), Some(y)) = (parts.next(), parts.next()) {
            if let (Ok(x), Ok(y)) = (x.parse::<i32>(), y.parse::<i32>()) {
                train_data.push((x, y));
            } else {
                eprintln!("Warning: Could not parse line in part2: '{line}'");
            }
        } else {
            eprintln!("Warning: Malformed line in part2: '{line}'");
        }
    }

    // Output the parsed data
    println!("Test Data: {:?}", test_data);
    println!("Train Data: {:?}", train_data);

    let mut curr_data: Vec<(i32, i32)> = Vec::new();
    if data_part=="PART1"{
        curr_data= test_data.clone();
    }
    else{
        curr_data = train_data.clone();
    }
    

    let col1: Vec<i32> = curr_data.iter().map(|(x, _)| *x).collect();
    let col2: Vec<i32> = curr_data.iter().map(|(_, y)| *y).collect();

    println!("Column 1: {:?}", col1);
    println!("Column 2: {:?}", col2);

    // sort col1 and col2
    let mut col1_sorted = col1.clone();

    col1_sorted.sort();
    let mut col2_sorted = col2.clone();
    col2_sorted.sort();

    let mut diff: i32 = 0;
    for i in 0..col1.len() {
        let x_sorted = col1_sorted[i];
        let y_sorted = col2_sorted[i];
        diff += (x_sorted-y_sorted).abs();

    }
    print!("Day 1 sol 1: {:?}", diff);

    // now for the second part
    let mut counts = HashMap::new();

    // Count the occurrences of each element
    for item in col2 {
        *counts.entry(item).or_insert(0) += 1;
    }

    // read elements in first col and if it exists in the counts, multiply the counts value and sum
    let mut sol2: i32 = 0;
    for item in col1 {
        if counts.contains_key(&item){
            sol2 += item * counts[&item];
        }
    }
    print!("\n");
    print!("Day 1 sol 2: {:?}", sol2);




}


fn is_safe1(elem_arr: Vec<i32>){
//

}

fn day2_solve(file_path: &str, data_part: &str) {
    // Read the parts first
    let (part1, part2) = match read_data(file_path) {
        Ok((part1, part2)) => (part1, part2),
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    println!("{}", part1);
    println!("{}", part2);

    // Initialize vectors for test and train data

    // Determine the current part to process
    let mut curr_part: String = part1;
    if data_part != "TEST" {
        curr_part = part2;
    }

    // Parse `curr_part` into `test_data` or `train_data`
    let mut safe_cnt: i32 = 0; // Counter for safe data
    for line in curr_part.lines() {
        let parts = line.split_whitespace(); // Split the line by spaces

        let mut prev_elem = 0;
        let mut is_inc = true; // Track if the sequence is increasing

        let mut is_safe = true;
        let elem_arr: Vec<i32> = parts.into_iter() // Iterate over string list
        .map(|s| s.parse::<i32>().expect("Failed to parse")) // Parse each string to an integer
        .collect(); 

        for (i, elem) in parts.enumerate() {
            // Parse element to integer
            let element: i32 = elem.parse().expect("Failed to parse element");

            if i == 0 {
                // Initialize `prev_elem` on the first iteration
                prev_elem = element;
                continue;
            }

            // Check if sequence starts decreasing
            if element - prev_elem < 0 && i == 1 {
                is_inc = false;
            }

            // Check for invalid conditions
            if (element - prev_elem).abs() > 3
                || (element - prev_elem).abs() == 0
                || (is_inc && element - prev_elem < 0)
                || (!is_inc && element - prev_elem > 0)
            {
                is_safe=false;
                break;

            }

            // Update `prev_elem`
            prev_elem = element;
        }

        // If the line passes all checks, increment `safe_cnt`
        if is_safe{
            safe_cnt += 1;

        }
        
    }

    println!("Part 1 Safe count: {}", safe_cnt);

    // now for the part 2

}




fn main() {
    // Specify the file path
    // TEST or none
    let data_part: &str = "";
    let curr_day = 2;
    let file_path = &format!("D:/work/Adevent-Of-Code/2024/data/day{}.txt", curr_day);


    if curr_day == 1 {
        day1_solve(file_path, data_part);
    }
    if curr_day == 2 {
        day2_solve(file_path, data_part);
    }
    // if curr_day == 3 {
    //     day3_solve(file_path, data_part);
    // }
    // if curr_day == 4 {
    //     day4_solve(file_path, data_part);
    // }
    // if curr_day == 5 {
    //     day5_solve(file_path, data_part);
    // }
    // if curr_day == 6 {
    //     day6_solve(file_path, data_part);
    // }
    // if curr_day == 7 {
    //     day7_solve(file_path, data_part);
    // }
    // if curr_day == 8 {
    //     day8_solve(file_path, data_part);
    // }
    // if curr_day == 9 {
    //     day9_solve(file_path, data_part);
    // }
    // if curr_day == 10 {
    //     day10_solve(file_path, data_part);
    // }
    // if curr_day == 11 {
    //     day11_solve(file_path, data_part);
    // }
    // if curr_day == 12 {
    //     day12_solve(file_path, data_part);
    // }
    // if curr_day == 13 {
    //     day13_solve(file_path, data_part);
    // }
    // if curr_day == 14 {
    //     day14_solve(file_path, data_part);
    // }
    // if curr_day == 15 {
    //     day15_solve(file_path, data_part);
    // }
    // if curr_day == 16 {
    //     day16_solve(file_path, data_part);
    // }
    // if curr_day == 17 {
    //     day17_solve(file_path, data_part);
    // }
    // if curr_day == 18 {
    //     day18_solve(file_path, data_part);
    // }
    // if curr_day == 19 {
    //     day19_solve(file_path, data_part);
    // }
    // if curr_day == 20 {
    //     day20_solve(file_path, data_part);
    // }
    // if curr_day == 21 {
    //     day21_solve(file_path, data_part);
    // }
    // if curr_day == 22 {
    //     day22_solve(file_path, data_part);
    // }
    // if curr_day == 23 {
    //     day23_solve(file_path, data_part);
    // }
    // if curr_day == 24 {
    //     day24_solve(file_path, data_part);
    // }
    // if curr_day == 25 {
    //     day25_solve(file_path, data_part);
    // }
}