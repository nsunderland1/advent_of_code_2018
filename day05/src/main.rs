use std::collections::HashSet;
use std::collections::LinkedList;
use std::error::Error;
use std::fs;

fn reduced_len(polymer_string: &str) -> usize {
    let mut polymer: LinkedList<char> = LinkedList::new();

    for chr in polymer_string.chars() {
        match polymer.back() {
            Some(prev)
                if *prev != chr
                    && prev.to_ascii_lowercase()
                        == chr.to_ascii_lowercase() =>
            {
                polymer.pop_back();
            }
            _ => polymer.push_back(chr),
        }
    }
    polymer.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    let polymer_string = contents.trim_end();
    let mut distinct_units = HashSet::new();

    for chr in polymer_string.chars() {
        distinct_units.insert(chr.to_ascii_lowercase());
    }

    let best = distinct_units
        .iter()
        .map(|unit| {
            reduced_len(
                &polymer_string
                    .chars()
                    .filter(|chr| chr.to_ascii_lowercase() != *unit)
                    .collect::<String>(),
            )
        })
        .min()
        .unwrap();
    println!("{}", best);
    Ok(())
}
