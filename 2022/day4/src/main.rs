// Read input from file with macro
const FILE_INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Hello, world!");
    println!("Part 1: {}", part1(FILE_INPUT));
    println!("Part 2: {}", part2(FILE_INPUT));
}

fn part1(input: &str) -> i32 {
    let mut amount_overlapped = 0;

    let input = process_input(input);
    for i in input {
        let split = split_on_comma(i);
        let range = create_range(split[0]);
        let other = create_range(split[1]);
        let contains = range_contains(&range, &other) || range_contains(&other, &range);
        if contains {
            amount_overlapped += 1;
        }
    }

    amount_overlapped
}

fn part2(input: &str) -> i32 {
    let mut amount_overlapped = 0;

    let input = process_input(input);
    for i in input {
        let split = split_on_comma(i);
        let range = create_range(split[0]);
        let other = create_range(split[1]);
        let contains = range_overlaps(&range, &other) || range_overlaps(&other, &range);
        if contains {
            amount_overlapped += 1;
        }
    }

    amount_overlapped
}

// check if range overlaps with other range
fn range_overlaps(range: &Vec<i32>, other: &Vec<i32>) -> bool {
    for i in range {
        if other.contains(i) {
            return true;
        }
    }

    false
}

// Check if range fully contains other range
fn range_contains(range: &Vec<i32>, other: &Vec<i32>) -> bool {
    let mut contains = true;

    for i in other {
        if !range.contains(i) {
            contains = false;
            break;
        }
    }

    contains
}

// Create range from numbers
fn create_range(input: &str) -> Vec<i32> {
    let mut range: Vec<i32> = Vec::new();
    let mut split = input.split("-");
    let start = split.next().unwrap().parse::<i32>().unwrap();
    let end = split.next().unwrap().parse::<i32>().unwrap();

    for i in start..=end {
        range.push(i);
    }

    range
}

// Split string on comma
fn split_on_comma(input: &str) -> Vec<&str> {
    input.split(",").collect()
}

// take input and return a vector of strings
fn process_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}
