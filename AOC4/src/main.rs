fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Result: {}", firstPart(&input));
    println!("Result: {}", secondPart(&input));
    
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn firstPart(input: &String) -> i64 {
    let mut sum: i64 = 0;
    // initialize a matrix di n elements chars
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut f_matrix: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            println!("Char: {}", c);
            row.push(c);
        }
        matrix.push(row);
    }
    f_matrix = matrix.clone();

    for i in 0..matrix.len() {
        let mut count = 0;
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '@' {
                count = 0;
                if i == 0 {
                    if j == 0 || j == matrix[i].len() - 1 {
                        sum += 1;
                        continue;
                    }
                    if matrix[i+1][j] == '@' {
                        count += 1;
                    }
                    if matrix[i][j-1] == '@' {
                        count += 1;
                    }
                    if matrix[i][j+1] == '@' {
                        count += 1;
                    }
                    if matrix[i+1][j+1] == '@' {
                        count += 1;
                    }
                    if matrix[i+1][j-1] == '@' {
                        count += 1;
                    }
                }else if i == matrix.len() - 1 {
                    if j == 0 || j == matrix[i].len() - 1 {
                        sum+=1;
                        continue;
                    }
                    if matrix[i-1][j] == '@' {
                        count += 1;
                    }
                    if matrix[i][j-1] == '@' {
                        count += 1;
                    }
                    if matrix[i][j+1] == '@' {
                        count += 1;
                    }
                    if matrix[i-1][j+1] == '@' {
                        count += 1;
                    }
                    if matrix[i-1][j-1] == '@' {
                        count += 1;
                    }
                }else if j == 0 {
                    if matrix[i-1][j] == '@' {
                        count += 1;
                    }
                    if matrix[i+1][j] == '@' {
                        count += 1;
                    }
                    if matrix[i][j+1] == '@' {
                        count += 1;
                    }
                    if matrix[i-1][j+1] == '@' {
                        count += 1;
                    }
                    if matrix[i+1][j+1] == '@' {
                        count += 1;
                    }
                }else if j == matrix[i].len() - 1 {
                    if matrix[i-1][j] == '@' {
                        count += 1;
                    }
                    if matrix[i+1][j] == '@' {
                        count += 1;
                    }
                    if matrix[i][j-1] == '@' {
                        count += 1;
                    }
                    if matrix[i-1][j-1] == '@' {
                        count += 1;
                    }
                    if matrix[i+1][j-1] == '@' {
                        count += 1;
                    }
                }else {
                    if matrix[i-1][j] == '@' {
                        count += 1;
                    }
                    if matrix[i+1][j] == '@' {
                        count += 1;
                    }
                    if matrix[i][j-1] == '@' {
                        count += 1;
                    }
                    if matrix[i][j+1] == '@' {
                        count += 1;
                    }
                    if matrix[i-1][j+1] == '@' {
                        count += 1;
                    }
                    if matrix[i-1][j-1] == '@' {
                        count += 1;
                    }
                    if matrix[i+1][j+1] == '@' {
                        count += 1;
                    }
                    if matrix[i+1][j-1] == '@' {
                        count += 1;
                    }
                }
            
                if count < 4 {
                    sum += 1;
                    f_matrix[i][j] = 'X';
                    count = 0;
                }
            }
            
            

            
        }
    }
    print_matrix(&f_matrix);

    sum
}

fn secondPart(input: &String) -> i64 {
    let mut sum: i64 = 1;
    // initialize a matrix di n elements chars
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut f_matrix: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            //println!("Char: {}", c);
            row.push(c);
        }
        matrix.push(row);
    }

    f_matrix = matrix.clone();
    let mut sum_total: i64 = 0;
    while sum != 0{
        sum = 0;

        for i in 0..matrix.len() {
            let mut count = 0;
            for j in 0..matrix[i].len() {
                if matrix[i][j] == '@' {
                    count = 0;
                    if i == 0 {
                        if j == 0 || j == matrix[i].len() - 1 {
                            sum += 1;
                            f_matrix[i][j] = 'X';   
                            continue;
                        }
                        if matrix[i+1][j] == '@' {
                            count += 1;
                        }
                        if matrix[i][j-1] == '@' {
                            count += 1;
                        }
                        if matrix[i][j+1] == '@' {
                            count += 1;
                        }
                        if matrix[i+1][j+1] == '@' {
                            count += 1;
                        }
                        if matrix[i+1][j-1] == '@' {
                            count += 1;
                        }
                    }else if i == matrix.len() - 1 {
                        if j == 0 || j == matrix[i].len() - 1 {
                            sum+=1;
                            f_matrix[i][j] = 'X';
                            continue;
                        }
                        if matrix[i-1][j] == '@' {
                            count += 1;
                        }
                        if matrix[i][j-1] == '@' {
                            count += 1;
                        }
                        if matrix[i][j+1] == '@' {
                            count += 1;
                        }
                        if matrix[i-1][j+1] == '@' {
                            count += 1;
                        }
                        if matrix[i-1][j-1] == '@' {
                            count += 1;
                        }
                    }else if j == 0 {
                        if matrix[i-1][j] == '@' {
                            count += 1;
                        }
                        if matrix[i+1][j] == '@' {
                            count += 1;
                        }
                        if matrix[i][j+1] == '@' {
                            count += 1;
                        }
                        if matrix[i-1][j+1] == '@' {
                            count += 1;
                        }
                        if matrix[i+1][j+1] == '@' {
                            count += 1;
                        }
                    }else if j == matrix[i].len() - 1 {
                        if matrix[i-1][j] == '@' {
                            count += 1;
                        }
                        if matrix[i+1][j] == '@' {
                            count += 1;
                        }
                        if matrix[i][j-1] == '@' {
                            count += 1;
                        }
                        if matrix[i-1][j-1] == '@' {
                            count += 1;
                        }
                        if matrix[i+1][j-1] == '@' {
                            count += 1;
                        }
                    }else {
                        if matrix[i-1][j] == '@' {
                            count += 1;
                        }
                        if matrix[i+1][j] == '@' {
                            count += 1;
                        }
                        if matrix[i][j-1] == '@' {
                            count += 1;
                        }
                        if matrix[i][j+1] == '@' {
                            count += 1;
                        }
                        if matrix[i-1][j+1] == '@' {
                            count += 1;
                        }
                        if matrix[i-1][j-1] == '@' {
                            count += 1;
                        }
                        if matrix[i+1][j+1] == '@' {
                            count += 1;
                        }
                        if matrix[i+1][j-1] == '@' {
                            count += 1;
                        }
                    }
                
                    if count < 4 {
                        sum += 1;
                        f_matrix[i][j] = 'X';
                        count = 0;
                    }
                }
                
                

                
            }
        }
        sum_total += sum;
        println!("Total count: {}", sum);
        print_matrix(&f_matrix);
        matrix = f_matrix.clone();

    }
    
    sum_total
}