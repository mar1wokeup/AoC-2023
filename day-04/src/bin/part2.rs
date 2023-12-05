use std::fs;
use std::collections::VecDeque;
use std::time::Instant;
    
fn parse_card(card: &str) -> (Vec<i32>, Vec<i32>) {
    let parts: Vec<&str> = card.split(" | ").collect();
    let winning_numbers = parts[0].split_whitespace()
                                  .filter_map(|num| num.parse::<i32>().ok())
                                  .collect();
    let user_numbers = parts[1].split_whitespace()
                                .filter_map(|num| num.parse::<i32>().ok())
                                .collect();
    (winning_numbers, user_numbers)
}

fn calculate_card_points(winning_numbers: &Vec<i32>, user_numbers: &Vec<i32>) -> i32 {
    let mut points = 0;

    for &num in user_numbers {
        if winning_numbers.contains(&num) {
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
            println!("set {:?}, found {}", user_numbers, &num);
        }
    }
    println!("set is worth {:?} points", points);
    points
}

#[allow(dead_code)]
fn total_points(cards: &Vec<(Vec<i32>, Vec<i32>)>) -> i32 {
    cards.iter()
         .map(|(winning, user)| calculate_card_points(winning, user))
         .sum()
}

fn count_matches(winning_numbers: &Vec<i32>, user_numbers: &Vec<i32>) -> usize {
    user_numbers.iter().filter(|&&num| winning_numbers.contains(&num)).count()
}

fn process_cards_opti(cards: &Vec<(Vec<i32>, Vec<i32>)>) -> usize {
    //println!("testing with opti function");
    //let start = Instant::now();
    
    let mut total_cards = 0;
    let match_counts: Vec<usize> = cards.iter()
                                           .map(|(winning, user)| count_matches(winning, user))
                                           .collect();
    let mut queue: VecDeque<usize> = (0..cards.len()).collect();  // Store indices of the cards

    while let Some(card_index) = queue.pop_front() {
        total_cards += 1;
        let matches = match_counts[card_index];

        // Enqueue indices of subsequent cards based on the number of matches
        for i in 1..=matches {
            if card_index + i < cards.len() {
                queue.push_back(card_index + i);
            }
        }
    }
    //let duration = start.elapsed();
    //println!("task done in {:?}", duration);

    total_cards
}

fn process_cards(cards: &Vec<(Vec<i32>, Vec<i32>)>) -> usize {
    //println!("testing with non-opti function");
    //let start = Instant::now();
    
    let mut total_cards = 0;
    let mut queue: VecDeque<(Vec<i32>, Vec<i32>)> = cards.clone().into_iter().collect();

    while let Some((winning, user)) = queue.pop_front() {
        total_cards += 1;  // Count the processed card
        let matches = count_matches(&winning, &user);

        // Enqueue copies of subsequent cards based on the number of matches
        if matches > 0 {
            let start_index = cards.iter().position(|card| card == &(winning.clone(), user.clone())).unwrap();
            for i in 1..=matches {
                if let Some(card) = cards.get(start_index + i) {
                    queue.push_back(card.clone());
                }
            }
        }
    }
    //let duration = start.elapsed();
    //println!("task done in {:?}", duration);

    total_cards
}

fn main() {
    
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let cards: Vec<(Vec<i32>, Vec<i32>)> = input.lines().map(|line| parse_card(line)).collect();

    let start2 = Instant::now();
    
    println!("Processing Cards w/ opti");
    let total = process_cards_opti(&cards);

    let duration = start2.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!("Total points: {}", total);

    let start = Instant::now();
    
    println!("Processing Cards w/o opti");
    let total = process_cards(&cards);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!("Total points: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_test_input() {
        //println!("testing with non-opti function");
        let start = Instant::now();

        let input = fs::read_to_string("input.txt").expect("Failed to read file");
        let cards: Vec<(Vec<i32>, Vec<i32>)> = input.lines().map(|line| parse_card(line)).collect();

        let total = process_cards(&cards);

        let duration = start.elapsed();
        //println!("task done in {:?}", duration);

        //assert_eq!(duration, std::time::Duration::from_secs(0));
        assert_eq!(total, 5329815);
    }
    #[test]
    fn check_test_input_opti() {
        

        let input = fs::read_to_string("input.txt").expect("Failed to read file");
        let cards: Vec<(Vec<i32>, Vec<i32>)> = input.lines().map(|line| parse_card(line)).collect();

        let total = process_cards_opti(&cards);


        //assert_eq!(duration, std::time::Duration::from_secs(0));
        assert_eq!(total, 5329815);
    }
} 