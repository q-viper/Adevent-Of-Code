use std::fs;

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

fn main() {
    // Specify the file path
    let file_path = "D:/work/Adevent-Of-Code/2024/data/day1.txt";
    let data_part: &str = "TEST";
    println!("Reading file: {file_path}");

    let (part1, part2) = match read_data(file_path) {
        Ok((part1, part2)) => (part1, part2),
        Err(e) => {
            eprintln!("{e}");
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
    if data_part=="TEST"{
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
    println!("Diff: {:?}", diff);


}
