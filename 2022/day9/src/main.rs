use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    // part1();
    part2();
}

fn part1() {
    let moves = read_input();
    let mut bridge = RopeBridge::new(1);
    for m in moves {
        bridge.move_head(&m);
    }
    println!("{:?}", bridge.tail_unique_coordinates.len());
}

fn part2() {
    let moves = read_input();
    let mut bridge = RopeBridge::new(9);
    for m in moves {
        bridge.move_head(&m);
    }
    println!("{:?}", bridge.tail_unique_coordinates.len());
}

fn read_input() -> Vec<Move> {
    INPUT
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let direction = match direction {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction"),
            };
            let distance = distance.trim().parse().unwrap();
            Move(direction, distance)
        })
        .collect()
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move(Direction, i32);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Coordinates(i32, i32);

struct RopeBridge {
    head_coordinates: Coordinates,
    tail_coordinates: Vec<Coordinates>,
    tail_unique_coordinates: HashSet<Coordinates>,
    knots: usize,
}

impl RopeBridge {
    fn new(knots: usize) -> Self {
        RopeBridge {
            head_coordinates: Coordinates(0, 0),
            tail_coordinates: vec![Coordinates(0, 0); knots],
            tail_unique_coordinates: HashSet::from([Coordinates(0, 0)]),
            knots,
        }
    }

    fn move_head(&mut self, m: &Move) {
        for _ in 1..=m.1 {
            match m.0 {
                Direction::Up => self.head_coordinates.1 += 1,
                Direction::Down => self.head_coordinates.1 -= 1,
                Direction::Left => self.head_coordinates.0 -= 1,
                Direction::Right => self.head_coordinates.0 += 1,
            }
            for knot in 0..self.knots {
                let first_coordinate = if knot == 0 {
                    self.head_coordinates.clone()
                } else {
                    self.tail_coordinates[knot - 1].clone()
                };
                let second_coordinate = self.tail_coordinates[knot].clone();
                if !self.is_next_to_each_other(&first_coordinate, &second_coordinate) {
                    let moves =
                        self.get_moves_to_move_a_to_b(&first_coordinate, &second_coordinate);
                    for m in moves {
                        self.move_tail_at_index(knot, &m);
                    }
                    if knot == self.knots - 1 {
                        self.tail_unique_coordinates
                            .insert(self.tail_coordinates[knot].clone());
                    }
                } else {
                    break;
                }
            }
        }
    }

    fn move_tail_at_index(&mut self, index: usize, m: &Move) {
        match m.0 {
            Direction::Up => self.tail_coordinates[index].1 += 1,
            Direction::Down => self.tail_coordinates[index].1 -= 1,
            Direction::Left => self.tail_coordinates[index].0 -= 1,
            Direction::Right => self.tail_coordinates[index].0 += 1,
        }
    }

    // return the moves needed to move coordinate a next to coordinate b
    fn get_moves_to_move_a_to_b(&self, a: &Coordinates, b: &Coordinates) -> Vec<Move> {
        let mut moves = Vec::new();
        let x_diff = a.0 - b.0;
        let y_diff = a.1 - b.1;
        if x_diff > 0 {
            moves.push(Move(Direction::Right, x_diff));
        } else if x_diff < 0 {
            moves.push(Move(Direction::Left, x_diff.abs()));
        }
        if y_diff > 0 {
            moves.push(Move(Direction::Up, y_diff));
        } else if y_diff < 0 {
            moves.push(Move(Direction::Down, y_diff.abs()));
        }
        moves
    }

    // checks if two coordinates are next to each other, including diagonally
    fn is_next_to_each_other(&self, a: &Coordinates, b: &Coordinates) -> bool {
        let x_diff = (a.0 - b.0).abs();
        let y_diff = (a.1 - b.1).abs();
        x_diff <= 1 && y_diff <= 1
    }

    fn visualize_coordinates(&self) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for c in self.tail_unique_coordinates.iter() {
            if c.0 < min_x {
                min_x = c.0;
            }
            if c.0 > max_x {
                max_x = c.0;
            }
            if c.1 < min_y {
                min_y = c.1;
            }
            if c.1 > max_y {
                max_y = c.1;
            }
        }
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let c = Coordinates(x, y);
                if self.tail_unique_coordinates.contains(&c) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
