use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut f = File::open("input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    first_part(&contents);
    second_part(&contents);
}

fn first_part(contents: &str) {
    let result: i32 = contents
        .lines()
        .filter_map(|line| -> Option<i32> { line.parse().ok() })
        .sum();

    println!("Frequency is {}", result);
}

fn second_part(contents: &str) {
    let mut set = HashSet::new();
    let mut target_frequency: i32 = 0;

    loop {
        let iterator = contents
            .lines()
            .filter_map(|line| line.parse().ok());

        for frequency in iterator {
            target_frequency += frequency;
            if set.contains(&target_frequency) {
                println!("Reorcurring frequency {}", target_frequency);
                return;
            }

            set.insert(target_frequency);
        }
    }
}
