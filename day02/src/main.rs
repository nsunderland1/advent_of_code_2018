// use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .collect();
    // let mut two_count = 0;
    // let mut three_count = 0;
    // for line in lines.iter() {
    //     let mut char_count = HashMap::new();
    //     for chr in line.chars() {
    //         char_count.insert(chr, char_count.get(&chr).unwrap_or(&0) + 1);
    //     }
    //     if char_count.values().any(|&v| v == 2) {
    //         two_count += 1;
    //     }
    //     if char_count.values().any(|&v| v == 3) {
    //         three_count += 1;
    //     }
    // }
    // println!("{}", two_count * three_count);
    for (pos, str1) in lines.iter().enumerate() {
        for str2 in lines.iter().skip(pos + 1) {
            if str1
                .chars()
                .zip(str2.chars())
                .filter(|(a, b)| a != b)
                .count()
                == 1
            {
                println!("{} {}", str1, str2);
                return Ok(());
            }
        }
    }
    Ok(())
}
