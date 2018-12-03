use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::result::Result;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Claim {
    id: i32,
    top: i32,
    left: i32,
    width: i32,
    height: i32,
}

fn main() {
    let mut f = File::open("input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    first_part(&contents);
    second_part(&contents);
}

fn parse_string(string: &str) -> Result<Claim, ParseIntError> {
    let splitted: Vec<_> = string.split(|c: char| {
        return c == '#' || c == ' ' || c == 'x' || c == ',' || c == ':'
    }).collect();

    let id = splitted[1].parse()?;
    let top = splitted[4].parse()?;
    let left = splitted[3].parse()?;
    let width = splitted[6].parse()?;
    let height = splitted[7].parse()?;

    let claim = Claim { id: id, top: top, left: left, width: width, height: height };

    Ok(claim)
}

fn points_from_claim(claim: &Claim) -> Vec<Point> {
    let mut points = Vec::<_>::new();
    for x in claim.left..claim.left + claim.width {
        for y in claim.top..claim.top + claim.height {
            let point = Point { x: x, y: y };
            points.push(point);
        }
    }

    // println!("claim is {:?} points is {:?}\n", claim, points);
    return points
}

fn first_part(contents: &str) {
    let claims: Vec<_> = contents
        .lines()
        .map(|s| parse_string(s))
        .filter_map(|s| s.ok())
        .collect();

    let mut charmap = HashMap::<Point, i32>::new();
    for claim in claims {
        let points = points_from_claim(&claim);

        for point in points {
            let entry = charmap.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    let total = charmap.iter().filter(|&(_, v)| v > &1 ).count();
    println!("total {}", total);
}

fn second_part(contents: &str) {
    // Parse claims
    let claims: Vec<_> = contents
        .lines()
        .map(|s| parse_string(s))
        .filter_map(|s| s.ok())
        .collect();

    let mut charmap = HashMap::<Point, i32>::new();
    // populate charmap
    for claim in &claims {
        let points = points_from_claim(&claim);
        for point in points {
            let entry = charmap.entry(point).or_insert(0);
            *entry += 1;
        }
    }


    // find unclaimed claim
    for claim in &claims {
        let points = points_from_claim(&claim);

        let mut is_unclaimed = true;

        for point in points {
            let entry = charmap.entry(point).or_insert(0);
            if *entry > 1 {
                is_unclaimed = false;
            }
        }

        if is_unclaimed {
            println!("UNCLAIMED {:?}", claim);
        }

    }

}
