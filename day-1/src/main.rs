use std::io::{self, BufRead};
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let floor = (lines[0].chars().filter(|&c| c == '(').count() as i32) - (lines[0].chars().filter(|&c| c == ')').count() as i32);

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Floor: {}\r\n", floor);

    let find_basement = || -> i32 {
        let mut floor: i32 = 0;
        for (i, c) in lines[0].chars().enumerate() {
            floor += if c == '(' { 1 } else { - 1};
            if floor < 0 {
                return i as i32 + 1;
            }
        }
        return 0;
    };

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("First basement position: {}", find_basement());

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}