#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::VecDeque;

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");
lazy_static! {
    static ref OPERATION_REGEX: Regex = Regex::new(r"Operation: new = (.+) (\+|\*) (.+)").unwrap();
    static ref GET_DIGIT_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

fn main() {
    // part1();
    part2();
}

fn part1() {
    let mut monkeys = read_input();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let item = match monkeys[i].items.pop_front() {
                    Some(i) => i,
                    None => break,
                };
                monkeys[i].inspections += 1;
                let mut worry_level = monkeys[i].operation(item);
                worry_level /= 3;
                let throw_to = if worry_level % monkeys[i].divisible_by == 0 {
                    monkeys[i].test_result.0
                } else {
                    monkeys[i].test_result.1
                };
                monkeys[throw_to as usize].items.push_back(worry_level);
            }
        }
    }

    // find the two monkeys with the most inspections
    let mut inspection_counters: Vec<i64> = Vec::new();
    for monkey in monkeys {
        inspection_counters.push(monkey.inspections);
    }
    inspection_counters.sort();
    inspection_counters.reverse();
    println!(
        "Part 1: {}",
        inspection_counters[0] * inspection_counters[1]
    );
}

fn part2() {
    let mut monkeys = read_input();
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let item = match monkeys[i].items.pop_front() {
                    Some(i) => i,
                    None => break,
                };
                monkeys[i].inspections += 1;
                let worry_level = monkeys[i].operation(item);
                let throw_to = if worry_level % monkeys[i].divisible_by == 0 {
                    monkeys[i].test_result.0
                } else {
                    monkeys[i].test_result.1
                };
                monkeys[throw_to as usize].items.push_back(worry_level);
            }
        }
    }

    // find the two monkeys with the most inspections
    let mut inspection_counters: Vec<i64> = Vec::new();
    for monkey in monkeys {
        inspection_counters.push(monkey.inspections);
    }
    inspection_counters.sort();
    inspection_counters.reverse();
    println!(
        "Part 2: {}",
        inspection_counters[0] * inspection_counters[1]
    );
}

fn read_input() -> Vec<Monkey> {
    // loop over input in chunks of 7
    let lines: Vec<&str> = INPUT.lines().collect();
    let mut monkeys = Vec::new();
    for line in lines.chunks(7) {
        monkeys.push(create_monkey(&line.to_vec()));
    }

    let supermodulo = monkeys.iter().fold(1, |acc, m| acc * m.divisible_by);

    for monkey in &mut monkeys {
        monkey.supermodulo = supermodulo;
    }

    monkeys
}

fn create_monkey(lines: &Vec<&str>) -> Monkey {
    let mut items: VecDeque<i64> = VecDeque::new();
    let mut counter = 0;
    let mut operation = Operation::Add;
    let mut operation_values = [OperationValue::Old, OperationValue::Old];
    let mut divisible_by: i64 = 0;
    let mut test_result = (0, 0);
    for line in lines {
        match counter {
            1 => {
                // Starting items
                items = GET_DIGIT_REGEX
                    .captures_iter(line)
                    .map(|c| c.get(0).unwrap().as_str().parse::<i64>().unwrap())
                    .collect();
            }
            2 => {
                // Operation
                let captures = OPERATION_REGEX.captures(line);
                // get the two values and operation from the captures
                operation = match captures.as_ref().unwrap().get(2).unwrap().as_str() {
                    "+" => Operation::Add,
                    "*" => Operation::Multiply,
                    _ => panic!("Unknown operation"),
                };
                operation_values[0] = match captures.as_ref().unwrap().get(1).unwrap().as_str() {
                    "old" => OperationValue::Old,
                    _ => OperationValue::Number(
                        captures
                            .as_ref()
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str()
                            .parse::<i64>()
                            .unwrap(),
                    ),
                };
                operation_values[1] = match captures.as_ref().unwrap().get(3).unwrap().as_str() {
                    "old" => OperationValue::Old,
                    _ => OperationValue::Number(
                        captures
                            .as_ref()
                            .unwrap()
                            .get(3)
                            .unwrap()
                            .as_str()
                            .parse::<i64>()
                            .unwrap(),
                    ),
                };
            }
            3 => {
                // Divisible by
                let captures = GET_DIGIT_REGEX.captures(line);
                divisible_by = captures
                    .as_ref()
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap();
            }
            4 => {
                // Test result: true
                let captures = GET_DIGIT_REGEX.captures(line);
                test_result.0 = captures
                    .as_ref()
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap();
            }
            5 => {
                // Test result: false
                let captures = GET_DIGIT_REGEX.captures(line);
                test_result.1 = captures
                    .as_ref()
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap();
            }
            _ => {}
        }

        counter += 1;
    }
    Monkey {
        items,
        inspections: 0,
        operation,
        operation_values,
        divisible_by,
        test_result,
        supermodulo: 0,
    }
}

#[derive(Debug)]
enum OperationValue {
    Old,
    Number(i64),
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    inspections: i64,
    operation: Operation,
    operation_values: [OperationValue; 2],
    divisible_by: i64,
    test_result: (i64, i64),
    supermodulo: i64,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: VecDeque::new(),
            inspections: 0,
            operation: Operation::Add,
            operation_values: [OperationValue::Old, OperationValue::Old],
            divisible_by: 0,
            test_result: (0, 0),
            supermodulo: 0,
        }
    }

    fn operation(&self, value_for_old: i64) -> i64 {
        let value1 = match self.operation_values[0] {
            OperationValue::Old => value_for_old,
            OperationValue::Number(n) => n,
        };
        let value2 = match self.operation_values[1] {
            OperationValue::Old => value_for_old,
            OperationValue::Number(n) => n,
        };

        let result = match self.operation {
            Operation::Add => (value1 % self.supermodulo) + (value2 % self.supermodulo),
            Operation::Multiply => (value1 % self.supermodulo) * (value2 % self.supermodulo),
        };

        result
    }
}

struct KeepAwayGame {
    monkeys: Vec<Monkey>,
}
