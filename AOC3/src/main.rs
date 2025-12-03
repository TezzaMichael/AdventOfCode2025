fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Result: {}", firstPart(&input));
    println!("Result: {}", secondPart(&input));
    

}

fn firstPart(input: &String) -> i64 {
    let mut sum: i64 = 0;
    for line in input.lines() {
        let mut max = 9;
        let mut first_num = 0;
        let mut second_num = 0;
        let mut punt_pos = 0;
        println!("Line: {}", line);
        while max >= 0 && first_num == 0 {
            for i in 0..line.len() - 1 {
                if line.chars().nth(i).unwrap().to_digit(10).unwrap() as i64 == max {
                    first_num = max;
                    punt_pos = i;
                    break
                }
            }
            max -= 1;
        }
        max = 9;
        while max >= 0 && second_num == 0 {
            for i in punt_pos + 1..line.len() {
                if line.chars().nth(i).unwrap().to_digit(10).unwrap() as i64 == max {
                    second_num = max;
                    sum += first_num * 10 + second_num;
                    println!("Current sum: {}", sum);
                    break;
                }
            }
            max -= 1;
        }
    }
    sum

}

fn secondPart(input: &String) -> i64 {
    let mut sum: i64 = 0;
    for line in input.lines() {
        let mut max = 9;
        let mut list: Vec<i64> = vec![];
        let mut index = 0;
        while max >= 0 {
            for i in index..line.len() - (11 - list.len()) {
                if line.chars().nth(i).unwrap().to_digit(10).unwrap() as i64 == max {
                    list.push(max);
                    println!("Found digit {}: {}", list.len(),max);
                    index = i + 1;
                    max = 10;
                    break;
                }
            }
            if list.len() == 12 {
                let mut number: i64 = 0;
                for n in &list {
                    number = number * 10 + n;
                }
                println!("Found number: {}", number);
                sum += number;
                println!("Current sum: {}", sum);
                break;
            }

            max -= 1;
        }
    }
    sum
}