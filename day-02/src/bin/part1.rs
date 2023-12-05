use std::fs;

struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}

fn parse_cube_set(set: &str) -> CubeSet {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for part in set.split(", ") {
        let parts: Vec<&str> = part.split_whitespace().collect();
        let count = parts[0].parse::<usize>().unwrap();
        match parts[1] {
            "red" => red = count,
            "green" => green = count,
            "blue" => blue = count,
            _ => {}
        }
    }

    CubeSet { red, green, blue }
}

fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split(": ").collect();
        let id = parts[0].replace("Game ", "").parse::<usize>().unwrap();
        let sets = parts[1].split("; ")
                          .map(|set| parse_cube_set(set))
                          .collect();
        Game { id, sets }
    }).collect()
}

fn is_game_possible(game: &Game, max_red: usize, max_green: usize, max_blue: usize) -> bool {
    game.sets.iter().all(|set| set.red <= max_red && set.green <= max_green && set.blue <= max_blue)
}

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read file");
    let games = parse_input(&input);

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let sum_of_ids: usize = games.iter()
        .filter(|&game| is_game_possible(game, max_red, max_green, max_blue))
        .map(|game| game.id)
        .sum();

    println!("Sum of IDs: {}", sum_of_ids);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_puzzle_solution() {
        let test_input = fs::read_to_string("inputtest.txt").expect("Failed to read test file");
        let games = parse_input(&test_input);

        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        let sum_of_ids: usize = games.iter()
            .filter(|&game| is_game_possible(game, max_red, max_green, max_blue))
            .map(|game| game.id)
            .sum();

        assert_eq!(sum_of_ids, 8, "The sum of IDs of possible games should be 8");
    }
}