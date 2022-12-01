const INPUT_FILE: &str = include_str!("input.txt");

fn main() {
    if let Some(elf) = part1() {
        println!("Part 1:\n{}", elf.total_calories());
    } else {
        println!("Part 1: Failure!");
    }

    if let Some(elfs) = part2() {
        println!(
            "Part 2:\n{}",
            elfs.iter()
                .fold(0i32, |accum, item| accum + item.total_calories())
        );
    } else {
        println!("Part 2: Failure!");
    }
}

fn part1() -> Option<Elf> {
    let elfs = parse_input();

    if let Some(juiciest_elf) = elfs.iter().max() {
        Some(juiciest_elf.clone())
    } else {
        None
    }
}

fn part2() -> Option<Vec<Elf>> {
    let mut elfs = parse_input();
    elfs.sort();
    elfs.reverse();

    Some(elfs[0..3].to_vec())
}

fn parse_input() -> Vec<Elf> {
    let mut lines = INPUT_FILE.lines();
    let mut elfs: Vec<Elf> = Vec::new();
    let mut items: Vec<i32> = Vec::new();

    while let Some(line) = lines.next() {
        if let Ok(number) = line.parse::<i32>() {
            items.push(number);
        } else {
            elfs.push(Elf {
                calories: items.clone(),
            });
            items.clear();
        }
    }

    elfs.push(Elf {
        calories: items.clone(),
    });
    items.clear();

    elfs
}

// Elf stuff

#[derive(Debug, Clone)]
struct Elf {
    calories: Vec<i32>,
}

impl Elf {
    fn total_calories(&self) -> i32 {
        self.calories.iter().sum()
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_calories().cmp(&other.total_calories())
    }
}

impl Eq for Elf {}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.total_calories() == other.total_calories()
    }
}
