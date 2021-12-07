use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

const BIT_LENGTH: u8 = 12;

fn main() {
    if let Ok(binary_lines) = read_input() {
        println!("Result for part1: {}", part1(&binary_lines));
        println!("Result for part2: {}", part2(&binary_lines));
    }
}

fn read_input() -> Result<Vec<u16>, Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let mut binary_lines: Vec<u16> = Vec::new();
    for line in buf_reader.lines() {
        if let Ok(bits) = line {
            binary_lines.push(u16::from_str_radix(&bits, 2).unwrap());
        }
    }
    Ok(binary_lines)
}

fn part1(binary_lines: &Vec<u16>) -> i32 {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for i in (0..BIT_LENGTH).rev() {
        let mut parity = 0;
        for bits in binary_lines.iter() {
            let bit: u8 = nth_bit(&bits, i).unwrap();
            if bit == 1 {
                parity += 1;
            } else {
                parity -= 1;
            }
        }
        if parity >= 0 {
            set_nth_bit(&mut gamma_rate, i);
        } else {
            set_nth_bit(&mut epsilon_rate, i);
        }
    }
    println!("{}", gamma_rate);
    println!("{}", epsilon_rate);
    gamma_rate as i32 * epsilon_rate as i32
}

fn part2(binary_lines: &Vec<u16>) -> i32 {
    let o_rating = oxy_rating(&binary_lines);
    let s_rating = scrubber_rating(&binary_lines);
    o_rating * s_rating
}

fn oxy_rating(binary_lines: &Vec<u16>) -> i32 {
    let mut bits_vec = binary_lines.clone();
    let mut common_bit: Option<u8> = None;
    let mut index: i32 = (BIT_LENGTH - 1).try_into().unwrap();
    loop {
        match common_bit {
            Some(bit) => {
                bits_vec = bits_vec.into_iter().filter(|&x| nth_bit(&x, index.try_into().unwrap()).unwrap() == bit).collect();
                common_bit = None;
                index -= 1;
                if index < 0 || bits_vec.len() <= 1 {
                    break;
                }
            },
            None => {
                let mut parity = 0;
                for bits in bits_vec.iter() {
                    let bit: u8 = nth_bit(&bits, index.try_into().unwrap()).unwrap();
                    if bit == 1 {
                        parity += 1;
                    } else {
                        parity -= 1;
                    }
                }
                common_bit = if parity >= 0 { Some(1) } else { Some(0) }
            },
        }
    }
    println!("{:?}", bits_vec);
    bits_vec[0] as i32
}

fn scrubber_rating(binary_lines: &Vec<u16>) -> i32 {
    let mut bits_vec = binary_lines.clone();
    let mut common_bit: Option<u8> = None;
    let mut index: i32 = (BIT_LENGTH - 1).try_into().unwrap();
    loop {
        match common_bit {
            Some(bit) => {
                bits_vec = bits_vec.into_iter().filter(|&x| nth_bit(&x, index.try_into().unwrap()).unwrap() == bit).collect();
                common_bit = None;
                index -= 1;
                if index < 0 || bits_vec.len() <= 1 {
                    break;
                }
            },
            None => {
                let mut parity = 0;
                for bits in bits_vec.iter() {
                    let bit: u8 = nth_bit(&bits, index.try_into().unwrap()).unwrap();
                    if bit == 1 {
                        parity += 1;
                    } else {
                        parity -= 1;
                    }
                }
                common_bit = if parity >= 0 { Some(0) } else { Some(1) }
            },
        }
    }
    println!("{:?}", bits_vec);
    bits_vec[0] as i32
}

fn nth_bit(input: &u16, n: u8) -> Result<u8, ()> {
    if n < 16 {
        Ok(if input & (1 << n) != 0 { 1 } else { 0 })
    } else {
        Err(())
    }
}

fn set_nth_bit(input: &mut u16, n: u8) {
    *input |= 1 << n;
}

