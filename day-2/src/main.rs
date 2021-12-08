use std::io::{self, BufRead};
use std::time::{Duration, Instant};
use std::cmp::min;

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let areas: i32 = lines.iter().map(|line| {
        let dims: Vec<i32> = line.split("x").flat_map(|dim| dim.trim().parse::<i32>()).collect();
        return 2 *(dims[0] * dims[1] + dims[0] * dims[2] + dims[1] * dims[2])
                + min(dims[0] * dims[1],min(dims[0] * dims[2], dims[1] * dims[2]));
        }).sum();

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Total area: {}", areas);

    let feet: i32 = lines.iter().map(|line| {
        let mut dims: Vec<i32> = line.split("x").flat_map(|dim| dim.trim().parse::<i32>()).collect();
        dims.sort();
        return 2*(dims[0] + dims[1]) + dims[0] * dims[1] * dims[2];
        }).sum();

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Total ribbon: {}", feet);

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}