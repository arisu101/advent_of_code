use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_file(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut left_value: Vec<i32> = Vec::new();
    let mut right_value: Vec<i32> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                let parts: Vec<&str> = line_content.split_whitespace().collect();
                if parts.len() == 2 {
                    if let (Ok(left), Ok(right)) =
                        (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                    {
                        left_value.push(left);
                        right_value.push(right);
                    } else {
                        println!("skipping line due to invalid format:{}", line_content)
                    }
                } else {
                    eprintln!(
                        "Skipping line due to incorrect number of parts: {}",
                        line_content
                    );
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
    Ok((left_value, right_value))
}

fn distance_between_list(file_path: &str) -> i32 {
    match read_file(file_path) {
        Ok((mut left_values, mut right_values)) => {
            let mut count: i32 = 0;
            println!("Left values: {}", left_values.len());
            println!("Right values: {}", right_values.len());
            left_values.sort();
            right_values.sort();
            for i in 0..left_values.len() {
                let distance = (left_values[i] - right_values[i]).abs();
                count += distance;
            }
            return count;
        }
        Err(e) => {
            eprintln!("Error processing file: {}", e);
        }
    }
    0
}

fn find_similarity_score(file_path: &str) -> i32 {
    match read_file(file_path) {
        Ok((mut left_values, mut right_values)) => {
            let mut score: i32 = 0;
            left_values.sort();
            right_values.sort();
            dbg!(&left_values);
            dbg!(&right_values);
            for i in 0..left_values.len() {
                let mut occurance = 0;
                occurance = right_values
                    .iter()
                    .filter(|&&x| x == left_values[i])
                    .count();
                score += occurance as i32 * left_values[i];
            }
            return score;
        }
        Err(e) => {
            eprintln!("Error processing file: {}", e);
        }
    }
    0
}

fn main() {
    let file_path = "inputs.txt";
    //dbg!(distance_between_list(file_path));
    dbg!(find_similarity_score(file_path));
}
