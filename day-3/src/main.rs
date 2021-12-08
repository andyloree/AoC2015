use std::io::{self, BufRead};
use std::time::{Duration, Instant};
use std::collections::HashMap;

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let mut path: HashMap<(i32,i32),i32> = HashMap::new();
    path.insert((0,0), 1);
    let mut position: (i32,i32) = (0,0);
    let mut delivered: i32 = 1; // home
    lines[0].chars().for_each(|dir| {
        match dir {
            '^' => position.1 += 1,
            '>' => position.0 += 1,
            'v' => position.1 += -1,
            '<' => position.0 += -1,
            _ => unimplemented!()
        }
        if !path.contains_key(&position) {
            path.insert(position, 1);
            delivered += 1;
        }
    });

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Houses delivered at least once: {}", delivered);

    let mut path: HashMap<(i32,i32),i32> = HashMap::new();
    path.insert((0,0), 1);  // no place like home
    let mut santa: (i32,i32) = (0,0);
    let mut robot: (i32,i32) = (0,0);
    let mut delivered: i32 = 1; // home
    let mut your_move = | position: &mut (i32, i32), dir: char| {
        match dir {
            '^' => position.1 += 1,
            '>' => position.0 += 1,
            'v' => position.1 += -1,
            '<' => position.0 += -1,
            _ => unimplemented!()
        }
        if !path.contains_key(&position) {
            path.insert(*position, 1);
            delivered += 1;
        }
    };

    lines[0].chars().enumerate().for_each(|(i,dir)| {
        if i % 2 == 0{
            your_move(&mut santa, dir);
        }
        else
        {
            your_move(&mut robot, dir);
        }
    });


    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Houses delivered at least once: {}", delivered);

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}