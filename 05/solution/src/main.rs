use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let first_answer = solve_first(&contents);
    println!("first answer {}", first_answer);

    let second_answer = solve_second(&contents);
    println!("Second answer {}", second_answer);
}

fn solve_first(contents: &str) -> usize {
    react_multi_pass(&contents).len()
}

fn solve_second(contents: &str) -> usize {
    // react_multi_pass(&contents).len()
    let mut shortest = 0;
    // Generate a-z
    (97u8..123u8).for_each(|i| {
        let ch = i as char;
        let length = react_multi_pass(&contents.replace(ch, "")).len();

        println!("length for {} is {}", ch, length);
        if length < shortest || shortest == 0 {
            shortest = length
        }
    });
    shortest
}

fn react_multi_pass(polymer: &str) -> String {
    let mut reaction = polymer.to_string();
    loop {
        let (string, did_react) = react_single_pass(&reaction);
        if !did_react {
            return reaction;
        }

        reaction = string;
    }
}

fn react_single_pass(polymer: &str) -> (String, bool) {
    let eos = '!'; // custom end of string char
    let mut had_match = false;
    let mut did_react = false;

    let reaction = polymer
        .trim()
        .chars()
        .zip(// zipping the same chars, offet by one. Adding a EOF char.
            polymer
                .chars()
                .skip(1)
                .chain(eos.to_string().chars()))
        .fold(String::new(), |mut acc, (lhs, rhs)| {

            if had_match {
                had_match = false;
                return acc;
            }

            if rhs == eos {
                acc.push(lhs);
                return acc;
            }

            if are_polar_opposites(&lhs, &rhs) {
                had_match = true;
                did_react = true;
                return acc;
            } else {
                acc.push(lhs);
                return acc;
            }
        });

    (reaction, did_react)
}

fn are_polar_opposites(lhs: &char, rhs: &char) -> bool {
    if *lhs > *rhs {
        return *lhs as u32 - *rhs as u32 == 32;
    } else {
        return *rhs as u32 - *lhs as u32 == 32;
    }
}

#[test]
fn test_react_multi_pass_multiple_reactions() {
    // Make sure that new combinations after a single cause new reactions
    assert_eq!("dabCBAcaDA", react_multi_pass("dabAcCaCBAcCcaDA"));
    assert_eq!("", react_multi_pass("abcCBA"));
    assert_eq!("ff", react_multi_pass("fabcCBAf"));
    assert_eq!("fbaBAf", react_multi_pass("fbacCBAf"));
}

#[test]
fn test_react_multi_pass_simple() {
    // Make sure that single reactions occur, like single_pass
    assert_eq!("", react_multi_pass("aA"));
    assert_eq!("", react_multi_pass("aABb"));
    assert_eq!("aa", react_multi_pass("aABbaabB"));
    assert_eq!("bb", react_multi_pass("bBAabb"));
}

#[test]
fn test_react_single_pass() {
    assert_eq!(("".to_string(), true), react_single_pass("aA"));
    assert_eq!(("".to_string(), true), react_single_pass("aABb"));
    assert_eq!(("aa".to_string(), true), react_single_pass("aABbaabB"));
    assert_eq!(("bb".to_string(), true), react_single_pass("bBAabb"));

    // no reactions
    assert_eq!(("bb".to_string(), false), react_single_pass("bb"));
}

#[test]
fn test_polar_opposites() {
    assert!(are_polar_opposites(&'a', &'A'));
    assert!(are_polar_opposites(&'A', &'a'));
    assert!(!are_polar_opposites(&'a', &'a'), "Same chars shouldnt be polar opposites");
    assert!(!are_polar_opposites(&'A', &'A'), "Same chars shouldnt be polar opposites");
    assert!(!are_polar_opposites(&'b', &'a'), "diff chars shouldnt be polar opposites");
}
