use std::fs;

fn parse_card(card: &str) -> (Vec<i32>, Vec<i32>) {
    let parts: Vec<&str> = card.split(" | ").collect();
    let winning_numbers = parts[0].split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();
    let user_numbers = parts[1].split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();
    (winning_numbers, user_numbers)
}

fn calculate_card_points(winning_numbers: &Vec<i32>, user_numbers: &Vec<i32>) -> i32 {
    let mut points = 0;
    let mut multiplier = 1;

    for &num in user_numbers {
        if winning_numbers.contains(&num) {
            points += multiplier;
            multiplier *= 2;
        }
    }

    points
}

fn total_points(cards: &Vec<(Vec<i32>, Vec<i32>)>) -> i32 {
    cards.iter()
         .map(|(winning, user)| calculate_card_points(winning, user))
         .sum()
}

#[cfg(test)]:
mod tests {
    use super::*;

    #[test]
    fn check_test_input() {
        let input = fs::read_to_string("inputtest.txt").expect("Failed to read file");
        let cards: Vec<(Vec<i32>, Vec<i32>)> = input.lines().map(|line| parse_card(line)).collect();

        let total = total_points(cards);
        assert_eq!(total, 13);
    }
} 

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let cards: Vec<(Vec<i32>, Vec<i32>)> = input.lines().map(|line| parse_card(line)).collect();

    let total = total_points(cards);

    println!("Total points: {}", total);
}
