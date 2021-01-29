use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    error::context,
    sequence::{preceded, separated_pair, tuple},
};
use std::cmp::Eq;
// use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Claim {
    id: ClaimID,
    position: Position,
    dimensions: Dimensions,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct ClaimID(u32);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Dimensions {
    width: u32,
    height: u32,
}

type Res<T, U> = nom::IResult<T, U, nom::error::VerboseError<T>>;
trait Parse: Sized {
    fn nom_parse(input: &str) -> Res<&str, Self>;
}

fn number(input: &str) -> Res<&str, u32> {
    context(
        "number",
        map_res(digit1, |result: &str| result.parse::<u32>()),
    )(input)
}

impl Parse for ClaimID {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context("ClaimID", map(preceded(tag("#"), number), ClaimID))(input)
    }
}

impl Parse for Position {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context(
            "Position",
            map(separated_pair(number, tag(","), number), |(x, y)| {
                Position { x, y }
            }),
        )(input)
    }
}

impl Parse for Dimensions {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context(
            "Dimensions",
            map(
                separated_pair(number, tag("x"), number),
                |(width, height)| Dimensions { width, height },
            ),
        )(input)
    }
}

impl Parse for Claim {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context(
            "Claim",
            map(
                tuple((
                    ClaimID::nom_parse,
                    tag(" @ "),
                    Position::nom_parse,
                    tag(": "),
                    Dimensions::nom_parse,
                )),
                |(id, _, position, _, dimensions)| Claim {
                    id,
                    position,
                    dimensions,
                },
            ),
        )(input)
    }
}

// fn all_positions(claim: Claim) -> impl Iterator<Item = Position> {
//     let Claim {
//         position,
//         dimensions,
//         ..
//     } = claim;
//     (position.x..(position.x + dimensions.width)).flat_map(move |x| {
//         (position.y..(position.y + dimensions.height))
//             .map(move |y| Position { x, y })
//     })
// }

fn overlapping(claim1: &Claim, claim2: &Claim) -> bool {
    let left = |claim: &Claim| claim.position.x;
    let right = |claim: &Claim| claim.position.x + claim.dimensions.width - 1;
    let top = |claim: &Claim| claim.position.y;
    let bottom = |claim: &Claim| claim.position.y + claim.dimensions.height - 1;
    let bounds =
        |claim: &Claim| (left(claim), right(claim), top(claim), bottom(claim));
    let (l1, r1, t1, b1) = bounds(claim1);
    let (l2, r2, t2, b2) = bounds(claim2);
    !(l1 > r2 || l2 > r1 || t1 > b2 || t2 > b1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let lines = io::BufReader::new(file).lines().map(Result::unwrap);
    let claims: Vec<_> = lines
        .map(|line| {
            let (rest, claim) = Claim::nom_parse(&line).unwrap();
            assert_eq!(rest, "");
            claim
        })
        .collect();
    let mut non_overlapping: HashSet<_> = claims.iter().collect();
    for (pos, claim1) in claims.iter().enumerate() {
        for claim2 in claims.iter().skip(pos + 1) {
            if overlapping(claim1, claim2) {
                non_overlapping.remove(claim1);
                non_overlapping.remove(claim2);
            }
        }
    }
    assert_eq!(non_overlapping.len(), 1);
    println!("{:?}", non_overlapping.iter().next().unwrap().id);

    // let mut pos_count: HashMap<Position, u32> = HashMap::new();
    // for claim in claims.iter() {
    //     for position in all_positions(claim) {
    //         let count = pos_count.get(&position).unwrap_or(&0).clone();
    //         pos_count.insert(position, count + 1);
    //     }
    // }
    // println!(
    //     "{}",
    //     pos_count.values().filter(|count| **count >= 2).count()
    // );
    Ok(())
}
