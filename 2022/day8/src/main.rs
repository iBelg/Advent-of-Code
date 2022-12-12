const INPUT: &str = include_str!("input.txt");

fn main() {
    // part1();
    part2();
}

fn part1() {
    let forest = process_input();
    let mut count = 0;
    for y in 1..forest.height - 1 {
        for x in 1..forest.width - 1 {
            if forest.is_visible(x, y) {
                count += 1;
            }
        }
    }
    println!("Visible trees: {}", count + forest.circumference());
}

fn part2() {
    let forest = process_input();
    let (x, y) = (2, 3);
    println!("{} {} {}", x, y, forest.get(x, y));
    // find highest scenery score
    let mut highest_score = 0;
    for y in 1..forest.height - 1 {
        for x in 1..forest.width - 1 {
            let score = forest.scenic_score(x, y);
            if score > highest_score {
                highest_score = score;
            }
        }
    }
    println!("Highest score: {}", highest_score);
}

fn process_input() -> Forest {
    let mut forest = Forest {
        trees: Vec::new(),
        width: 0,
        height: 0,
    };

    for line in INPUT.lines() {
        forest.width = line.len();
        forest.height += 1;

        for c in line.chars() {
            forest.trees.push(c.to_digit(10).unwrap() as u8);
        }
    }

    forest
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Forest {
    trees: Vec<u8>,
    width: usize,
    height: usize,
}

impl Forest {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.trees[y * self.width + x]
    }

    fn is_on_edge(&self, x: usize, y: usize) -> bool {
        x == self.width - 1 || y == self.height - 1 || x == 0 || y == 0
    }

    fn circumference(&self) -> usize {
        self.width * 2 + self.height * 2 - 4
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        if self.is_on_edge(x, y) {
            return true;
        }

        for direction in vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if self.is_visible_from(x, y, direction) {
                return true;
            }
        }

        false
    }

    // multiply the viewing distance in each direction
    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let mut score = 1;
        for direction in vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            score *= self.viewing_distance(x, y, direction);
        }
        score
    }

    fn viewing_distance(&self, x: usize, y: usize, direction: Direction) -> usize {
        let tree_height = self.get(x, y);
        match direction {
            Direction::Up => {
                for i in 1..=y {
                    if self.get(x, y - i) >= tree_height {
                        return i;
                    }
                }
                return y;
            }
            Direction::Down => {
                for i in 1..self.height - y {
                    if self.get(x, y + i) >= tree_height {
                        return i;
                    }
                }
                return self.height - y - 1;
            }
            Direction::Left => {
                for i in 1..=x {
                    if self.get(x - i, y) >= tree_height {
                        return i;
                    }
                }
                return x;
            }
            Direction::Right => {
                for i in 1..self.width - x {
                    if self.get(x + i, y) >= tree_height {
                        return i;
                    }
                }
                return self.width - x - 1;
            }
        }
    }

    fn is_visible_from(&self, x: usize, y: usize, direction: Direction) -> bool {
        let tree_height = self.get(x, y);
        match direction {
            Direction::Up => {
                for i in 1..=y {
                    if self.get(x, y - i) >= tree_height {
                        return false;
                    }
                }
            }
            Direction::Down => {
                for i in 1..self.height - y {
                    if self.get(x, y + i) >= tree_height {
                        return false;
                    }
                }
            }
            Direction::Left => {
                for i in 1..=x {
                    if self.get(x - i, y) >= tree_height {
                        return false;
                    }
                }
            }
            Direction::Right => {
                for i in 1..self.width - x {
                    if self.get(x + i, y) >= tree_height {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn surrounding_trees(&self, x: usize, y: usize) -> Vec<u8> {
        let mut trees = Vec::new();

        if x > 0 {
            trees.push(self.get(x - 1, y));
        }
        if x < self.width - 1 {
            trees.push(self.get(x + 1, y));
        }
        if y > 0 {
            trees.push(self.get(x, y - 1));
        }
        if y < self.height - 1 {
            trees.push(self.get(x, y + 1));
        }

        trees
    }
}
