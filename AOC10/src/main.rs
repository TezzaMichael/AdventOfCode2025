fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Result: {}", firstPart(&input));
    //println!("Result: {}", secondPart(&input));
}

fn firstPart(input: &String) -> usize {
    let mut total = 0;

    for line in input.lines() {
        let start = line.find('[').unwrap() + 1;
        let end = line[start..].find(']').unwrap() + start;
        let target_str = &line[start..end];
        let target: Vec<char> = target_str.chars().collect();

        let mut buttons = Vec::new();
        for part in line.split_whitespace() {
            if part.starts_with('(') && part.ends_with(')') {
                let inner = &part[1..part.len() - 1];
                let mut indices = Vec::new();
                for num in inner.split(',') {
                    indices.push(num.parse::<usize>().unwrap());
                }
                buttons.push(indices);
            }
        }

        if let Some(min) = min_presses_bruteforce(&target, &buttons) {
            total += min;
        }
    }

    total
}

fn secondPart(input: &String) -> usize {
    println!("Work in progress...");
    0
}

fn generate_combinations_rec(n: usize, pos: usize, current: &mut Vec<bool>, result: &mut Vec<Vec<bool>>) {
    if pos == n {
        result.push(current.clone());
        return;
    }

    current[pos] = false;
    generate_combinations_rec(n, pos + 1, current, result);

    current[pos] = true;
    generate_combinations_rec(n, pos + 1, current, result);
}

fn generate_combinations(n: usize) -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    let mut current = vec![false; n];
    generate_combinations_rec(n, 0, &mut current, &mut result);
    result
}

fn min_presses_bruteforce(target: &Vec<char>, buttons: &Vec<Vec<usize>>) -> Option<usize> {
    let n = buttons.len();
    let all_combos = generate_combinations(n);

    let mut best: Option<usize> = None;

    for combo in all_combos {
        let mut state = vec!['.'; target.len()];
        let mut presses = 0;

        for b in 0..n {
            if combo[b] {
                presses += 1;
                for &idx in &buttons[b] {
                    state[idx] = if state[idx] == '.' { '#' } else { '.' };
                }
            }
        }

        if state == *target {
            if let Some(prev) = best {
                best = Some(prev.min(presses));
            } else {
                best = Some(presses);
            }
        }
    }

    best
}