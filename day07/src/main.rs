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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone, Default)]
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

fn task_time(step: &Step) -> u32 {
    (((step.0 as u8) - ('A' as u8) + 1) as u32) + 60
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
    for dependency in dependencies.iter() {
        (*dependers.entry(dependency.prereq).or_insert(Vec::new()))
            .push(dependency.depender);
        *number_of_prereqs.entry(dependency.depender).or_insert(0) += 1;
        if !number_of_prereqs.contains_key(&dependency.prereq) {
            number_of_prereqs.insert(dependency.prereq, 0);
        }
    }

    let mut queue = BinaryHeap::new();
    let prereqs_iter = number_of_prereqs.iter();
    for (step, num_prereqs) in prereqs_iter {
        if *num_prereqs == 0 {
            queue.push(Reverse(step.clone()));
        }
    }
    let mut step_order = Vec::new();
    let mut number_of_prereqs_left = number_of_prereqs.clone();
    while let Some(Reverse(step)) = queue.pop() {
        step_order.push(step);
        if let Some(step_dependers) = dependers.get(&step) {
            for depender in step_dependers.iter() {
                let num_prereqs = number_of_prereqs_left
                    .entry(*depender)
                    .and_modify(|e| *e -= 1)
                    .or_default();
                if *num_prereqs == 0 {
                    queue.push(Reverse(*depender));
                }
            }
        }
    }
    number_of_prereqs_left = number_of_prereqs.clone();

    let mut active_tasks: [Option<(Step, u32)>; 5];

    Ok(())
}
