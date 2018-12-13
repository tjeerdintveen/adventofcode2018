use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

type Id = usize;
type Distance = u32;
type Coordinate = Point;

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Default)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let mut f = File::open("input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let first_answer = solve_first(&contents);
    println!("first answer {}", first_answer);
    // let second_answer = solve_second(&contents);
    // println!("Second answer {}", second_answer);
}

fn solve_first(contents: &str) -> u32 {
    let points: Vec<_> = parse(&contents);
    let (rectangle, map) = generate_map(&points);
    largest_finite_area(&rectangle, &map)
}

fn print_map(map: &HashMap<Coordinate, Option<(Id, Distance)>>, width: &u32) {
    print!("\n");
    let mut points: Vec<(&Point, Option<&Id>)> = map
        .iter()
        .map(|(coordinate, values)| {
            if let Some((id, _)) = values {
                (coordinate, Some(id))
            } else {
                (coordinate, None)
            }
        })
        .collect();
    points.sort_by(|(lhs, _), (rhs, _)| {
        if lhs.y < rhs.y {
            Ordering::Less
        } else if lhs.y == rhs.y && lhs.x < rhs.x {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let grid: Vec<_> = points.chunks(*width as usize).collect();

    for &points in &grid {
        for (_coordinate, id) in points {
            if let Some(val) = id {
                print!("{}", val);
            } else {
                print!("x");
            }
        }
        print!("\n");
    }
}

// fn solve_second(contents: &str) -> usize {
//     0
// }

fn is_infinite_coordinate(coordinate: &Coordinate, rectangle: &Rectangle) -> bool {
    rectangle.top_left.x == coordinate.x
        || rectangle.top_left.y == coordinate.y
        || rectangle.bottom_right.x == coordinate.x
        || rectangle.bottom_right.y == coordinate.y
}

fn largest_finite_area(
    rectangle: &Rectangle,
    map: &HashMap<Coordinate, Option<(Id, Distance)>>,
) -> u32 {

    let mut disqualified_ids = HashSet::<Id>::new();

    let keys_values = map
        .iter()
        .filter_map(|(coordinate, value)| {
            value.and_then(|val| {

                if disqualified_ids.contains(&val.0) {
                    return None;
                }

                if is_infinite_coordinate(&coordinate, &rectangle) {
                    disqualified_ids.insert(val.0);
                    None
                } else {
                    Some(val.0)
                }
            })
        })
        .fold(HashMap::<Id, u32>::new(), |mut hashmap, id| {
            let entry = hashmap.entry(id).or_insert(0);
            *entry += 1;
            hashmap
        });

    keys_values
        .iter()
        .max_by_key(|(_key, &value)| value)
        .unwrap()
        .1
        .clone()
}

fn generate_map(points: &[Point]) -> (Rectangle, HashMap<Coordinate, Option<(Id, Distance)>>) {
    let rectangle = rectangle_from_points(&points);
    let mut map = HashMap::<Coordinate, Option<(Id, Distance)>>::new();
    let first_point = &points[0];
    // Generate coordinates, fill them with first point
    for x in 0..=rectangle.bottom_right.x + 1{
        for y in 0..=rectangle.bottom_right.y {
            let coordinate = Coordinate { x, y };
            let distance = distance_between_points(&first_point, &coordinate);
            map.insert(coordinate, Some((1, distance)));
        }
    }
    let clone = map.clone();

    // figure out which coordinate has equal distances, to decide to nullify a coordinate, or to help decide if the coordinate belongs to a different point
    let mut equal_distances = HashMap::<&Coordinate, Distance>::new();

    // Improve, filtering nils
    // TODO: Can we use zip and enumerate to generate ids and then collect into hashmap?
    // Update map with all other points
    for (id, point) in points.iter().skip(1).enumerate() {
        for (coordinate, _values) in &clone {
            map.entry(coordinate.clone())
                .and_modify(|values| {
                    match values {
                        Some((_current_id, current_distance)) => {
                            let distance = distance_between_points(&point, &coordinate);
                            if distance < *current_distance {
                                *values = Some((id + 2, distance));
                            } else if distance == *current_distance {
                                // clash, same distance for two points, nullify
                                equal_distances.insert(coordinate, distance);
                                *values = None;
                            }
                        },
                        None => {
                            // If a coordinate is emptied before, see if a new point can get a spot
                            if let Some(current_distance) = equal_distances.get(coordinate) {
                                let distance = distance_between_points(&point, &coordinate);
                                // new point has shorter distance than earlier emptying.
                                // Update the emptying dictionary (equal_distances)
                                if distance < *current_distance {
                                    // id + 2 because we skip the first point and the first point is 1
                                    *values = Some((id + 2, distance));
                                    equal_distances.insert(coordinate, distance);
                                }
                            }
                        },
                    }
                });
        }
    }

    // print_map(&map, &(&rectangle.bottom_right.x + 2));
    (rectangle, map)
}

fn distance_between_points(lhs: &Point, rhs: &Point) -> Distance {
    let horizontal = if lhs.x > rhs.x {
        lhs.x - rhs.x
    } else {
        rhs.x - lhs.x
    };

    let vertical = if lhs.y > rhs.y {
        lhs.y - rhs.y
    } else {
        rhs.y - lhs.y
    };

    horizontal + vertical
}

// Grows a rectangle from points
// assumes grid starts at 0,0
// (I still need to look into the From Trait)
fn rectangle_from_points(points: &[Point]) -> Rectangle {
    points
        .iter()
        .fold(Rectangle::default(), |mut rect, point| match point {
            Point { x, y } if x > &rect.bottom_right.x && y > &rect.bottom_right.y => {
                rect.bottom_right = point.clone();
                rect
            }
            Point { x, .. } if x > &rect.bottom_right.x => {
                rect.bottom_right.x = *x;
                rect
            }
            Point { y, .. } if y > &rect.bottom_right.y => {
                rect.bottom_right.y = *y;
                rect
            }
            _ => rect,
        })
}

fn parse(contents: &str) -> Vec<Point> {
    contents
        .lines()
        .map(|line: &str| {
            let strings: Vec<_> = line.split(',').collect();
            let x = strings[0].trim();
            let y = strings[1].trim();

            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect()
}

#[test]
fn test_generate_map_small() {
    let points = [
        Point { x: 1, y: 1 },
        Point { x: 4, y: 5 },
        Point { x: 102, y: 2 },
    ];

    let expected_points = 103 * 6; // (width times height) + 1 for both sides

    let (_rectangle, map) = generate_map(&points);
    assert_eq!(expected_points, map.len());
}

#[test]
fn test_largest_finite_area() {
    let points = [Point { x: 0, y: 0 }, Point { x: 1, y: 1 }];

    let expected_points = 103 * 6; // (width times height) + 1 for both sides

    let (rectangle, map) = generate_map(&points);
    let largest = largest_finite_area(&rectangle, &map);
    assert_eq!(2, largest);
}

// #[test]
// fn test_map_values() {
//     let points = [
//         Point { x: 1, y: 1 },
//         Point { x: 3, y: 1 },
//     ];

//     let map = generate_map(&points);

//     let mut values: Vec<_> = map.iter().collect();
//     values.sort_by(|(lhs, _), (rhs, _)| {
//         if lhs.y < rhs.y {
//             Ordering::Less
//         } else if lhs.y == rhs.y && lhs.x < rhs.x {
//             Ordering::Less
//         } else {
//             Ordering::Greater
//         }
//     });
//     println!("sorted_values is {:?}", values);
//     assert_eq!(2, map.len());
// }

#[test]
fn test_distance_between_points() {
    let lhs = Point { x: 1, y: 1 };
    let rhs = Point { x: 3, y: 3 };

    let distance = distance_between_points(&lhs, &rhs);
    assert_eq!(4, distance);

    let distance = distance_between_points(&rhs, &lhs);
    assert_eq!(4, distance);
}

#[test]
fn test_rectangle_from_points() {
    let points = [
        Point { x: 1, y: 1 },
        Point { x: 4, y: 5 },
        Point { x: 102, y: 2 },
    ];

    let rectangle = rectangle_from_points(&points);
    assert_eq!(0, rectangle.top_left.x);
    assert_eq!(0, rectangle.top_left.y);
    assert_eq!(102, rectangle.bottom_right.x);
    assert_eq!(5, rectangle.bottom_right.y);
}
