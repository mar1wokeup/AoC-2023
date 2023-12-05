use std::fs;

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_numeric()
}

fn is_adjacent_to_symbol(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for (dx, dy) in directions.iter() {
        let new_i = i as i32 + dx;
        let new_j = j as i32 + dy;
        if new_i >= 0 && new_i < schematic.len() as i32 &&
           new_j >= 0 && new_j < schematic[0].len() as i32 &&
           is_symbol(schematic[new_i as usize][new_j as usize]) {
            return true;
        }
    }
    false
}

fn sum_part_numbers(schematic: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;

    for (i, row) in schematic.iter().enumerate() {
        let mut j = 0;
        while j < row.len() {
            if row[j].is_numeric() {
                let mut num = 0;
                let mut is_part_number = false;
                while j < row.len() && row[j].is_numeric() {
                    if is_adjacent_to_symbol(schematic, i, j) {
                        is_part_number = true;
                    }
                    num = num * 10 + row[j].to_digit(10).unwrap() as i32;
                    j += 1;
                }

                if is_part_number {
                    println!("Adding number: {}", num);
                    sum += num;
                }
            } else {
                j += 1;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_part_numbers() {
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
        let total = sum_part_numbers(&input);
        assert_eq!(total, 4361);
    }
}
// mod tests {
//     use super::*;

//     #[test]
//     fn test_sum_part_numbers() {
//         let input = fs::read_to_string("inputtest.txt").expect("Failed to read file");
//         let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        
//         let total = sum_part_numbers(&schematic);
//         assert_eq!(total, 4361);
//     }
// }
fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let total = sum_part_numbers(&schematic);
    println!("Total sum of part numbers: {}", total);
}