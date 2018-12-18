use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;

type Duration = u32;

struct Pool {
    count: usize,
    workers: HashMap<char, Duration>,
    queue: VecDeque<char>,
}

impl Pool {
    fn is_empty(&self) -> bool {
        self.queue.is_empty() && self.workers.is_empty()
    }

    fn push(&mut self, ch: char) {
        self.queue.push_back(ch);
        self.add_next_job();
    }

    fn add_next_job(&mut self) {
        if self.is_occupied() {
            return;
        }
        if let Some(ch) = self.queue.pop_front() {
            self.add_job(ch);
        }
    }

    fn add_job(&mut self, ch: char) {
        if self.is_occupied() {
            panic!("Can't add jobs while occupied");
        }
        let duration = duration_for_char(ch);
        self.workers.insert(ch, duration);
    }

    fn tick(&mut self) -> Option<char> {
        for value in self.workers.values_mut() {
            *value -= 1;
        }

        let removed = self
            .workers
            .iter()
            .find(|(_k, v)| **v == 0)
            .map(|(k, _v)| *k);

        self.workers.retain(|_ch, duration| *duration >= 1);

        let diff = self.count - self.workers.len();
        (0..diff).for_each(|_| {
            if !self.is_occupied() {
                self.add_next_job();
            }
        });
        removed
    }

    fn is_occupied(&self) -> bool {
        self.workers.len() == self.count
    }
}

fn main() {
    let mut f = File::open("input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let char_map = parse(&contents);
    let first_result = solve_first(&char_map);
    let second_result = solve_second(&char_map);
    println!("First result is {:?}", first_result);
    println!("Second result is {}", second_result);
}

fn solve_second(char_map: &HashMap<char, Vec<char>>) -> u32 {
    let mut seconds_passed = 0;

    let mut indegrees = indegrees(&char_map);

    let roots = find_roots(&char_map, &indegrees);

    let queue = roots.iter().fold(VecDeque::<char>::new(), |mut q, value| {
        q.push_back(*value);
        q
    });

    // 2 workers foe example input
    let mut pool = Pool {
        count: 5,
        workers: HashMap::<_, _>::new(),
        queue,
    };

    while !pool.is_empty() {
        let finished: Option<char> = pool.tick();

        seconds_passed += 1;
        if let Some(ch) = finished {
            // char is finished, add neighbors
            if let Some(neighbors) = &char_map.get(&ch) {
                for neighbor in *neighbors {
                    let entry = indegrees.entry(*neighbor).or_insert(0);
                    *entry -= 1;

                    if *entry == 0 {
                        pool.push(neighbor.clone());
                    }
                }
            }
        }
    }

    seconds_passed - 1 // ignoring last loop
}

fn duration_for_char(ch: char) -> u32 {
    // ch as u32 - 64 for example input
    ch as u32 - 4
}

fn solve_first(char_map: &HashMap<char, Vec<char>>) -> String {
    let mut result = String::new();
    let mut indegrees = indegrees(&char_map);

    let roots = find_roots(&char_map, &indegrees);
    let mut queue: String = roots.iter().collect();

    while !queue.is_empty() {
        // sort string
        let mut chars: Vec<char> = queue.chars().collect();
        chars.sort_by(|a, b| b.cmp(a));
        queue = chars.iter().collect();

        let popped = queue.pop().unwrap();
        result.push(popped);
        if let Some(neighbors) = &char_map.get(&popped) {
            for neighbor in *neighbors {
                let entry = indegrees.entry(*neighbor).or_insert(0);
                *entry -= 1;

                if *entry == 0 {
                    queue.insert(0, neighbor.clone());
                }
            }
        }
    }
    result
}

fn find_roots(char_map: &HashMap<char, Vec<char>>, indegrees: &HashMap<char, u32>) -> Vec<char> {
    let mut roots = Vec::<char>::new();
    char_map.keys().for_each(|k| {
        if !&indegrees.contains_key(&k) {
            roots.push(k.clone());
        }
    });
    roots.sort_by(|lhs, rhs| rhs.cmp(lhs));
    roots
}

fn indegrees(char_map: &HashMap<char, Vec<char>>) -> HashMap<char, u32> {
    let mut indegrees = HashMap::<char, u32>::new();

    for values in char_map.values() {
        for ch in values {
            let entry = indegrees.entry(*ch).or_insert(0);
            *entry += 1;
        }
    }

    indegrees
}

fn parse(contents: &str) -> HashMap<char, Vec<char>> {
    contents
        .lines()
        .map(|str| (str.chars().nth(5).unwrap(), str.chars().nth(36).unwrap()))
        .fold(
            HashMap::<char, Vec<char>>::new(),
            |mut char_map, (lhs, rhs)| {
                char_map
                    .entry(lhs)
                    .and_modify(|chars| {
                        chars.push(rhs);
                    })
                    .or_insert_with(|| vec![rhs]);
                char_map
            },
        )
}
