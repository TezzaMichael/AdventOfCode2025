fn main() {
    let input = std::fs::read_to_string("input_test.txt").expect("Failed to read input.txt");
    println!("Result: {}", firstPart(&input));
    println!("Result: {}", secondPart(&input));
}

fn firstPart(input: &String) -> i64 {
    let mut sum: i64 = 0;
    let mut switch = false;
    let mut ranges: Vec<(i64,i64)> = vec![];
    let mut number: Vec<i64> = vec![];
    for line in input.lines() {
        if line == "" {
            switch = true;
            continue;
        }
        if switch {
            let num: i64 = line.parse().unwrap();
            number.push(num);
        }else{
            let parts: Vec<&str> = line.split('-').collect();
            let start: i64 = parts[0].parse().unwrap();
            let end: i64 = parts[1].parse().unwrap();
            ranges.push((start,end));
        }
    } 
    for num in number.iter() {
        for range in ranges.iter() {
            if *num >= range.0 && *num <= range.1 {
                //println!("Number {} is in range {}-{}", num, range.0, range.1);
                sum+=1;
                break;
            }
        }
    }       
    sum
}

fn secondPart(input: &String) -> i64 {
    let mut sum: i64 = 0;
    let mut ranges: Vec<(i64,i64)> = vec![];
    for line in input.lines() {
        if line == "" {
            break;
        }
        let parts: Vec<&str> = line.split('-').collect();
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();
        ranges.push((start,end));  
    } 
    let mut final_ranges: Vec<(i64, i64)> = vec![];

    for r in ranges.into_iter() {
        let mut current = r;
        let mut splits = Vec::new();

        for &(s, e) in &final_ranges {

            // s -> current.0 -> current.1 -> e
            if current.0 >= s && current.1 <= e {
                continue;
            }

            // current.0 -> s -> current.1 -> e
            // current.0 -> s - 1
            if current.0 <= s && current.1 <= e && current.1 >= s {
                current.1 = s - 1; 
            }

            // s -> current.0 -> e -> current.1
            // e + 1 -> current.1
            if current.0 >= s && current.0 <= e && current.1 >= e {
                current.0 = e + 1;
            }

            // current.0 -> s -> e -> current.1
            if current.0 <= s && current.1 >= e {
                splits.push((current.0, s - 1));
                splits.push((e + 1, current.1));
                continue;
            }

            if current.0 > current.1 {
                continue;
            }
        }

        if splits.len() > 0 {
            for sp in splits {
                if sp.0 <= sp.1 {
                    final_ranges.push(sp);
                }
            }
        } else {
            final_ranges.push(current);
        }
    }

    for range in final_ranges.iter() {
        sum += range.1 - range.0 + 1;
    }

    sum
}
