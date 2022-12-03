const INPUT_FILE: &str = include_str!("input.txt");
const SCORE_MAP: [[u8; 3]; 3] = [[3, 6, 0], [0, 3, 6], [6, 0, 3]];
const RESULT_MAP: [[u8; 3]; 3] = [[2, 0, 1], [0, 1, 2], [1, 2, 0]];

fn main() {
    println!("{}", part1(parse_input()));
    println!("{}", part2(parse_input()));
}

fn part1(strategies: Vec<(u8, u8)>) -> i32 {
    let mut sum: i32 = 0;
    for strategy in strategies {
        sum += score_tuple(strategy) as i32;
    }
    sum as i32
}

fn part2(strategies: Vec<(u8, u8)>) -> i32 {
    let mut sum: i32 = 0;
    for strategy in strategies {
        sum += score_tuple_2(strategy) as i32;
    }
    sum as i32
}

fn score_tuple_2((a, b): (u8, u8)) -> u8 {
    RESULT_MAP[a as usize][b as usize] + 1 + b * 3
}

fn score_tuple((a, b): (u8, u8)) -> u8 {
    let mut score = b + 1;

    score += SCORE_MAP[a as usize][b as usize];

    score
}

fn parse_input() -> Vec<(u8, u8)> {
    let mut lines = INPUT_FILE.lines();
    let mut input: Vec<(u8, u8)> = Vec::new();

    while let Some(line) = lines.next() {
        let parts: Vec<u8> = line
            .chars()
            .filter(|c| c != &' ')
            .map(|c| {
                if c == 'A' || c == 'X' {
                    0
                } else if c == 'B' || c == 'Y' {
                    1
                } else {
                    2
                }
            })
            .collect();
        let a = parts.get(0).unwrap();
        let b = parts.get(1).unwrap();

        input.push((*a, *b));
    }

    input
}
