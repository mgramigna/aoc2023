fn part_one() -> usize {
    let lines: Vec<&str> = include_str!("../inputs/1-1.txt").lines().collect();

    let sum: usize = lines
        .iter()
        .map(|line| {
            let first_digit_char = line
                .chars()
                .find(|c| c.is_numeric())
                .expect("Could not locate a numeric character in line");

            let last_digit_char = line
                .chars()
                .rfind(|c| c.is_numeric())
                .expect("Could not locate a numeric character in line");

            let digit_string = format!("{}{}", first_digit_char, last_digit_char);

            digit_string
                .parse::<usize>()
                .expect("Could not parse int from string")
        })
        .sum();

    sum
}

// Too tired to do this rn
fn part_two() -> usize {
    0
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
