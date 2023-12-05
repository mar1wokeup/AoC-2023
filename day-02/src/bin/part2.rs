use std::fs;

struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}
#[allow(dead_code)]
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

fn min_cubes_and_power(game: &Game) -> (usize, usize, usize, usize) {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    for set in &game.sets {
        min_red = std::cmp::max(min_red, set.red);
        min_green = std::cmp::max(min_green, set.green);
        min_blue = std::cmp::max(min_blue, set.blue);
    }

    let power = min_red * min_green * min_blue;
    (min_red, min_green, min_blue, power)
}
#[allow(dead_code)]
fn is_game_possible(game: &Game, max_red: usize, max_green: usize, max_blue: usize) -> bool {
    game.sets.iter().all(|set| set.red <= max_red && set.green <= max_green && set.blue <= max_blue)
}
#[allow(dead_code)]
fn compute_total_power(input: &str) -> usize {
    let games = parse_input(input);
    games.iter()
         .map(|game| min_cubes_and_power(game).3)
         .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_power() {
        let input = fs::read_to_string("inputtest.txt")
                    .expect("Failed to read test input file");
        let total_power = compute_total_power(&input);
        assert_eq!(total_power, 2286);
    }
}

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read file");
    let games = parse_input(&input);

    let total_power: usize = games.iter()
        .map(|game| min_cubes_and_power(game).3)
        .sum();

    println!("Total power of minimum sets: {}", total_power);
}