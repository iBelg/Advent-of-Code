// Read input file with macro
const INPUT_FILE: &str = include_str!("input.txt");

fn main() {
    let input = read_input();
    println!("{:?}", input);
    let result = part1(&input);
    println!("Result: {}", result);
    let result = part2(&input);
    println!("Result: {}", result);
}

fn part2(input: &Vec<String>) -> usize {
    let mut sum = 0;
    for group in input.chunks(3) {
        let letter = shared_letter_in_group(group);
        let converted_letter = letter_to_number(letter);
        println!("{} -> {}", group[0], letter);
        sum += converted_letter;
    }
    sum
}

fn shared_letter_in_group(group: &[String]) -> char {
    let mut shared_letter = ' ';
    for letter in group[0].chars() {
        if group[1].contains(letter) && group[2].contains(letter) {
            shared_letter = letter;
        }
    }
    shared_letter
}

fn part1(input: &Vec<String>) -> usize {
    let mut sum = 0;
    for line in input {
        let (left, right) = split_in_half(line);
        let letter = shared_letter(&left, &right);
        let converted_letter = letter_to_number(letter);
        sum += converted_letter;
    }
    sum
}

fn letter_to_number(letter: char) -> usize {
    match letter {
        'a'..='z' => letter as usize - 96,
        'A'..='Z' => letter as usize - 38,
        _ => 0,
    }
}

// Find the letter contained in both strings
fn shared_letter(left: &str, right: &str) -> char {
    for letter in left.chars() {
        if right.contains(letter) {
            return letter;
        }
    }
    ' '
}

fn split_in_half(input: &str) -> (String, String) {
    let half = input.len() / 2;
    let (left, right) = input.split_at(half);
    (left.to_string(), right.to_string())
}

fn read_input() -> Vec<String> {
    INPUT_FILE.lines().map(|line| line.to_string()).collect()
}
