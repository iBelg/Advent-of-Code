use std::env;
use std::fs::File;
use std::process;
use std::error::Error;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Failed to parse arguments: {}", error);
        process::exit(1)
    });

    let numbers: Vec<i32> = read_input(config).unwrap_or_else(|error| {
        println!("Failed to read lines: {}", error);
        process::exit(1)
    });

    let result: i32 = part1(&numbers).unwrap_or_else(|error| {
        println!("Application error: {}", error);
        process::exit(1);
    });

    println!("Result part 1: {}", result);

    let result: i32 = part2(&numbers).unwrap_or_else(|error| {
        println!("Application error: {}", error);
        process::exit(1);
    });
    println!("Result part 2: {}", result);
}

fn read_input(config: Config) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(config.filename)?;
    let buf_reader = BufReader::new(file);
    let mut numbers: Vec<i32> = Vec::new();
    for line in buf_reader.lines() {
        if let Ok(string_number) = line {
            let number = string_number.parse::<i32>().unwrap();
            numbers.push(number);
        }
    }
    Ok(numbers)
}

fn part1(numbers: &Vec<i32>) -> Result<i32, Box<dyn Error>> {
    let mut previous_number: Option<i32> = None;
    let mut increases: i32 = 0;
    for number in numbers {
        if previous_number.is_some() && number > &previous_number.unwrap() {
            increases += 1;
        }
        previous_number = Some(*number);
    }
    Ok(increases)
}

fn part2(numbers: &Vec<i32>) -> Result<i32, Box<dyn Error>> {
    let sliding_window = 3;
    let mut sliding_sums: Vec<SlidingSum> = Vec::new();
    for number in numbers {
        sliding_sums.push(SlidingSum::new(sliding_window));
            sliding_sums = sliding_sums.into_iter()
                .map(|mut s| { if !s.valid() { s.values.push(*number) } s })
                .collect();
    }
    let sums: Vec<i32> = sliding_sums.iter().filter(|s| s.valid()).map(|s| s.sum()).collect();
    
    Ok(part1(&sums)?)
}

#[derive(Debug)]
struct SlidingSum {
    int_amount: i32,
    values: Vec<i32>,
}

impl SlidingSum {
    fn new(int_amount: i32) -> SlidingSum {
        SlidingSum{ int_amount, values: Vec::new() }
    }

    fn valid(self: &Self) -> bool {
        self.values.len() == self.int_amount.try_into().unwrap()
    }

    fn sum(self: &Self) -> i32 {
        self.values.iter().sum()
    }
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 1 {
            return Err("Pass filename as argument");
        }
        let filename = args[0].clone();

        Ok(Config { filename })
    }
}
