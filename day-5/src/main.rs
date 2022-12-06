use std::io::{self};

fn has_vowels(s: String) -> bool {
    let num_volwes = s.chars().fold(0, |acc , c| {
        if "aeiou".contains(c) {
            acc + 1
        }
        else 
        {
            acc
        }
    });

    return num_volwes >= 3;
}

fn exclude_substrings(s: String) -> bool {
    return !(s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy"));
}


fn has_letter_pairs(s: String) -> bool {
    let letters = s.chars().collect::<Vec<_>>();
    for idx in 0..letters.len()-1 {
        if letters[idx] == letters[idx+1] {
            return true;
        }
    }
    return false;
}

fn count_nice_strings(lines: &Vec<String>) -> usize {

    let num_nice_strings = lines.iter().filter(|s| {
            has_vowels(s.to_string()) && has_letter_pairs(s.to_string()) && exclude_substrings(s.to_string())
        }).count();

    return num_nice_strings;
}

fn has_pairs(s: String) -> bool {
    let letters = s.chars().collect::<Vec<_>>();

    for idx in 0..letters.len() - 3 {
        for check_idx in idx + 2..letters.len() - 1 {
            if letters[idx] == letters[check_idx] && letters[idx+1] == letters[check_idx+1] {
                return true;
            }
        }
    }
    return false;
}

fn book_end_letters(s: String) -> bool {
    let letters = s.chars().collect::<Vec<_>>();
    let windows = letters.windows(3);

    for window in windows {
        if window[0] == window[2] {
            return true;
        }
    }
    return false;
}

fn count_really_nice_strings(lines: &Vec<String>) -> usize {

    let num_nice_strings = lines.iter().filter(|s| {
            has_pairs(s.to_string()) && book_end_letters(s.to_string())
        }).count();

    return num_nice_strings;
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();

    println!("Part 1\r\n{}", "-".repeat(10));
    
    let num_nice_strings = count_nice_strings(&lines);

    println!("Number nice strings: {}\n", num_nice_strings);


    println!("Part 2\r\n{}", "-".repeat(10));
    
    let num_nice_strings = count_really_nice_strings(&lines);

    println!("Number nice strings: {}\n", num_nice_strings);
}
