use nom::{
    character::complete::{char, digit1},
    combinator::{flat_map, map, map_res},
    error::context,
    multi::count,
    sequence::{pair, preceded, separated_pair},
};

use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct TreeNode {
    children: Vec<TreeNode>,
    metadata: Vec<u32>,
}

type Res<T, U> = nom::IResult<T, U, nom::error::VerboseError<T>>;
trait Parse: Sized {
    fn nom_parse(input: &str) -> Res<&str, Self>;
}

fn number(input: &str) -> Res<&str, u32> {
    context("number", map_res(digit1, str::parse::<u32>))(input)
}

#[derive(Debug, Clone)]
struct Header {
    num_children: usize,
    num_metadata: usize,
}

impl Parse for Header {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context(
            "header",
            map(
                separated_pair(number, char(' '), number),
                |(num_children, num_metadata)| Header {
                    num_children: num_children as usize,
                    num_metadata: num_metadata as usize,
                },
            ),
        )(input)
    }
}

impl Parse for TreeNode {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context(
            "TreeNode",
            flat_map(Header::nom_parse, |header| {
                map(
                    pair(
                        count(
                            preceded(char(' '), TreeNode::nom_parse),
                            header.num_children,
                        ),
                        count(preceded(char(' '), number), header.num_metadata),
                    ),
                    |(children, metadata)| TreeNode { children, metadata },
                )
            }),
        )(input)
    }
}

fn sum_metadata(tree: &TreeNode) -> u32 {
    tree.metadata.iter().sum::<u32>()
        + tree.children.iter().map(sum_metadata).sum::<u32>()
}

fn tree_value(tree: &TreeNode) -> u32 {
    if tree.children.is_empty() {
        return sum_metadata(tree);
    }

    let child_indices = tree
        .metadata
        .iter()
        .map(|data| (*data as usize) - 1) // switch to 0-indexing
        .filter(|index| *index < tree.children.len());
    let mut multiplicity: HashMap<usize, u32> = HashMap::new();

    for index in child_indices {
        multiplicity
            .entry(index)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    multiplicity
        .iter()
        .map(|(child, count)| tree_value(&tree.children[*child]) * count)
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    let tree = TreeNode::nom_parse(&contents).unwrap().1;
    println!("{}", tree_value(&tree));
    Ok(())
}
