use std::fs;

fn main() {
    let contents = fs::read_to_string("input2.txt")
        .expect("Something went wrong reading the file");

    let mut digit_map = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"),
                     ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];

    digit_map.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    let sum = contents.lines()
        .map(|line| {
            let mut new_line = line.to_string();
            let new_line_copy = new_line.clone();
            let mut i = 0;
            while i < new_line_copy.len() {
                for (word, digit) in &digit_map {
                    if new_line_copy[i..].starts_with(word) {
                        new_line.replace_range(i..i+word.len(), digit);
                        i += word.len() - 1; // Skip ahead by the length of the word
                        break;
                    }
                }
                i += 1;
            }

            let digits: Vec<char> = new_line.chars().filter(|c| c.is_digit(10)).collect();
            let calibration_value = if !digits.is_empty() {
                digits[0].to_digit(10).unwrap() * 10 + digits.last().unwrap().to_digit(10).unwrap()
            } else {
                0
            };

            println!("Original line: {:?}", line);
            println!("Processed line: {:?}", new_line);
            println!("Extracted digits: {:?}", digits);
            println!("Calibration value: {:?}", calibration_value);

            println!("-----------------------------------");

            calibration_value
        })
        .sum::<u32>();

    println!("Sum of calibration values with spelled-out digits: {}", sum);
}
