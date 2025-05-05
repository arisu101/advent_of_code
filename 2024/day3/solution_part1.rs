use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

struct Solution {
    file_path: String,
}

impl Solution {
    fn new(file_path: &str) -> Self {
        Solution {
            file_path: file_path.to_string(),
        }
    }

    fn find_mul_result(&self) -> io::Result<i32> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        let mut result: i32 = 0;

        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    let mul_matched_pattern = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();

                    for captures in mul_matched_pattern.captures_iter(&line_content) {
                        let x = captures[1].parse().unwrap_or(0);
                        let y = captures[2].parse().unwrap_or(0);
                        result += x * y;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                }
            }
        }
        Ok(result)
    }
}

fn main() {
    let file_path = "inputs.txt";
    let solution = Solution::new(file_path);

    // Check the result of safe_reports
    match solution.find_mul_result() {
        Ok(result) => println!("Total sum: {}", result),
        Err(e) => eprintln!("Error processing the file: {}", e),
    }
}
