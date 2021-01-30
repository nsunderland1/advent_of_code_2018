use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, value},
    error::context,
    sequence::{delimited, preceded, separated_pair, tuple},
};

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Event {
    datetime: DateTime,
    action: Action,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Copy)]
struct GuardID(u32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default, Clone)]
struct DateTime {
    date: Date,
    time: Time,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default, Clone)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default, Clone)]
struct Time {
    hour: u32,
    minute: u32,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Action {
    StartShift(GuardID),
    Sleep,
    Wake,
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

impl Parse for Date {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context(
            "Date",
            map(
                tuple((number, tag("-"), number, tag("-"), number)),
                |(year, _, month, _, day)| Date { year, month, day },
            ),
        )(input)
    }
}

impl Parse for Time {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context(
            "Time",
            map(
                separated_pair(number, tag(":"), number),
                |(hour, minute)| Time { hour, minute },
            ),
        )(input)
    }
}

impl Parse for DateTime {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context(
            "DateTime",
            map(
                delimited(
                    tag("["),
                    separated_pair(Date::nom_parse, tag(" "), Time::nom_parse),
                    tag("]"),
                ),
                |(date, time)| DateTime { date, time },
            ),
        )(input)
    }
}

impl Parse for GuardID {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context("GuardID", map(preceded(tag("#"), number), GuardID))(input)
    }
}

impl Parse for Action {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context(
            "Action",
            alt((
                value(Action::Sleep, tag("falls asleep")),
                value(Action::Wake, tag("wakes up")),
                map(
                    delimited(
                        tag("Guard "),
                        GuardID::nom_parse,
                        tag(" begins shift"),
                    ),
                    Action::StartShift,
                ),
            )),
        )(input)
    }
}

impl Parse for Event {
    fn nom_parse(input: &str) -> Res<&str, Self> {
        context(
            "Event",
            map(
                separated_pair(
                    DateTime::nom_parse,
                    tag(" "),
                    Action::nom_parse,
                ),
                |(datetime, action)| Event { datetime, action },
            ),
        )(input)
    }
}

fn to_minutes(time: &Time) -> u32 {
    time.minute + 60 * time.hour
}

fn time_passed(from: &DateTime, to: &DateTime) -> u32 {
    if to < from {
        panic!("Invalid time comparison");
    }
    to_minutes(&to.time) - to_minutes(&from.time)
}

fn all_minutes(from: &DateTime, to: &DateTime) -> impl Iterator<Item = u32> {
    if to < from {
        panic!("all_minutes invalid");
    }
    to_minutes(&from.time)..to_minutes(&to.time)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let lines = io::BufReader::new(file).lines().map(Result::unwrap);
    let mut events: Vec<_> = lines
        .map(|line| {
            let (rest, event) = Event::nom_parse(&line).unwrap();
            assert_eq!(rest, "");
            event
        })
        .collect();
    events.sort();
    if let Action::StartShift(guard_id) = events[0].action {
        let mut sleep_per_minute = HashMap::new();
        let mut guard_id = guard_id;
        let mut fell_asleep = DateTime::default();
        for event in events.iter() {
            match event.action {
                Action::StartShift(id) => guard_id = id,
                Action::Sleep => fell_asleep = event.datetime.clone(),
                Action::Wake => {
                    for minute in all_minutes(&fell_asleep, &event.datetime) {
                        sleep_per_minute.insert(
                            (guard_id, minute),
                            sleep_per_minute
                                .get(&(guard_id, minute))
                                .unwrap_or(&0)
                                + 1,
                        );
                    }
                }
            }
        }

        let ((sleepiest_guard, sleepiest_minute), _) = sleep_per_minute
            .iter()
            .max_by(|(_, times1), (_, times2)| times1.cmp(times2))
            .unwrap();
        println!("{}", sleepiest_guard.0 * sleepiest_minute);

    // let mut sleep_minutes = HashMap::new();
    // {
    //     let mut guard_id = guard_id;
    //     let mut fell_asleep = DateTime::default();

    //     for event in events.iter() {
    //         match event.action {
    //             Action::StartShift(id) => {
    //                 if !sleep_minutes.contains_key(&id) {
    //                     sleep_minutes.insert(id, 0);
    //                 }
    //                 guard_id = id;
    //             }
    //             Action::Sleep => {
    //                 fell_asleep = event.datetime.clone();
    //             }
    //             Action::Wake => {
    //                 sleep_minutes.insert(
    //                     guard_id,
    //                     sleep_minutes.get(&guard_id).unwrap()
    //                         + time_passed(&fell_asleep, &event.datetime),
    //                 );
    //             }
    //         }
    //     }
    // }

    // let (sleepiest_guard, _) = sleep_minutes
    //     .iter()
    //     .max_by(|(_, minutes1), (_, minutes2)| minutes1.cmp(minutes2))
    //     .unwrap();

    // let mut guard_id = guard_id;
    // let mut fell_asleep = DateTime::default();
    // let mut sleep_per_minute = HashMap::new();
    // for event in events.iter() {
    //     match event.action {
    //         Action::StartShift(id) => guard_id = id,
    //         _ if guard_id != *sleepiest_guard => continue,
    //         Action::Sleep => fell_asleep = event.datetime.clone(),
    //         Action::Wake => {
    //             for minute in all_minutes(&fell_asleep, &event.datetime) {
    //                 sleep_per_minute.insert(
    //                     minute,
    //                     sleep_per_minute.get(&minute).unwrap_or(&0) + 1,
    //                 );
    //             }
    //         }
    //     }
    // }

    // let (sleepiest_minute, _) = sleep_per_minute
    //     .iter()
    //     .max_by(|(_, times1), (_, times2)| times1.cmp(times2))
    //     .unwrap();
    // println!("{}", sleepiest_guard.0 * sleepiest_minute);
    } else {
        panic!("First action isn't a shift start");
    }

    Ok(())
}
