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

    fn safe_reports(&self) -> io::Result<i32> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        let mut levels: i32 = 0;

        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    let reports: Vec<i32> = line_content
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    if Self::is_safe_report(&reports) {
                        levels += 1;
                    } else {
                        let mut is_safe: bool = false;
                        for i in 0..reports.len() {
                            let mut temp_report = reports.clone();
                            temp_report.remove(i);
                            if Self::is_safe_report(&temp_report) {
                                is_safe = true;
                            }
                        }
                        if is_safe {
                            levels += 1;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                }
            }
        }
        Ok(levels)
    }

    fn is_safe_report(report: &Vec<i32>) -> bool {
        let is_ascending = report.windows(2).all(|w| w[0] < w[1]);
        let is_descending = report.windows(2).all(|w| w[0] > w[1]);

        for i in 0..report.len() - 1 {
            let diff = (report[i] - report[i + 1]).abs();
            if diff < 1 || diff > 3 {
                return false;
            }
        }
        is_ascending || is_descending
    }
}
//Conditions for safe:
//1. the difference between adjacents elements should be at least one and at most 3
//2. the levels are either increasing or decreasing

fn main() {
    let file_path = "inputs.txt";
    let solution = Solution::new(file_path);
    dbg!(&solution.safe_reports());
}
