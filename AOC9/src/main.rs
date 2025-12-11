fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    //println!("Result: {}", firstPart(&input));
    println!("Result: {}", secondPart(&input));
}

fn firstPart(input: &String) -> usize {
    let mut points = Vec::<(usize, usize)>::new();
    for line in input.lines() {
        let (x, y) = line.split_once(",").unwrap();
        points.push((x.parse().unwrap(), y.parse().unwrap()));
    }
    let mut max_area = 0;
    for point in &points {
        for point2 in &points {
            let area = area(*point, *point2);
            //println!("Area between {:?} and {:?} is {}", point, point2, area);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

use std::collections::{HashSet, HashMap};

fn secondPart(input: &String) -> usize {
    println!("Work in progress...");
    0
}


fn area(point1: (usize, usize), point2: (usize, usize)) -> usize {
    ((point1.0 as isize - point2.0 as isize).abs() as usize +1) * ((point1.1 as isize - point2.1 as isize).abs() as usize +1)
}