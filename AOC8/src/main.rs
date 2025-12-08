fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Result: {}", firstPart(&input));
    println!("Result: {}", secondPart(&input));
}

fn firstPart(input: &String) -> usize {
    let mut points: Vec<(isize,isize,isize)> = Vec::new();
    for line in input.lines() {
        let coords: Vec<isize> = line.split(",").map(|s| s.parse().unwrap()).collect();
        points.push((coords[0], coords[1], coords[2]));
    }

    let n = points.len();
    let mut edges: Vec<(isize, usize, usize)> = Vec::new();

    for i in 0..n {
        for j in i+1..n {
            let d = distance(points[i], points[j]);
            edges.push((d, i, j));
        }
    }

    edges.sort_by_key(|e| e.0);

    let mut comp: Vec<usize> = Vec::new();
    for i in 0..n {
        comp.push(i);
    }

    let limit = 1000.min(edges.len());

    for k in 0..limit {
        let (_, a, b) = edges[k];
        union(a, b, &mut comp);
    }

    let mut count = std::collections::HashMap::<usize, usize>::new();

    for c in comp {
        *count.entry(c).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = count.values().cloned().collect();

    sizes.sort_by(|a,b| b.cmp(a));

    sizes[0] * sizes[1] * sizes[2]
}

fn secondPart(input: &String) -> isize {
    let mut points: Vec<(isize,isize,isize)> = Vec::new();
    for line in input.lines() {
        let coords: Vec<isize> = line.split(",").map(|s| s.parse().unwrap()).collect();
        points.push((coords[0], coords[1], coords[2]));
    }

    let n = points.len();

    let mut edges: Vec<(isize, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i+1..n {
            let d = distance(points[i], points[j]);
            edges.push((d, i, j));
        }
    }

    edges.sort_by_key(|e| e.0);

    let mut comp: Vec<usize> = Vec::new();
    for i in 0..n { 
        comp.push(i); 
    }

    let mut components = n;
    let mut last_pair = (0, 0);

    for (_, a, b) in edges {
        let ca = comp[a];
        let cb = comp[b];
        if ca != cb {
            union(a, b, &mut comp);
            components -= 1;
            last_pair = (a, b);
            if components == 1 {
                break;
            }
        }
    }

    points[last_pair.0].0 * points[last_pair.1].0
}

fn distance(a: (isize,isize,isize), b: (isize,isize,isize)) -> isize {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx*dx + dy*dy + dz*dz
}

fn union(a: usize, b: usize, comp: &mut Vec<usize>) {
    let ca = comp[a];
    let cb = comp[b];

    if ca == cb {
        return; 
    }

    for i in 0..comp.len() {
        if comp[i] == cb {
            comp[i] = ca;
        }
    }
}

