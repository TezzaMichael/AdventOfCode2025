fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Result: {}", firstPart(&input));
}

fn firstPart(input: &String) -> usize {
    let (region_names, region_coords, shapes) = parse(input);

    println!("Parsed {} regions and {} shapes.", region_names.len(), shapes.len());
    for i in 0..region_names.len() {
        println!("Region {} at coords {:?}", region_names[i], region_coords[i]);
    }

    let mut count = 0;

    for i in 0..region_names.len() {
        if check_area(&shapes, &region_names[i], &region_coords[i]) {
            count += 1;
        }
    }

    count
}

fn check_area(shapes: &Vec<Vec<char>>, name_region: &str, coords_region: &Vec<&str>) -> bool {
    let mut area_shape = 0;

    let parts: Vec<&str> = name_region.split('x').collect();
    let area_region = parts[0].parse::<usize>().unwrap() * parts[1].parse::<usize>().unwrap();

    for s in 0..coords_region.len() {
        area_shape += calculate_area(&shapes[s], &coords_region[s].parse::<usize>().unwrap());
    }

    area_shape <= area_region
}

fn calculate_area(shape: &Vec<char>, n: &usize) -> usize {
    let mut area = 0;
    for c in shape.iter() {
        if *c == '#' {
            area += 1;
        }
    }
    area * n
}

fn parse(input: &String) -> (Vec<String>, Vec<Vec<&str>>, Vec<Vec<char>>) {
    let mut region_names = Vec::new();
    let mut region_coords = Vec::new();
    let mut shapes = Vec::new();
    let mut shape = Vec::new();

    for line in input.lines() {
        if line.contains('.') || line.contains('#') {
            for c in line.chars() {
                shape.push(c);
            }
            continue;
        }

        if !shape.is_empty() {
            shapes.push(shape);
            shape = Vec::new();
        }

        if line.contains('x') {
            let parts = line.split(':').collect::<Vec<&str>>();
            let name = parts[0].trim().to_string();
            let coords_part = parts[1].trim();
            let coords_vec = coords_part.split_whitespace().collect::<Vec<&str>>();

            region_names.push(name);
            region_coords.push(coords_vec);
        }
    }

    (region_names, region_coords, shapes)
}