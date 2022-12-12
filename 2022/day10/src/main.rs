const INPUT: &str = include_str!("input.txt");

fn main() {
    // part1();
    part2();
}

fn part1() {
    let mut cpu = CPU::new();
    let commands = read_input();
    let mut signal_strength_sum = 0;
    for command in commands {
        cpu.process(command);
        while cpu.temp.is_some() {
            cpu.cycle();
            if (cpu.cycle_count - 20) % 40 == 0 && cpu.temp.is_some() {
                signal_strength_sum += cpu.signal_strength();
            }
        }
    }

    println!("Part 1: {}", signal_strength_sum);
}

fn part2() {
    let mut cpu = CPU::new();
    let mut crt = CRT::new();
    let commands = read_input();
    for command in commands {
        cpu.process(command);
        while cpu.temp.is_some() {
            cpu.cycle();
            if cpu.temp.is_some() {
                crt.cycle(cpu.registers[0]);
            }
        }
    }
    crt.print();
}

fn read_input() -> Vec<Command> {
    let mut commands = Vec::new();
    for line in INPUT.lines() {
        let mut parts = line.split_whitespace();
        let action = parts.next().unwrap().to_string();
        let value = if let Some(value) = parts.next() {
            value.parse().unwrap()
        } else {
            0
        };
        let cycles = get_cycles(action.as_str());

        commands.push(Command {
            action,
            value,
            cycles,
        });
    }

    commands
}

fn get_cycles(action: &str) -> i32 {
    match action {
        "addx" => 2,
        _ => 1,
    }
}

#[derive(Debug)]
struct Command {
    action: String,
    value: i32,
    cycles: i32,
}

struct CPU {
    temp: Option<Command>,
    registers: [i32; 1],
    cycle_count: i32,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            registers: [1; 1],
            cycle_count: 0,
            temp: None,
        }
    }

    fn cycle(&mut self) {
        if let Some(command) = self.temp.take() {
            if command.cycles == 0 {
                self.execute(command);
            } else {
                self.temp = Some(Command {
                    action: command.action,
                    value: command.value,
                    cycles: command.cycles - 1,
                });
                self.cycle_count += 1;
            }
        }
    }

    fn execute(&mut self, command: Command) {
        match command.action.as_str() {
            "noop" => {}
            "addx" => self.registers[0] += command.value,
            _ => panic!("Unknown command"),
        }
    }

    fn process(&mut self, command: Command) {
        if self.temp.is_none() {
            self.temp = Some(command);
        }
    }

    // function that multiplies the value in register 0 by the cycle count
    fn signal_strength(&self) -> i32 {
        self.registers[0] * self.cycle_count
    }
}

struct CRT {
    pixels: [char; 40 * 6],
    cycle_count: i32,
}

impl CRT {
    fn new() -> CRT {
        CRT {
            pixels: ['*'; 40 * 6],
            cycle_count: 0,
        }
    }

    fn cycle(&mut self, x: i32) {
        let diff = (self.cycle_count % 40) - x;
        if diff >= -1 && diff <= 1 {
            self.pixels[self.cycle_count as usize] = '█';
        } else {
            self.pixels[self.cycle_count as usize] = '░';
        }

        self.cycle_count += 1;
    }

    fn print(&self) {
        for y in 0..6 {
            for x in 0..40 {
                print!("{}", self.pixels[y * 40 + x]);
            }
            println!();
        }
    }
}
