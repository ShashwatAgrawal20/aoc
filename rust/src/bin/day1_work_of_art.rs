use std::fs;

fn main() {
    let contents =
        fs::read_to_string("inputs/day1_input").expect("Should have been able to read the file");

    let mut max = 0;
    let mut max_vec = Vec::new();
    for group in contents.split("\n\n") {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<i32>().unwrap();
            sum += value;
        }
        if sum > max {
            max = sum;
        }
        max_vec.push(sum);
    }
    max_vec.sort_unstable_by(|a, b| b.cmp(a));
    println!("Part 1 Shit:- {}", max);
    let top_three = &max_vec[..3];
    let total: i32 = top_three.iter().copied().sum();
    println!("Part 2 Shit:- {}", total);
}
