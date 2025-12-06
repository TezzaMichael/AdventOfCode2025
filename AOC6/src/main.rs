fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Result: {}", firstPart(&input));
    println!("Result: {}", secondPart(&input));  
}

fn firstPart(input: &str) -> i64 {
    let mut operations = Vec::new();
    let mut totals = Vec::new();
    for line in input.lines() {
        if line.contains("+") {
            for op in line.chars() {
                if op != ' ' {
                operations.push(op);
                }
            }
        } 
    }
    println!("{:?}", operations);
    let mut first_line = true;
    for line in input.lines() {
        if first_line {
            first_line = false;
            for n in line.split_whitespace() {
                if let Ok(num) = n.parse::<i64>() {
                    totals.push(num);
                }
            }
            continue;
        }
        let mut index = 0;
        for n in line.split_whitespace() {
            if let Ok(num) = n.parse::<i64>() {
                if operations[index] == '+' {
                    totals[index] += num;
                }else{
                    totals[index] *= num;
                }
                index += 1;
            }
        }
    }

    let mut sum = 0;
    for total in totals {
        sum += total;
    }
    
    sum
}

fn secondPart(input: &str) -> i64 {
    let mut operations = Vec::new();
    for line in input.lines() {
        if line.contains("+") {
            for op in line.chars() {
                if op == '+' || op == '*' {
                    operations.push(op);
                }
            }
        }
    }

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if line.contains("+") {
            continue;
        }
        matrix.push(line.chars().collect());
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut col_matrix: Vec<Vec<char>> = Vec::new();
    for i in 0..cols {
        let mut col = Vec::new();
        for j in 0..rows {
            col.push(matrix[j][i]);
        }
        col_matrix.push(col);
    }

    
    let mut col_to_op = vec!['?'; cols];
    let mut op_index = 0;
    let mut i = 0;

    while i < cols {
        if col_matrix[i].iter().all(|&c| c == ' ') {
            i += 1;
            continue;
        }

        let op = operations[op_index];

        while i < cols && !col_matrix[i].iter().all(|&c| c == ' ') {
            col_to_op[i] = op;
            i += 1;
        }

        op_index += 1;
    }

    let mut sum: i64 = 0;
    let mut i = 0;

    while i < cols {
        if col_matrix[i].iter().all(|&c| c == ' ') {
            i += 1;
            continue;
        }

        let op = col_to_op[i];

        let mut values: Vec<i64> = Vec::new();

        while i < cols && !col_matrix[i].iter().all(|&c| c == ' ') {
            let mut s = String::new();
            for j in 0..rows {
                if col_matrix[i][j] != ' ' {
                    s.push(col_matrix[i][j]);
                }
            }
            if !s.is_empty() {
                values.push(s.parse::<i64>().unwrap());
            }
            i += 1;
        }

        let block_result = if op == '+' {
            values.iter().sum::<i64>()
        } else {
            values.iter().product::<i64>()
        };

        sum += block_result;
    }

    sum 
}   