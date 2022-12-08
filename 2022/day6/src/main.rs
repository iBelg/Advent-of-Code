const INPUT: &str = include_str!("input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    let data = read_input();
    let index = data.find_first_unique_sequence_n(4).unwrap();
    println!("Part 1: {}", index);
}

fn part2() {
    let data = read_input();
    let index = data.find_first_unique_sequence_n(14).unwrap();
    println!("Part 1: {}", index);
}

fn read_input() -> DataStream {
    let mut data = Vec::new();
    for line in INPUT.lines() {
        for c in line.chars() {
            data.push(c);
        }
    }
    DataStream { data }
}

struct DataStream {
    data: Vec<char>,
}

impl DataStream {
    // Get the index of the first sequence of n characters that are all unique
    fn find_first_unique_sequence_n(&self, n: usize) -> Option<usize> {
        let mut index = 0;
        let mut unique = true;
        while index < self.data.len() - n {
            for i in 0..n {
                for j in 0..n {
                    if i != j && self.data[index + i] == self.data[index + j] {
                        unique = false;
                        break;
                    }
                }
                if !unique {
                    break;
                }
            }
            if unique {
                return Some(index + n);
            }
            unique = true;
            index += 1;
        }
        None
    }
}
