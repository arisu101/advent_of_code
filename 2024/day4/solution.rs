use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    usize,
};

fn find_xmas_(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut input_words: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                input_words.push(line_content);
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
    Ok(input_words)
}

fn calculate(file_path: &str) -> usize {
    let input_words = match find_xmas_(file_path) {
        Ok(words) => words,
        Err(e) => {
            eprintln!("{e}");
            return 0;
        }
    };
    let mut count: usize = 0;
    //first count in linear form for left to right and right to left
    for lines in &input_words {
        count += find_words_in_linear(lines);
        dbg!(&count);
        let lines_reverse: String = lines.chars().rev().collect();
        count += find_words_in_linear(&lines_reverse);
    }
    //now create a vector to store the values of up and down values and find the count in them
    for i in 0..10 {
        let mut word: String = input_words
            .iter()
            .filter_map(|s| s.chars().nth(i))
            .collect();
        count += find_words_in_linear(&word);
        word = word.chars().rev().collect();
        count += find_words_in_linear(&word);
    }

    //now for diagonal
    let len = input_words.len();
    let wlen = input_words[0].len();
    let _positive_string_right: String = String::new();
    let _positive_string_down: String = String::new();
    let mut negative_string_right: String = String::new();
    let mut negative_string_down: String = String::new();
    for i in 0..wlen {
        let mut a = 0;
        let mut b = 0;
        for j in i..len {
            negative_string_right.push(input_words[a].chars().nth(j).unwrap());
            a += 1;
        }
        if negative_string_right.len() == 4 {
            count += find_words_in_linear(&negative_string_right);
            negative_string_right = negative_string_right.chars().rev().collect();
            count += find_words_in_linear(&negative_string_right);
        }
        for k in i + 1..len {
            negative_string_down.push(input_words[k].chars().nth(b).unwrap());
            b += 1;
        }

        if negative_string_down.len() == 4 {
            count += find_words_in_linear(&negative_string_down);
            negative_string_down = negative_string_right.chars().rev().collect();
            count += find_words_in_linear(&negative_string_down);
        }
        negative_string_right.clear();
        negative_string_down.clear();
    }

    return count;
}

fn find_words_in_linear(lines: &str) -> usize {
    let word: String = String::from("XMAS");
    let count = lines.matches(&word).count();
    count
}

fn main() {
    let file_path = "test_inputs.txt";
    dbg!(calculate(&file_path));
}
