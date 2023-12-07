use std::collections::HashMap;
use std::fs;

// Step 1: Read Input
// Reads and parses the input file
fn read_input(filename: &str) -> (Vec<i32>, Vec<HashMap<i32, i32>>) {
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let mut lines = content.lines();

    // Parse initial seeds
    let seeds: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1) // Skip the "seeds:" prefix
        .map(|s| s.parse().unwrap())
        .collect();

    let mut mappings: Vec<HashMap<i32, i32>> = Vec::new();
    let mut map = HashMap::new();

    for line in lines {
        if line.trim().is_empty() {
            // When encountering an empty line, push the current map and reset it for the next section
            if !map.is_empty() {
                mappings.push(map);
                map = HashMap::new();
            }
            continue;
        }

        // Skip header lines
        if line.ends_with("map:") {
            continue;
        }

        let values: Vec<i32> = line.split_whitespace()
                                   .map(|s| s.parse().unwrap())
                                   .collect();

        let (start_dest, start_src, length) = (values[0], values[1], values[2]);
        for i in 0..length {
            map.insert(start_src + i, start_dest + i);
        }
    }

    // Push the last map
    if !map.is_empty() {
        mappings.push(map);
    }

    (seeds, mappings)
}

// Step 2: Process Mappings
// Maps a number using the provided mapping
fn map_number(number: i32, mapping: &HashMap<i32, i32>) -> i32 {
    // If the mapping contains the number, return the mapped value.
    // Otherwise, return the number itself.
    *mapping.get(&number).unwrap_or(&number)
}


// Step 3: Find Locations
// Finds the lowest location number corresponding to the initial seed numbers
fn find_lowest_location(seeds: Vec<i32>, mappings: Vec<HashMap<i32, i32>>) -> i32 {
    let mut lowest_location = i32::MAX;
    let mut index_seeds = 1;
    for seed in seeds {
        println!("Seed number{}: {}", index_seeds, seed);
        let index_map = 1;
        let mut current_number = seed;
        for mapping in &mappings {
            current_number = map_number(current_number, mapping);
            println!("Current map n{}: gives {}", index_map, current_number)
        }

        if current_number < lowest_location {
            lowest_location = current_number;
        }
        index_seeds+=1;
    }

    lowest_location
}


fn main() {
    let (seeds, mappings) = read_input("input.txt");
    println!("{:?}\\ln{:?}", seeds, mappings);
    let lowest_location = find_lowest_location(seeds, mappings);
    println!("Lowest Location Number: {}", lowest_location);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_location() {
        let (seeds, mappings) = read_input("inputtest.txt");
        println!("{:?}", seeds);
        //for mapping in &mappings {
            println!("mappings lenghts: {}", mappings.len());
            // for (key, value) in mapping {
            //     println!("{}: {}", key, value);
            // }
        for mapping in &mappings {
            let sorted_mappings = mapping.clone();
            let mut keys: Vec<i32> = sorted_mappings.keys().cloned().collect();
            keys.sort();
            for key in keys {
                println!("{}: {}", key, sorted_mappings[&key]);
            }
            println!("===================")
        }
        let lowest_location = find_lowest_location(seeds, mappings);
        println!("Lowest Location Number: {}", lowest_location);
    }
}

