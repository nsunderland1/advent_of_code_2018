use nom::{
    bytes::complete::tag, character::complete::satisfy, combinator::map,
    error::context, sequence::tuple,
};

use std::cmp::Eq;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone)]
struct Step(char);

struct Dependency {
    depender: Step,
    prereq: Step,
}

type Res<T, U> = nom::IResult<T, U, nom::error::VerboseError<T>>;
trait Parse: Sized {
    fn nom_parse(input: &str) -> Res<&str, Self>;
}

impl Parse for Step {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context("Step", map(satisfy(|c| c >= 'A' && c <= 'Z'), Step))(input)
    }
}

impl Parse for Dependency {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context(
            "Dependency",
            map(
                tuple((
                    tag("Step "),
                    Step::nom_parse,
                    tag(" must be finished before step "),
                    Step::nom_parse,
                    tag(" can begin."),
                )),
                |(_, prereq, _, depender, _)| Dependency { prereq, depender },
            ),
        )(input)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let lines = io::BufReader::new(file).lines().map(Result::unwrap);
    let dependencies: Vec<_> = lines
        .map(|line| {
            let (rest, dependency) = Dependency::nom_parse(&line).unwrap();
            assert_eq!(rest, "");
            dependency
        })
        .collect();
    let mut number_of_prereqs: HashMap<Step, u32> = HashMap::new();
    let mut dependers: HashMap<Step, Vec<Step>> = HashMap::new();
    // for dependency in dependencies.iter() {
    //     let &mut deps = dependers
    //         .get_mut(&dependency.prereq)
    //         .unwrap_or(&mut Vec::new());
    //     deps.push(dependency.depender);
    //     let &mut num_prereqs = number_of_prereqs
    //         .get_mut(&dependency.depender)
    //         .unwrap_or(&mut 0);
    //     num_prereqs += 1;
    // }
    let mut queue = BinaryHeap::new();
    {
        let prereqs_iter = number_of_prereqs.iter();
        for (step, num_prereqs) in prereqs_iter {
            if *num_prereqs == 0 {
                queue.push(Reverse(step));
            }
        }
    }
    {
        let mut step_order = Vec::new();
        while let Some(Reverse(step)) = queue.pop() {
            step_order.push(step);
            for depender in dependers.get(step).unwrap().iter() {
                let num_prereqs = number_of_prereqs
                    .entry(*depender)
                    .and_modify(|e| *e -= 1)
                    .or_default();
                if *num_prereqs == 0 {
                    queue.push(Reverse(depender));
                }
            }
        }
        println!("{:?}", step_order);
    }
    Ok(())
}
