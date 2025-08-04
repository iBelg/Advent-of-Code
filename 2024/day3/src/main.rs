use regex::Regex;
const INPUT_FILE: &str = include_str!("input");

fn main() {
    // part1();
    part2();
}

fn part1() {
    let re = Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();
    let mut mul_instructions = Vec::<MulInstruction>::new();

    for line in INPUT_FILE.lines() {
        for (_, [x, y]) in re.captures_iter(line).map(|c| c.extract()) {
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            mul_instructions.push(MulInstruction { x, y });
        }
    }

    let sum = mul_instructions.iter().map(|i| i.x * i.y).sum::<i32>();
    println!("Sum: {}", sum);
}

fn part2() {
    let re = Regex::new(r"(?<opcode>mul|don't|do)\((?<params>\d{1,3},\d{1,3}|)\)").unwrap();
    let mut instructions = Vec::<Instruction>::new();

    for line in INPUT_FILE.lines() {
        for (_, [opcode, params]) in re.captures_iter(line).map(|c| c.extract()) {
            instructions.push(Instruction { opcode: opcode.to_string(), params: params.to_string() });
        }
    }

    let mut skip = false;
    let mut sum = 0;

    for instruction in instructions {
        if instruction.opcode == "mul" && !skip {
            let mut params = instruction.params.split(",");
            let x = params.nth(0).unwrap().parse::<i32>().unwrap();
            let y = params.nth(0).unwrap().parse::<i32>().unwrap();
            sum += x * y;
        } else if instruction.opcode == "don't" {
            skip = true;
        } else if instruction.opcode == "do" {
            skip = false;
        }
    }

    println!("Sum: {}", sum);
}

#[derive(Debug)]
struct MulInstruction {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Instruction {
    opcode: String,
    params: String,
}
