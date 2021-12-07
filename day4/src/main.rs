const INPUT_FILE: &str = include_str!("input.txt");
const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = 5;

fn main() {
    if let Ok((numbers, bingo_boards)) = parse_input() {
        println!("Part 1: {}", part1(&numbers, &bingo_boards).unwrap());
        println!("Part 2: {}", part2(&numbers, &bingo_boards).unwrap());
    }
}

fn part1(numbers: &Vec<i32>, bingo_boards: &Vec<BingoBoard>) -> Result<i32, ()> {
    let mut boards = bingo_boards.clone();
    for number in numbers.iter() {
        for board in boards.iter_mut() {
            board.mark(*number);
            if board.check() {
                return Ok(board.score() * number);
            }
        }
    }
    Err(())
}

fn part2(numbers: &Vec<i32>, bingo_boards: &Vec<BingoBoard>) -> Result<i32, ()> {
    let mut boards = bingo_boards.clone();
    let mut last_score: Option<i32> = None;
    for number in numbers.iter() {
        for board in boards.iter_mut() {
            board.mark(*number);
            if board.check() {
                last_score = Some(board.score() * number);
            }
        }
        boards = boards.into_iter().filter(|b| !b.check()).collect();
    }
    if let Some(some_last_score) = last_score {
        return Ok(some_last_score);
    }
    Err(())
}

fn parse_input() -> Result<(Vec<i32>, Vec<BingoBoard>), ()> {
    let mut numbers: Vec<i32> = Vec::new();
    let mut lines = INPUT_FILE.lines();
    if let Some(random_numbers) = lines.next() {
        numbers =  random_numbers.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    }
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut bingo_board: Option<BingoBoard> = None;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            if let Some(mut some_board) = bingo_board {
                boards.push(some_board);
            }
            bingo_board = Some(BingoBoard::new(BOARD_WIDTH, BOARD_HEIGHT));
            continue;
        }
        let numbers = line.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if let Some(some_board) = &mut bingo_board {
            some_board.board.push(numbers);
        }
    }
    boards.push(bingo_board.unwrap());
    if numbers.len() > 0 && boards.len() > 0 {
        return Ok((numbers, boards))
    }
    Err(())
}

#[derive(Debug, Clone)]
struct BingoBoard {
    width: usize,
    height: usize,
    board: Vec<Vec<i32>>,
    marked: Vec<Vec<bool>>,
}

impl BingoBoard {
    fn new(width: usize, height: usize) -> Self {
        BingoBoard { width, height, board: Vec::new(), marked: vec![vec![false; width]; height] }
    }
    
    fn check(&self) -> bool {
        let mut row_sums: Vec<i32> = vec![0; self.height];
        let mut col_sums: Vec<i32> = vec![0; self.width];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.marked[y][x] {
                    row_sums[y] += 1;
                    col_sums[x] += 1;
                }
            }
        }
        row_sums.iter().any(|&n| n == self.height.try_into().unwrap()) || col_sums.iter().any(|&n| n == self.width.try_into().unwrap())
    } 

    fn score(&self) -> i32 {
        let mut sum = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if !self.marked[y][x] {
                    sum += self.board[y][x];
                }
            }
        }
        sum
    }

    fn mark(&mut self, number: i32) -> bool {
        for (y, row) in self.board.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value == number {
                    self.marked[y][x] = true;
                    return true;
                }
            }
        }
        false
    }
}