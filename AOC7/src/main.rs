fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    //println!("Result: {}", firstPart(&input));
    println!("Result: {}", secondPart(&input));
}

fn firstPart(input: &String) -> i64 {
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        matrix.push(line.chars().collect());
    }

    let mut count = 0;
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            if matrix[r][c] == 'S' {
                matrix[r+1][c] = '|';
            }else if matrix[r][c] == '^' {
                matrix[r][c-1] = '|';
                matrix[r][c+1] = '|';
                if matrix[r-1][c] == '|' {
                    count += 1;
                }
            }else if r > 0 {
                if matrix[r-1][c] == '|' {
                    matrix[r][c] = '|';
                }
            }
        }
    }

    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            print!("{}", matrix[r][c]);
        }
        println!();
    }
    
    count
}

fn secondPart(input: &String) -> i64 {
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for ch in line.chars() {
            row.push(ch);
        }
        matrix.push(row);
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut matrix2: Vec<Vec<i64>> = vec![vec![0; cols]; rows];

    let mut start_x = 0;
    for c in 0..cols {
        if matrix[0][c] == 'S' {
            start_x = c;
            break;
        }
    }
    matrix2[0][start_x] = 1;

    for r in 0..rows-1 {
        for c in 0..cols {
            let count = matrix2[r][c];
            if count == 0 {
                continue;
            }

            if matrix[r][c] == '^' {
                if c > 0 {
                    matrix2[r + 1][c - 1] += count;
                }
                if c + 1 < cols {
                    matrix2[r + 1][c + 1] += count;
                }
            } else {
                matrix2[r + 1][c] += count;
            }
        }
    }

    let mut total = 0;
    for c in 0..cols {
        total += matrix2[rows - 1][c];
    }

    total
}