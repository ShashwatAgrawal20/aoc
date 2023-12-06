use std::collections::HashMap;
use std::fs;

fn part1(content: &str) -> u32 {
    let max: HashMap<&str, u32> = HashMap::from([("red", 12), ("blue", 14), ("green", 13)]);
    let result: Vec<u32> = content
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let game_id: u32 = line
                .split(":")
                .nth(0)
                .and_then(|s| s.split_whitespace().nth(1))
                .and_then(|s| s.parse().ok())
                .unwrap_or_default();
            let valid_lines: Vec<bool> = line
                .split(":")
                .nth(1)
                .unwrap()
                .split(";")
                .map(|game| {
                    game.split(",").all(|dice| {
                        let color = dice.trim().split(" ").nth(1).unwrap();
                        let count: u32 = dice.trim().split(" ").nth(0).unwrap().parse().unwrap();
                        count <= *max.get(color).unwrap_or(&0)
                    })
                })
                .collect();
            let valid = valid_lines.iter().all(|&line| line);
            if valid {
                Some(game_id)
            } else {
                None
            }
        })
        .collect();
    result.iter().sum::<u32>() as u32
}

fn part2(content: &str) -> u32 {
    let mut max: HashMap<&str, u32> = HashMap::new();
    let result: Vec<u32> = content
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let _valid_lines: Vec<_> = line
                .split(":")
                .nth(1)
                .unwrap()
                .split(";")
                .map(|game| {
                    game.split(",").for_each(|dice| {
                        let color = dice.trim().split_whitespace().nth(1).unwrap();
                        let count: u32 = dice
                            .trim()
                            .split_whitespace()
                            .nth(0)
                            .unwrap()
                            .parse()
                            .unwrap();

                        if count > *max.get(color).unwrap_or(&0) {
                            max.insert(color, count);
                        }
                        // println!("{game_id} -> {:?}", max);
                    })
                })
                .collect();

            let game_result = max.get("red").unwrap_or(&1)
                * max.get("green").unwrap_or(&1)
                * max.get("blue").unwrap_or(&1);
            max = HashMap::new();
            Some(game_result)
        })
        .collect();
    result.iter().sum::<u32>()
}

fn main() {
    let content = fs::read_to_string("inputs/day2").expect("Failed to read file");
    let part1_result = part1(&content);
    let part2_result = part2(&content);
    println!("part1 -> {part1_result}\npart2 -> {part2_result}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
             Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
             Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
             Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
             Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, super::part1(input));
    }
    #[test]
    fn part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
             Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
             Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
             Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
             Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, super::part2(input));
    }
}
