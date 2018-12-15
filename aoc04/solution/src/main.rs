use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
extern crate chrono;
use chrono::prelude::*;
use std::str::FromStr;
use std::string::ParseError;

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;
type GuardId = String;

fn main() -> GenResult<()> {
    let mut f = File::open("input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    first_part(&contents)?;
    second_part(&contents)?;
    Ok(())
}

fn first_part(contents: &str) -> GenResult<()> {
    let entries = parse_input(&contents);
    let grouped_shifts = build_shifts(entries)?;

    // Find guard that slept the most.
    let mut sleepiest_guard = String::new();
    let mut target_minutes_asleep = 0;
    let mut target_minute = 0;

    for (guard_id, shifts) in grouped_shifts {
        let (minute, _amount) = minutes_most_slept_on(&shifts);

        let mut minutes_asleep = 0;

        for shift in shifts {
            for &was_sleeping in shift.iter() {
                if was_sleeping {
                    minutes_asleep += 1
                }
            }
        }

        if minutes_asleep > target_minutes_asleep {
            sleepiest_guard = guard_id;
            target_minutes_asleep = minutes_asleep;
            target_minute = minute;
        }
    }

    let id: u32 = sleepiest_guard.parse()?;
    let end_result = id * target_minute as u32;
    println!("minute end result {}", end_result);
    Ok(())
}

fn second_part(contents: &str) -> GenResult<()> {
    let entries = parse_input(&contents);
    let grouped_shifts = build_shifts(entries)?;

    let mut guards_minutes = HashMap::<GuardId, (usize, i32)>::new();

    for (guard_id, shifts) in grouped_shifts {
        let (minute, amount) = minutes_most_slept_on(&shifts);
        guards_minutes.entry(guard_id).or_insert((minute, amount));
    }

    let guard = guards_minutes.iter().max_by_key(|v| v.1);

    match guard {
        Some((id, (minute, _amount))) => {
            let idsize: usize = id.parse()?;
            let answer = idsize * minute;
            println!("Second answer is {}", answer);
            Ok(())
        }
        None => panic!("No guard found"), // I don't want to make a custom error ;-)
    }
}

type Minute = usize;
// Find the minute that was slept on the most. Returns the minute and the amount.
fn minutes_most_slept_on(shifts: &Vec<[bool; 60]>) -> (Minute, i32) {
    let mut reoccuring_minutes: [i32; 60] = [0; 60];

    for shift in shifts {
        for (i, &was_sleeping) in shift.iter().enumerate() {
            if was_sleeping {
                reoccuring_minutes[i] += 1;
            }
        }
    }

    let mut minute = 0;
    let mut amount: i32 = 0;
    for (i, &v) in reoccuring_minutes.iter().enumerate() {
        if v > amount {
            amount = v;
            minute = i;
        }
    }

    (minute, amount)
}

// Shifts

fn build_shifts(entries: Vec<Entry>) -> GenResult<HashMap<GuardId, Vec<[bool; 60]>>> {
    let mut asleep = [false; 60];
    let mut last_asleep_time: Option<NaiveDateTime> = None;
    let mut shifts = Vec::<(GuardId, [bool; 60])>::new();

    // Values representing current shift
    let mut current_id: Option<GuardId> = None;

    for entry in entries {
        match entry.entry_type {
            EntryType::ShiftStarted { id } => {
                if let Some(id) = current_id {
                    shifts.push((id, asleep));
                }
                current_id = Some(id);
                last_asleep_time = None;
                asleep = [false; 60];
            }
            EntryType::WakesUp => {
                for i in
                    last_asleep_time.ok_or("No time available")?.minute()..entry.timestamp.minute()
                {
                    asleep[i as usize] = true;
                }
            }
            EntryType::FallsAsleep => {
                last_asleep_time = Some(entry.timestamp);
            }
        }
    }

    // Group shifts by guard id;
    let mut grouped_shifts = HashMap::<GuardId, Vec<[bool; 60]>>::new();
    for (guard_id, shift) in shifts {
        let entry = grouped_shifts
            .entry(guard_id)
            .or_insert(Vec::<[bool; 60]>::new());
        entry.push(shift);
    }
    Ok(grouped_shifts)
}

// Parsing

fn parse_input(contents: &str) -> Vec<Entry> {
    let mut entries: Vec<Entry> = contents
        .lines()
        .map(|s| parse_string(s))
        .filter_map(|s| s.ok())
        .collect();
    entries.sort_by(|lhs, rhs| lhs.timestamp.cmp(&rhs.timestamp));
    entries
}

fn parse_string(string: &str) -> GenResult<Entry> {
    let splitted: Vec<_> = string
        .split(|c: char| return c == '[' || c == ']' || c == '#')
        .collect();

    // Date
    let parse_from_str = NaiveDateTime::parse_from_str;
    let parsed_date = parse_from_str(&splitted[1], "%Y-%m-%d %H:%M")?;

    if splitted[2].trim() == "Guard" {
        let splitted: Vec<_> = splitted[3].split(' ').collect();
        let id = splitted[0];
        let entry_type: EntryType = id.parse()?;
        let entry = Entry {
            timestamp: parsed_date,
            entry_type: entry_type,
        };
        Ok(entry)
    } else {
        let entry_type: EntryType = splitted[2].parse()?;
        let entry = Entry {
            timestamp: parsed_date,
            entry_type: entry_type,
        };
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
