use std::fs;

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_numeric()
}

fn get_part_number(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> Option<i32> {
    if !schematic[i][j].is_numeric() || (j > 0 && schematic[i][j - 1].is_numeric()) {
        // Skip if not a number or part of a multi-digit number already counted
        return None;
    }

    let mut num = 0;
    let mut k = j;
    while k < schematic[i].len() && schematic[i][k].is_numeric() {
        num = num * 10 + schematic[i][k].to_digit(10).unwrap() as i32;
        k += 1;
    }
    Some(num)
}

fn get_full_number(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> Option<i32> {
    // Check if the cell is numeric
    if !schematic[i][j].is_numeric() {
        return None;
    }

    // Find the start of the number
    let mut start = j;
    while start > 0 && schematic[i][start - 1].is_numeric() {
        start -= 1;
    }

    // Extract the full number
    let mut num = 0;
    let mut k = start;
    while k < schematic[i].len() && schematic[i][k].is_numeric() {
        num = num * 10 + schematic[i][k].to_digit(10).unwrap() as i32;
        k += 1;
    }
    Some(num)
}

fn sum_gear_ratios(schematic: &Vec<Vec<char>>) -> i32 {
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut sum = 0;

    for (i, row) in schematic.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '*' {
                let mut part_numbers = Vec::new();

                for (dx, dy) in directions.iter() {
                    let new_i = i as i32 + dx;
                    let new_j = j as i32 + dy;
                    if new_i >= 0 && new_i < schematic.len() as i32 && new_j >= 0 && new_j < row.len() as i32 {
                        if let Some(num) = get_full_number(schematic, new_i as usize, new_j as usize) {
                            if !part_numbers.contains(&num) {
                                part_numbers.push(num);
                            }
                        }
                    }
                }

                if part_numbers.len() == 2 {
                    let gear_ratio = part_numbers[0] * part_numbers[1];
                    println!("Gear at ({}, {}): Numbers = {:?}, Ratio = {}", i, j, part_numbers, gear_ratio);
                    sum += gear_ratio;
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_gear_ratios() {
        let input = vec![
            "467..114..".chars().collect(),
            "...*......".chars().collect(),
            "..35..633.".chars().collect(),
            "......#...".chars().collect(),
            "617*......".chars().collect(),
            ".....+.58.".chars().collect(),
            "..592.....".chars().collect(),
            "......755.".chars().collect(),
            "...$.*....".chars().collect(),
            ".664.598..".chars().collect(),
        ];
        let total_gear_ratio = sum_gear_ratios(&input);
        assert_eq!(total_gear_ratio, 467835, "The calculated sum of gear ratios is incorrect.");
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let total_gear_ratio = sum_gear_ratios(&schematic);
    println!("Total sum of gear ratios: {}", total_gear_ratio);
}

