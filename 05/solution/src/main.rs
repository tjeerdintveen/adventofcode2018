// type GenError = Box<std::error::Error>;
// type GenResult<T> = Result<T, GenError>;

fn main() {
    println!("Hello, world!");
}

fn react_single_pass(polymer: &str) -> String {
    let mut skip = false;
    polymer
        .chars()
    // zipping the same chars, offet by one. Adding a EOF char.
        .zip(polymer.chars().skip(1).chain("!".to_string().chars()))
        .fold(String::new(), |acc, (lhs, rhs)| {
            // // TODO use slice
            println!("receive lhs rhs {} {}", lhs, rhs);
            if skip {
                skip = false;
                return acc;
            }

            let mut str = acc;
            if rhs == '!' {
                str.push(lhs);
                return str;
            } else if lhs == '!' {
                return str;
            }

            if are_polar_opposites(&lhs, &rhs) {
                skip = true;
                return str;
            } else {
                str.push(lhs);
            }
            return str;
        })
}

fn are_polar_opposites(lhs: &char, rhs: &char) -> bool {
    // let res = *lhs as u32 - *rhs as u32 == 32;
    // println!("res {}", res);
    // res
    if lhs == rhs { return false; }

    let result = match (lhs.is_uppercase(), rhs.is_uppercase()) {
        (true, true) => false,
        (false, false) => false,
        (true, false) => {
            *lhs == rhs.to_uppercase().next().unwrap()
        },
        (false, true) => {
            lhs.to_uppercase().next().unwrap() == *rhs
        },
    };

    result
}

#[test]
fn test_react_single_pass() {
    assert_eq!("", react_single_pass("aA"));
    assert_eq!("", react_single_pass("aABb"));
    assert_eq!("aa", react_single_pass("aABbaabB"));
    assert_eq!("bb", react_single_pass("bBAabb"));
}

#[test]
fn test_polar_opposites() {
    assert!(are_polar_opposites(&'a', &'A'));
    assert!(are_polar_opposites(&'A', &'a'));
    assert!(!are_polar_opposites(&'a', &'a'), "Same chars shouldnt be polar opposites");
    assert!(!are_polar_opposites(&'A', &'A'), "Same chars shouldnt be polar opposites");
    assert!(!are_polar_opposites(&'b', &'a'), "diff chars shouldnt be polar opposites");
}
