use std::io::{self};

#[derive(Copy,Clone)]
enum LightState {
    Off = 0,
    On = 1,
    Toggle = 2
}

struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize
}

impl Rect {
    fn from_strings(first: &str, second: &str) -> Rect {
        let mut point = first.split(",");
        let x1 = point.next().unwrap().parse::<usize>().unwrap();
        let y1 = point.next().unwrap().parse::<usize>().unwrap();
        let mut point = second.split(",");
        let x2 = point.next().unwrap().parse::<usize>().unwrap();
        let y2 = point.next().unwrap().parse::<usize>().unwrap();
        return Rect { x1: x1, y1: y1, x2: x2, y2: y2 };
    }

    fn from_parser(parser: &mut std::str::Split<&str>) -> Rect {
        let first = parser.next().unwrap();
        parser.next();
        let second = parser.next().unwrap();
        return Rect::from_strings(first,second);
    }
}

struct Lights {
    bulbs: Vec<Vec<bool>>,
    dimmered: Vec<Vec<usize>>
}

impl Lights {
    fn new() -> Self {
        return Lights { bulbs: vec![vec![ false; 1000]; 1000], dimmered: vec![vec![ 0; 1000]; 1000] };
    }

    fn apply_command(&mut self, cmd: String) {
        let mut parse = cmd.split(" ");
        let state = match parse.next().unwrap() {
            "turn" => {
                match parse.next().unwrap() {
                    "on" => LightState::On,
                    "off" => LightState::Off,
                    _ => panic!("Oh no!!!")
                }
            },
            "toggle" => LightState::Toggle,
            _ => panic!("Throw the switch Vern!")
        };
    
        let rect = Rect::from_parser(&mut parse);
        self._apply_range(rect, state);
    }

    fn _apply_range(&mut self, rect: Rect, state: LightState) {
        let row_start = usize::min(rect.x1, rect.x2);
        let row_end = usize::max(rect.x1, rect.x2);
        let col_start = usize::min(rect.y1, rect.y2);
        let col_end = usize::max(rect.y1, rect.y2);

        for row in row_start..=row_end {
            for col in col_start..=col_end {
                match state {
                    LightState::On => self.bulbs[row][col] = true,
                    LightState::Off => self.bulbs[row][col] = false,
                    LightState::Toggle => self.bulbs[row][col] = !self.bulbs[row][col]
                }
                match state {
                    LightState::On => self.dimmered[row][col] = self.dimmered[row][col] + 1,
                    LightState::Off => {
                        if self.dimmered[row][col] > 0 {
                            self.dimmered[row][col] = self.dimmered[row][col] - 1
                        }
                    },
                    LightState::Toggle => self.dimmered[row][col] = self.dimmered[row][col] + 2
                }
            }
        }
    }

    fn num_lights_on(&mut self) -> i32 {
        return self.bulbs.iter().fold(0,|acc, col| {
            acc + col.iter().fold(0, |acc, &b| {
                if b {
                    acc + 1
                }
                else {
                    acc
                }
            } )
        });
    }

    fn total_brightness(&mut self) -> i32 {
        return self.dimmered.iter().fold(0,|acc, col| {
            acc + col.iter().sum::<usize>() as i32
        });
    }


}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();

    let mut lights = Lights::new();
    lines.iter().for_each(|line| {
        lights.apply_command(line.to_string());
    });

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Number of lights on: {}\r\n", lights.num_lights_on());
    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Total brightness: {}", lights.total_brightness());
}
