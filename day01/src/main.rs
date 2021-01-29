use std::collections::HashSet;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("input")?;
    let adjustments = contents
        .split_ascii_whitespace()
        .map(str::parse::<i32>)
        .map(Result::unwrap);

    // let sum: i32 = adjustments.sum();
    // println!("{}", sum);

    let mut frequencies = HashSet::new();
    let mut current_frequency = 0;
    frequencies.insert(current_frequency);

    loop {
        for adjustment in adjustments.clone() {
            current_frequency += adjustment;
            if !frequencies.insert(current_frequency) {
                println!("{}", current_frequency);
                return Ok(());
            }
        }
    }
}
