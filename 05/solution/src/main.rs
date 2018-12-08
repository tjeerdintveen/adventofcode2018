// type GenError = Box<std::error::Error>;
// type GenResult<T> = Result<T, GenError>;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let first_answer = react_multi_pass(&contents).len();
    println!("first answer {}", first_answer);

    let ch = 'a' as u8;
    println!("ch {}", ch);
}

fn react_multi_pass(polymer: &str) -> String {
    let mut reaction = polymer.to_string();
    loop {
        let (string, did_react) = react_single_pass(&reaction);
        if !did_react {
            // println!("{}", reaction);
            return reaction;
        }

        reaction = string;
    }
}

fn react_single_pass(polymer: &str) -> (String, bool) {
    // println!("Before {}", polymer);
    let eos = '!'; // custom end of string char
    let mut skip = false;
    let mut did_react = false;

    let reaction = polymer
        .trim()
        .chars()
    // zipping the same chars, offet by one. Adding a EOF char.
        .zip(
            polymer
                .chars()
                .skip(1)
                .chain(eos.to_string().chars()))
        .fold(String::new(), |mut acc, (lhs, rhs)| {
            if skip {
                skip = false;
                return acc;
            }

            if rhs == eos {
                acc.push(lhs);
                return acc;
            }

            if are_polar_opposites(&lhs, &rhs) {
                skip = true;
                did_react = true;
                return acc;
            } else {
                acc.push(lhs);
                return acc;
            }
        });

    // println!("After did react {} {}", did_react, reaction);
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
fn test_react_single_pass_removing_char() {
    assert_eq!("", react_single_pass("aaABbaabBa", None));
    assert_eq!("B", react_single_pass("aaABBbaabBa", None));
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
