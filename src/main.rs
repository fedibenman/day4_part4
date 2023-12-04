use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("file.txt")?;
    let reader = io::BufReader::new(file);

    let mut modified_lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let unmodified_lines = modified_lines.clone(); // Clone unmodified_lines

    let mut additional_collection: Vec<String> = Vec::new();
    let mut i = 0;

    while i < modified_lines.len() {
        let text = &modified_lines[i];

        // Handle each line as needed

        let after_colon: String = text.split(':').skip(1).collect();
        let split_strings: Vec<&str> = after_colon.split('|').flat_map(|s| s.split(',')).collect();
        let winning_string: &str = split_strings[0];
        let your_string: &str = split_strings[1];
        let winning_numbers: Vec<&str> = winning_string.split_whitespace().collect();
        let your_numbers: Vec<&str> = your_string.split_whitespace().collect();

        let common_numbers: Vec<&&str> = winning_numbers.iter().filter(|&x| your_numbers.contains(x)).collect();
        let common_count = common_numbers.len();

        if let Some(index) = unmodified_lines.iter().position(|line| line == &modified_lines[i]) {
            for k in index + 1..index + common_count + 1 {
                additional_collection.push(unmodified_lines[k].clone());
            }
        }

        println!("additional {}", additional_collection.len());
        modified_lines.splice(i + 1..i + 1, additional_collection.drain(..));
        modified_lines.sort_by(|a, b| {
            let a_num: i32 = a.split(':').next().and_then(|s| s.split_whitespace().last()).and_then(|s| s.parse().ok()).unwrap_or(0);
            let b_num: i32 = b.split(':').next().and_then(|s| s.split_whitespace().last()).and_then(|s| s.parse().ok()).unwrap_or(0);
            a_num.cmp(&b_num)
        });
        println!("index:{} modified_lines = {:?}", i, modified_lines);

        i += 1;
    }

    println!("length = {} ", modified_lines.len());

    Ok(())
}
