use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let char_map = parse(&contents);
    print!("{}", contents);
    print!("{:?}", char_map);
    let result = solve_first(&char_map);
    println!("RESULT IS {:?}", result);
    println!("Expected CABDFE");
}


fn solve_first(char_map: &HashMap<char, Vec<char>>) -> String {
    // println!("char map {:?}", char_map);
    char_map.iter().for_each(|(k, v)| {
        println!("{} before {:?}", k, v);
    });

    let mut result = String::new();
    let mut visited = HashSet::<char>::new();

    // Start at keys
    loop {
        let mut keys: Vec<_> = char_map.keys().collect();
        keys.sort_by(|lhs, rhs| rhs.cmp(lhs) ); // a-z reverse sort
        let found = keys.iter().find(|ch| !visited.contains(&ch));
        match found {
            Some(ch) => {
                visited.insert(*ch.clone());
                visit(&ch, &char_map, &mut visited, &mut result);
                result.push(*ch.clone());
            },
            None => {
                break;
            },
        }
    }

    result.chars().rev().collect()
}

fn visit(ch: &char, char_map: &HashMap<char, Vec<char>>, mut visited: &mut HashSet<char>, mut acc: &mut String) {
    let mut current_char: Option<char> = None;
    let mut parent: char = ch.clone();

    loop {
        println!("A");
        match char_map.get(&parent) {
            Some(chars) => {
                let mut sorted_chars: Vec<_> = chars.iter().collect();
                sorted_chars.sort_by(|lhs, rhs| rhs.cmp(lhs)); // a-z reverse sort

                let mut new_char: Option<char> = None;
                for ch in sorted_chars {
                    if !visited.contains(&ch) {
                        new_char = Some(*ch);
                        continue;
                    }
                }

                current_char = new_char;

                match current_char {
                    Some(ch) => {
                        visited.insert(ch.clone());
                        acc.push(ch.clone());
                        parent = ch.clone();
                        println!("Adding {}",ch);
                    },
                    None => break,
                }
            },
            None => {
                println!("C");
                return
            },
        }
    }

}

// Recursive style.
// fn visit(parent: &char, char_map: &HashMap<char, Vec<char>>, mut visited: &mut HashSet<char>, mut acc: &mut String) {
//     match char_map.get(&parent) {
//         Some(chars) => {
//             let mut sorted_chars: Vec<_> = chars.iter().collect();
//             sorted_chars.sort_by(|lhs, rhs| rhs.cmp(lhs)); // a-z reverse sort
//             println!("parents {} chars {:?}", parent, chars);
//             let mut characters = Vec::<char>::new();
//             for ch in sorted_chars {
//                 if visited.contains(&ch) {
//                     continue;
//                 }
//                 visited.insert(ch.clone());
//                 visit(&ch, &char_map, &mut visited, &mut acc);
//                 characters.push(ch.clone());
//             }
//             println!("did insert {:?} for parent {}", &characters, &parent);
//         },
//         None => {
            
//         },
//     }

//     println!("Pushing {}", &parent);
//     acc.push(parent.clone());
// }

fn parse(contents: &str) -> HashMap<char, Vec<char>> {
    contents
        .lines()
        .map(|str| {
            (str.chars().nth(5).unwrap(), str.chars().nth(36).unwrap())
        })
        .fold(HashMap::<char, Vec<char>>::new(), |mut char_map, (lhs, rhs)| {
            char_map.entry(lhs).and_modify(|chars| {
                chars.push(rhs);
            }).or_insert(vec![rhs]);

            char_map
        })
}
