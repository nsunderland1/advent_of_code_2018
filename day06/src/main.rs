use itertools::Itertools;
// use std::cmp::Ordering;
// use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let lines = io::BufReader::new(file).lines().map(Result::unwrap);
    let points: HashSet<_> = lines
        .map(|line| {
            let mut pieces =
                line.split(", ").map(str::parse::<i32>).map(Result::unwrap);
            Point {
                x: pieces.next().unwrap(),
                y: pieces.next().unwrap(),
            }
        })
        .collect();
    let left = points.iter().min_by_key(|point| point.x).unwrap().x;
    let right = points.iter().max_by_key(|point| point.x).unwrap().x;
    let top = points.iter().min_by_key(|point| point.y).unwrap().y;
    let bottom = points.iter().max_by_key(|point| point.y).unwrap().y;

    let total_size = (left..(right + 1))
        .cartesian_product(top..(bottom + 1))
        .map(|grid_cell| Point {
            x: grid_cell.0,
            y: grid_cell.1,
        })
        .map(|grid_cell| {
            points.iter().map(|point| grid_cell.distance(point)).sum()
        })
        .filter(|dist: &i32| *dist < 10000)
        .count();
    println!("{}", total_size);

    // let mut nearest_point = HashMap::new();
    // for grid_cell in (left..(right + 1)).cartesian_product(top..(bottom + 1)) {
    //     let grid_cell = Point {
    //         x: grid_cell.0,
    //         y: grid_cell.1,
    //     };
    //     let mut min_dist = i32::MAX;
    //     let mut min_point = None;
    //     for point in points.iter() {
    //         let dist = grid_cell.distance(point);
    //         match dist.cmp(&min_dist) {
    //             Ordering::Less => {
    //                 min_dist = dist;
    //                 min_point = Some(point);
    //             }
    //             Ordering::Equal => {
    //                 // If there's a tie, there's no "closest point"
    //                 min_point = None;
    //             }
    //             Ordering::Greater => {}
    //         }
    //     }
    //     nearest_point.insert(grid_cell, min_point);
    // }

    // let best = points
    //     .iter()
    //     .filter_map(|point| {
    //         let nearest_to: Vec<_> = nearest_point
    //             .iter()
    //             .filter_map(|(key, val)| match val {
    //                 Some(val) if *val == point => Some(key),
    //                 _ => None,
    //             })
    //             .collect();
    //         let nearest_on_border = nearest_to
    //             .iter()
    //             .filter(|near_point| {
    //                 near_point.x == left
    //                     || near_point.x == right
    //                     || near_point.y == top
    //                     || near_point.y == bottom
    //             })
    //             .count();
    //         if nearest_on_border > 0 {
    //             None
    //         } else {
    //             Some(nearest_to.len())
    //         }
    //     })
    //     .max()
    //     .unwrap();
    // println!("{}", best);
    Ok(())
}
