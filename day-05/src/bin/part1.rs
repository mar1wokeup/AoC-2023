use std::fs;
use std::collections::HashMap;

fn parse_input(filename: &str) -> (Vec<i32>, Vec<Vec<(i32, i32, i32)>>) {
    let contents = fs::read_to_string(filename).expect("Failed to read file");
    let mut sections = contents.split("\n\n");
    let seeds = sections
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    let mappings = sections
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|line| {
                    let parts = line
                        .split_whitespace()
                        .map(|p| p.parse().unwrap())
                        .collect::<Vec<i32>>();
                    (parts[0], parts[1], parts[2])
                })
                .collect::<Vec<(i32, i32, i32)>>()
        })
        .collect::<Vec<_>>();

    (seeds, mappings)
}

fn convert_number(number: i32, map: &[(i32, i32, i32)]) -> i32 {
    for &(dest_start, src_start, len) in map {
        if number >= src_start && number < src_start + len {
            return dest_start + (number - src_start);
        }
    }
    number
}

fn find_lowest_location(seeds: &[i32], mappings: &[Vec<(i32, i32, i32)>]) -> i32 {
    seeds.iter()
        .map(|&seed| {
            mappings.iter().fold(seed, |acc, map| convert_number(acc, map))
        })
        .min()
        .unwrap()
}

fn main() {
    let (seeds, mappings) = parse_input("input.txt");
    let result = find_lowest_location(&seeds, &mappings);
    println!("The lowest location number is: {}", result);
}

// Tests will go here
