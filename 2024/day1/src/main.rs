use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("input");
const SEPARATOR: &str = "   ";

fn main() {
    //print the parsed input
    let parsed_input = parse_input();
    println!("Part1: {:?}", part1(parsed_input.0, parsed_input.1));
    let parsed_input = parse_input();
    println!("Part2: {:?}", part2(parsed_input.0, parsed_input.1));


}

fn part1(first_list: Vec<i32>, second_list: Vec<i32>) -> i32 {
    let mut result: Vec<i32> = Vec::new();
    for i in 0..first_list.len() {
        result.push((first_list[i] - second_list[i]).abs());
    }
    
    // sum the result
    result.iter().sum()
}

fn part2(first_list: Vec<i32>, second_list: Vec<i32>) -> i32 {
    let mut number_occurrences: HashMap<i32, i32> = HashMap::new();
    
    second_list.iter().for_each(|number| {
        number_occurrences.entry(*number).and_modify(|count| *count += 1).or_insert(1);
    });

    
    let sum = first_list.iter().map(|x| {
        let count = number_occurrences.get(x).unwrap_or(&0);
        x * count
    }).sum();

    sum
}

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let mut lines = INPUT_FILE.lines();
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();


    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let line: Vec<&str> = line.split(SEPARATOR).collect();
        if let Ok(number) = line[0].parse() {
            first_list.push(number);
        }
        if let Ok(number) = line[1].parse() {
            second_list.push(number);
        }
    }
    
    // sort the lists
    first_list.sort();
    second_list.sort();


    (first_list, second_list)
}