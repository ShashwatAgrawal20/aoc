use std::fs;

fn parsing_for_part1(item: &str) -> Vec<usize> {
    let item: Vec<usize> = item
        .split(':')
        .nth(1)
        .expect("Invalid time format")
        .split_whitespace()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().expect("Failed to parse integer"))
        .collect();
    item
}
fn parsing_for_part2(item: &str) -> Vec<usize> {
    let item: Vec<usize> = vec![item
        .split(':')
        .nth(1)
        .expect("Invalid time format")
        .split_whitespace()
        .collect::<String>()
        .parse()
        .expect("Failed to parse integer")];

    // println!("{:?}", item);
    item
}

fn calculate_counts(time: &Vec<usize>, distance: &Vec<usize>) -> Vec<usize> {
    time.iter()
        .zip(distance.iter())
        .map(|(t, d)| {
            (0..*t)
                .map(|j| j * (*t - j))
                .filter(|&my_distance| my_distance > *d)
                .count() as usize
        })
        .collect()
}

fn part1(time: &str, distance: &str) -> usize {
    let time = parsing_for_part1(time);
    let distance = parsing_for_part1(distance);
    let counts = calculate_counts(&time, &distance);
    // println!("{:?}", counts);
    counts.iter().product()
}

fn part2(time: &str, distance: &str) -> usize {
    let time = parsing_for_part2(time);
    let distance = parsing_for_part2(distance);
    let counts = calculate_counts(&time, &distance);
    // println!("{:?}", counts);
    counts.iter().product()
}

fn main() {
    let contents = fs::read_to_string("inputs/day6").expect("Failed to read file");
    let (time, distance) = contents.split_once('\n').unwrap();
    let part1_res = part1(time, distance);
    let part2_res = part2(time, distance);
    println!("part1 -> {part1_res}\npart2 -> {part2_res}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = "\
             Time:      7  15   30\n\
             Distance:  9  40  200";
        let (time, distance) = input.split_once('\n').unwrap();
        assert_eq!(288, super::part1(time, distance));
    }
    #[test]
    fn part2() {
        let input = "\
             Time:      7  15   30\n\
             Distance:  9  40  200";
        let (time, distance) = input.split_once('\n').unwrap();
        assert_eq!(71503, super::part2(time, distance));
    }
}
