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
            },
            None => {
                break;
            },
        }
    }

    result.chars().rev().collect()
}

fn visit(parent: &char, char_map: &HashMap<char, Vec<char>>, mut visited: &mut HashSet<char>, mut acc: &mut String) {
    match char_map.get(&parent) {
        Some(chars) => {
            //             let mut sorted_chars: Vec<_> = chars.iter().collect();
            //             sorted_chars.sort_by(|lhs, rhs| rhs.cmp(lhs)); // a-z reverse sort
            //             let mut result = String::new();
            //             for ch in sorted_chars {
            println!("parents {} chars {:?}", parent, chars);
            for ch in chars {
                if visited.contains(&ch) {
                    continue;
                }
                visited.insert(ch.clone());
                visit(&ch, &char_map, &mut visited, &mut acc);
            }
        },
        None => {
            
        },
    }

    acc.push(parent.clone());
}

// fn walk_through_rec(parent: char, char_map: &HashMap<char, Vec<char>>, visited: &mut HashSet<char>) -> String {
// fn walk_through(char_map: &HashMap<char, Vec<char>>) -> String {
//     let mut visited = HashSet::<char>::new();
//     let mut result = String::new();
//     loop {
//         let mut keys: Vec<_> = char_map.keys().collect();
//         keys.sort_by(|lhs, rhs| rhs.cmp(lhs) ); // a-z reverse sort
//         let found = keys.iter().find(|ch| !visited.contains(&ch));
//         match found {
//             Some(ch) => {
//                 println!("Searching root {}", ch);
//                 visited.insert(*ch.clone());
//                 let walked_result = walk_through_rec(**ch, &char_map, &mut visited);
//                 result.push_str(&walked_result);
//                 result.push(*ch.clone());
//                 println!("Push walked nested result {} and top char {} result is now {}", &walked_result, ch, result);
//                 // result.push(**ch);
//             },
//             None => {
//                 println!("No chars left");
//                 break;
//             },
//         }
//     }

//     result.chars().rev().collect()
// }


// fn walk_through_rec(parent: char, char_map: &HashMap<char, Vec<char>>, visited: &mut HashSet<char>) -> String {

//     println!("Walking through {}", parent);
//     match char_map.get(&parent) {
//         Some(chars) => {
//             let mut sorted_chars: Vec<_> = chars.iter().collect();
//             sorted_chars.sort_by(|lhs, rhs| rhs.cmp(lhs)); // a-z reverse sort
//             let mut result = String::new();
//             for ch in sorted_chars {
//                 if visited.contains(&ch) {
//                     // println!("Already visited {}", &ch);
//                     continue;
//                 }

//                 let walked_result = walk_through_rec(ch.clone(), &char_map, visited);
//                 visited.insert(ch.clone());
//                 // println!("Pushing {} to {}", ch, parent);
//                 result.push_str(&walked_result);
//                 result.push(ch.clone());

//                 // if result != "" {
//                 //     println!("char {} parent {}", &ch, &parent);
//                 // }
                
//                 println!("Nested push walked nested result {} and current char {} result is now {} parent {}", &walked_result, ch, result, &parent);
//                 // acc.push(ch.clone());
//             }

//             result
//         },
//         None => {
//             // TODO: Return optional
//             String::new()
//         }
//     }

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
