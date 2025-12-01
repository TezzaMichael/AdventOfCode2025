fn main() {
    let mut num: i64 = 50;
    let mut zero_count: u64 = 0;
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    for line in input.lines() {
        println!("Current num: {}", num);
        println!("Processing line: {}", line);
        println!("Zero count: {}", zero_count);
        println!("-----------------------");
        
        if line.contains("L"){
            if num == 0 {
                num = 100;
            }

            num = num - line[1..].parse::<i64>().unwrap();
            while num < 0 {
                num = num + 100;
                zero_count += 1;
            }
            
        }else{
            if num == 100 {
                num = 0;
            }

            num = num + line[1..].parse::<i64>().unwrap();
            while num > 100 {

                num = num - 100;
                zero_count += 1;

            }
            if num == 100 {
                num = 0;
            }
        }
        if num >= 100 || num < 0 {
            println!("Error: num exceeded");
        }
        if num == 0 {
            zero_count += 1;
        }

    }
    println!("Result: {}", zero_count);
}
