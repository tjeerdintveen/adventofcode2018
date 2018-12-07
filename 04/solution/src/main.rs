use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::result::Result;
extern crate chrono;
use chrono::prelude::*;
use std::str::FromStr;
use std::string::ParseError;

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;
type GuardId = String;

fn main() {
    let mut f = File::open("input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    first_part(&contents);
    // second_part(&contents);
}

fn first_part(contents: &str) {
    let entries = parse_input(&contents);
    // Build raw shifts
    let shifts = build_shifts(entries).unwrap();

    // Group shifts by guard id;
    let mut grouped_shifts = HashMap::<GuardId, Vec<[bool; 60]>>::new();
    for (guard_id, shift) in shifts {
        let entry = grouped_shifts.entry(guard_id).or_insert(Vec::<[bool; 60]>::new());
        entry.push(shift);
    }

    let mut sleepiest_guard = String::new();
    let mut target_minutes_asleep = 0;
    let mut target_reoccuring_minutes: [i32; 60] = [0; 60]; // Which minute the guard slept the most

    // Find guard that slept the most.
    for (guard_id, shifts) in grouped_shifts {
        let mut minutes_asleep = 0;
        let mut reoccuring_minutes: [i32; 60] = [0; 60]; // Which minute the guard slept the most

        for shift in shifts {
            for (i, &was_sleeping) in shift.iter().enumerate() {
                if was_sleeping {
                    reoccuring_minutes[i] += 1;
                    minutes_asleep += 1
                }
            }
        }

        if minutes_asleep > target_minutes_asleep {
            sleepiest_guard = guard_id;
            target_minutes_asleep = minutes_asleep;
            target_reoccuring_minutes = reoccuring_minutes;
        }
    }

    let mut highest = 0;
    let mut value: i32 = 0;
    for (i, &v) in target_reoccuring_minutes.iter().enumerate() {
        if v > value {
            value = v;
            highest = i;
        }
    }

    let id: u32 = sleepiest_guard.parse().unwrap();
    let end_result = id * highest as u32;
    println!("minute end result {}", end_result);
}

fn build_shifts(entries: Vec<Entry>) -> GenResult<Vec<(GuardId, [bool; 60])>> {

    let mut asleep = [false; 60];
    let mut last_asleep_time: Option<NaiveDateTime> = None;
    let mut results = Vec::<(GuardId, [bool; 60])>::new();

    // Values representing current shift
    let mut current_id: Option<GuardId> = None;

    for entry in entries {

        match entry.entry_type {
            EntryType::ShiftStarted { id } => {
                if let Some(id) = current_id {
                    results.push((id, asleep));
                }
                current_id = Some(id);
                last_asleep_time = None;
                asleep = [false; 60];
            },
            EntryType::WakesUp => {
                for i in last_asleep_time.ok_or("No time available")?.minute()..entry.timestamp.minute() {
                    asleep[i as usize] = true;
                }
            },
            EntryType::FallsAsleep => {
                last_asleep_time = Some(entry.timestamp);
            },
        }
    }

    Ok(results)
}

// Parsing

fn parse_input(contents: &str) -> Vec<Entry> {
    let mut entries: Vec<Entry> =
        contents
        .lines()
        .map(|s| parse_string(s))
        .filter_map(|s| s.ok())
        .collect();
    entries.sort_by(|lhs, rhs| lhs.timestamp.cmp(&rhs.timestamp) );
    entries
}

fn parse_string(string: &str) -> GenResult<Entry> {
    let splitted: Vec<_> = string.split(|c: char| {
        return c == '[' || c == ']' || c == '#'
    }).collect();

    // Date
    let parse_from_str = NaiveDateTime::parse_from_str;
    let parsed_date = parse_from_str(&splitted[1], "%Y-%m-%d %H:%M")?;

    if splitted[2].trim() == "Guard" {
        let splitted: Vec<_> = splitted[3].split(' ').collect();
        let id = splitted[0];
        let entry_type: EntryType = id.parse()?;
        let entry = Entry { timestamp: parsed_date, entry_type: entry_type };
        Ok(entry)
    } else {
        let entry_type: EntryType = splitted[2].parse()?;
        let entry = Entry { timestamp: parsed_date, entry_type: entry_type };
        Ok(entry)
    }
}

// Data types

#[derive(Debug, PartialEq, Eq, Hash)]
struct Entry {
    timestamp: NaiveDateTime,
    entry_type: EntryType,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum EntryType {
    ShiftStarted { id: String },
    WakesUp,
    FallsAsleep,
}

impl FromStr for EntryType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s.trim() {
            "wakes up" => EntryType::WakesUp,
            "falls asleep" => EntryType::FallsAsleep,
            _ => EntryType::ShiftStarted { id: s.to_string() },
        };
        Ok(value)
    }
}
