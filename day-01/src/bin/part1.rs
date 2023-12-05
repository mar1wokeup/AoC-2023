use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("Something went wrong reading the file");

    let sum = contents.lines()
        .filter_map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
            if !digits.is_empty() {
                Some(digits[0].to_digit(10).unwrap() * 10 + digits.last().unwrap().to_digit(10).unwrap())
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("Sum of calibration values: {}", sum);
}
