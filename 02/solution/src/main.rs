use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    first_part(&contents);
    second_part(&contents);
}

fn first_part(contents: &str) {
    let lines = contents
        .lines()
        .map(|s| s.chars());

    let mut second_occ = 0;
    let mut third_occ = 0;

    for chars in lines {
        // build charmap
        let mut charmap = HashMap::<char, i32>::new();
        for char in chars {
            let entry = charmap.entry(char).or_insert(0);
            *entry += 1;
        }

        // count occurences
        let mut second_occ_found = false;
        let mut third_occ_found = false;

        for (_, &v) in &charmap {
            if v == 2 && !second_occ_found {
                second_occ += 1;
                second_occ_found = true;
            } else if v == 3 && !third_occ_found {
                third_occ += 1;
                third_occ_found = true;
            }
        }
    }

    let total = second_occ * third_occ;
    println!("Result is {}", total);
}

fn second_part(contents: &str) {
    let lines: Vec<_> = contents
        .lines()
        .collect();

    for i in 0..lines.len() {
        for j in i+1..lines.len() {
            if let Some(common) = common_letters(&lines[i], &lines[j]) {
                println!("{}", common);
            }
        }
    }
}

fn common_letters(lhs: &str, rhs: &str) -> Option<String> {
    let mut found_mismatch = false;
    let mut string = String::new();
    for (l, r) in lhs.chars().zip(rhs.chars()) {
        if l != r {
            if found_mismatch {
                return None
    

            found_mismatch = true
        } else {
            string.push(l);
        }
    }
    return Some(string);
}

