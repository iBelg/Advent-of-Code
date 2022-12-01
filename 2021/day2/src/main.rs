use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::io::BufRead;

fn main() {
    if let Ok(commands) = read_input() {
        println!("Result for part1: {}", part1(&commands));
        println!("Result for part2: {}", part2(&commands));
    }
}

fn read_input() -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let mut commands: Vec<String> = Vec::new();
    for line in buf_reader.lines() {
        if let Ok(action) = line {
            commands.push(action);
        }
    }
    Ok(commands)
}

fn part1(commands: &Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for command in commands {
        let mut split = command.split_whitespace();
        let move_command = split.next().unwrap();
        let distance = split.next().unwrap().parse::<i32>().unwrap();
        match move_command {
            "forward" => horizontal += distance,
            "up" => depth -= distance,
            "down" => depth += distance,
            _ => println!("Unknown command: {} {}", move_command, distance)
        }
    }

    horizontal * depth
}

fn part2(commands: &Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        let mut split = command.split_whitespace();
        let move_command = split.next().unwrap();
        let distance = split.next().unwrap().parse::<i32>().unwrap();
        match move_command {
            "forward" => {
                horizontal += distance;
                depth += aim * distance;
            },
            "up" => aim -= distance,
            "down" => aim += distance,
            _ => println!("Unknown command: {} {}", move_command, distance)
        }
    }

    horizontal * depth
}
