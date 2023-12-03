use std::fs::read_to_string;

fn part1(content: &str) -> u32 {
    let mut sum = 0;
    for line in content.lines() {
        let nums: Vec<u32> = line
            .chars()
            .filter_map(|x| x.to_digit(10))
            .collect::<Vec<u32>>();
        if nums.len() == 0 {
            continue;
        }
        sum += nums.first().unwrap() * 10 + nums.last().unwrap();
    }
    sum
}

fn part2(content: &str) -> u32 {
    let mut content = content.to_string();
    let maps = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 10),
    ];
    for (key, value) in maps {
        // crazy replace to avoid overlapping values like eightwothree
        content = content.replace(key, format!("{key}{value}{key}").as_str());
    }
    part1(&content)
}

fn main() {
    let content: String = read_to_string("inputs/day1").expect("Failed to read file");
    let part1 = part1(&content);
    let part2 = part2(&content);
    println!("part1 -> {part1}\npart2 -> {part2}");
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "\
                    1abc2
                    pqr3stu8vwx
                    a1b2c3d4e5f
                    treb7uchet";

        assert_eq!(142, super::part1(input));
    }

    #[test]
    fn part2() {
        let input = "\
                    two1nine
                    eightwothree
                    abcone2threexyz
                    xtwone3four
                    4nineeightseven2
                    zoneight234
                    7pqrstsixteen";
        assert_eq!(281, super::part2(input));
    }
}
