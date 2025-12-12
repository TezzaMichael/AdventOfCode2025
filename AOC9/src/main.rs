use std::collections::HashSet;

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
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

fn secondPart(input: &String) -> usize {
    let mut points = Vec::<(usize, usize)>::new();
    for line in input.lines() {
        let (x, y) = line.split_once(",").unwrap();
        points.push((x.parse().unwrap(), y.parse().unwrap()));
    }
    
    let mut max_area = 0;
    
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            
            if p1.0 == p2.0 || p1.1 == p2.1 {
                continue; 
            }
        
            let min_x = p1.0.min(p2.0);
            let max_x = p1.0.max(p2.0);
            let min_y = p1.1.min(p2.1);
            let max_y = p1.1.max(p2.1);
            
            if rectangle_valid(&points, min_x, max_x, min_y, max_y) {
                let rect_area = area(p1, p2);
                if rect_area > max_area {
                    max_area = rect_area;
                }
            }
        }
    }
    
    max_area
}

fn rectangle_valid(points: &Vec<(usize, usize)>, min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> bool {
    for &(x, y) in &[(min_x, min_y), (min_x, max_y), (max_x, min_y), (max_x, max_y)] {
        if !valid_point(points, x, y) {
            return false;
        }
    }
    
    let step_y = ((max_y - min_y) / 100).max(1);
    for y in (min_y..=max_y).step_by(step_y) {
        if !valid_point(points, min_x, y) {
            return false; 
        }
        if !valid_point(points, max_x, y) {
            return false;
        }
    }
    
    let step_x = ((max_x - min_x) / 100).max(1);
    for x in (min_x..=max_x).step_by(step_x) {
        if !valid_point(points, x, min_y) {
            return false;
        }
        if !valid_point(points, x, max_y) {
            return false;
        }
    }
    
    let n = points.len();
    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n];
        
        if interior(p1, p2, min_x, max_x, min_y, max_y) {
            return false;
        }
    }
    
    let center_x = (min_x + max_x) / 2;
    let center_y = (min_y + max_y) / 2;
    if !valid_point(points, center_x, center_y) {
        return false;
    }
    
    true
}

fn interior(p1: (usize, usize), p2: (usize, usize), min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> bool {
    if p1.0 == p2.0 {
        let x = p1.0;
        let seg_min_y = p1.1.min(p2.1);
        let seg_max_y = p1.1.max(p2.1);
        
        if x == min_x || x == max_x {
            return false;
        }
        
        if x > min_x && x < max_x {
            if seg_max_y > min_y && seg_min_y < max_y {
                return true;
            }
        }
        
        return false;
    }
    
    if p1.1 == p2.1 {
        let y = p1.1;
        let seg_min_x = p1.0.min(p2.0);
        let seg_max_x = p1.0.max(p2.0);
        
        if y == min_y || y == max_y {
            return false;
        }
        
        if y > min_y && y < max_y {
            if seg_max_x > min_x && seg_min_x < max_x {
                return true;
            }
        }
        
        return false;
    }
    
    false
}

fn valid_point(points: &Vec<(usize, usize)>, x: usize, y: usize) -> bool {
    border(points, x, y) || polygon_inside(points, x, y)
}

fn border(points: &Vec<(usize, usize)>, x: usize, y: usize) -> bool {
    let n = points.len();
    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n];
        
        if p1.0 == p2.0 && p1.0 == x {
            let min_y = p1.1.min(p2.1);
            let max_y = p1.1.max(p2.1);
            if y >= min_y && y <= max_y {
                return true;
            }
        } else if p1.1 == p2.1 && p1.1 == y {
            let min_x = p1.0.min(p2.0);
            let max_x = p1.0.max(p2.0);
            if x >= min_x && x <= max_x {
                return true;
            }
        }
    }
    false
}

fn polygon_inside(points: &Vec<(usize, usize)>, x: usize, y: usize) -> bool {
    let mut inside = false;
    let n = points.len();
    
    let px = x as f64;
    let py = y as f64;
    
    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n];
        
        let x1 = p1.0 as f64;
        let y1 = p1.1 as f64;
        let x2 = p2.0 as f64;
        let y2 = p2.1 as f64;
        
        if ((y1 > py) != (y2 > py)) && (px < (x2 - x1) * (py - y1) / (y2 - y1) + x1) {
            inside = !inside;
        }
    }
    
    inside
}

fn area(point1: (usize, usize), point2: (usize, usize)) -> usize {
    ((point1.0 as isize - point2.0 as isize).abs() as usize + 1) * 
    ((point1.1 as isize - point2.1 as isize).abs() as usize + 1)
}