const INPUT_FILE: &str = include_str!("input");

fn main() {
    let input = parse_input(INPUT_FILE);
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<Report>) {
    let mut safe_reports = 0;
    for report in input {
        if report.is_valid(false) {
            safe_reports += 1;
        }
    }
    println!("Safe reports: {}", safe_reports);
}

fn part2(input: &Vec<Report>) {
    let mut safe_reports = 0;
    for report in input {
        if report.is_valid(true) {
            safe_reports += 1;
        }
    }
    println!("Safe reports: {}", safe_reports);
}

fn parse_input(input: &str) -> Vec<Report> {
    let lines = input.lines();
    let mut reports = Vec::<Report>::new();

    for line in lines {
        let mut report = Report::new();
        for c in line.split(" ") {
            report.levels.push(c.parse::<i32>().unwrap());
        }
        reports.push(report);
    }

    reports
}

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn new() -> Self {
        Self {
            levels: Vec::new(),
        }
    }   

    fn is_valid(&self, dampening: bool) -> bool {
        if Report::check_levels(&self.levels) {
            return true;
        } else if dampening {
            // go over every variation where you drop one level, until one succeeds or all fail
            for i in 0..self.levels.len() {
                let mut levels = self.levels.clone();
                levels.remove(i);
                let mut report = Report::new();
                report.levels = levels;
                if report.is_valid(false) {
                    return true;
                }
            }
            return false;
        }
        
        false
    }

    fn check_levels(levels: &Vec<i32>) -> bool {
        let mut direction = None;

        for i in 1..levels.len() {
            let previous = levels[i - 1];
            let current = levels[i];
            
            let current_direction = Report::check_direction(previous, current);

            if current_direction.is_none() {
                return false;
            }

            if direction.is_none() {
                direction = current_direction;
            } else if direction != current_direction {
                return false;
            }
        }    

        direction.is_some()
    }

    fn check_direction(previous: i32, current: i32) -> Option<Direction> {
        let calculated_direction = previous - current;
        let current_direction = if calculated_direction.abs() > 3 {
            None
        } else if calculated_direction > 0 {
            Some(Direction::Increasing)
        } else if calculated_direction < 0 {
            Some(Direction::Decreasing)
        } else {
            None
        };

        current_direction
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Increasing,
    Decreasing,
}