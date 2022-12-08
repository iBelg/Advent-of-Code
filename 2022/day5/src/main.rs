// Read input file with macro
const INPUT: &str = include_str!("input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    let (moves, mut stacks) = read_input();
    for (n, from, to) in moves {
        let from_stack = &mut stacks[(from - 1) as usize];
        // move last n elements to new stack
        let mut new_stack = from_stack.split_off(from_stack.len() - n as usize);
        new_stack.reverse();
        let to_stack = &mut stacks[(to - 1) as usize];
        to_stack.append(&mut new_stack);
    }
    // add last element of each stack to string
    let mut result = String::new();
    for stack in stacks {
        result.push(stack[stack.len() - 1]);
    }
    println!("Result: {}", result);
}

fn part2() {
    let (moves, mut stacks) = read_input();
    for (n, from, to) in moves {
        let from_stack = &mut stacks[(from - 1) as usize];
        // move last n elements to new stack
        let mut new_stack = from_stack.split_off(from_stack.len() - n as usize);
        let to_stack = &mut stacks[(to - 1) as usize];
        to_stack.append(&mut new_stack);
    }
    // add last element of each stack to string
    let mut result = String::new();
    for stack in stacks {
        result.push(stack[stack.len() - 1]);
    }
    println!("Result: {}", result);
}

// read input in reverse order
fn read_input() -> (Vec<(i32, i32, i32)>, Vec<Vec<char>>) {
    let input = INPUT.lines().rev();
    let mut moves: Vec<(i32, i32, i32)> = Vec::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in input {
        if line.starts_with("move") {
            let numbers = line
                .split_whitespace()
                .filter(|s| s.parse::<i32>().is_ok())
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            moves.push((numbers[0], numbers[1], numbers[2]));
        } else if line.contains("[") {
            // Loop over stacks with index
            for (i, stack) in stacks.iter_mut().enumerate() {
                let char_at = line.chars().nth(i * 4 + 1).unwrap();
                if char_at != ' ' {
                    stack.push(char_at);
                }
            }
        } else if is_number(remove_whitespace(line)) {
            let numbers = remove_whitespace(line)
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>();
            for _ in numbers {
                stacks.push(Vec::new());
            }
        }
    }
    moves.reverse();

    (moves, stacks)
}

// remove all whitespaces from string
fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

// check if string only contains numbers
fn is_number(s: String) -> bool {
    s.chars().all(|c| c.is_numeric())
}
