fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    println!("Result: {}", firstPart(&input));
    println!("Result: {}", secondPart(&input));
}

fn firstPart(input: &String) -> i64 {
    let mut sum: i64 = 0;
    for line in input.split(',') {
        let mut firstId = line.split('-').next().unwrap().parse::<i64>().unwrap();
        let lastId = line.split('-').nth(1).unwrap().parse::<i64>().unwrap();
        while firstId <= lastId {
            if firstId.to_string().len() % 2 == 0 {
                println!("Checking ID: {}", firstId);
                let firstHalf = firstId.to_string()[0..(firstId.to_string().len()/2)].to_string();
                let secondHalf = firstId.to_string()[(firstId.to_string().len()/2)..].to_string();
                if firstHalf == secondHalf {
                    sum += firstId;
                    println!("Found matching ID: {}", firstId);
                }
            }
            firstId += 1;
            
        } 
        
    }
    sum

}

fn secondPart(input: &String) -> i64 {
    let mut sum: i64 = 0;
    for line in input.split(',') {
        let mut firstId = line.split('-').next().unwrap().parse::<i64>().unwrap();
        let lastId = line.split('-').nth(1).unwrap().parse::<i64>().unwrap();
        
        while firstId <= lastId {
            let s = firstId.to_string();
            let mut isMatch = false;
            
            for pattern_len in 1..=s.len() / 2 {

                if s.len() % pattern_len == 0 {
                    let pattern = &s[0..pattern_len];
                    let mut all_match = true;
                    
                    for i in (pattern_len..s.len()).step_by(pattern_len) {
                        if &s[i..i + pattern_len] != pattern {
                            all_match = false;
                            break;
                        }
                    }
                    
                    if all_match {
                        isMatch = true;
                        break;
                    }
                }
            }
            
            if isMatch {
                sum += firstId;
                println!("Found matching ID: {}", firstId);
            }
            
            firstId += 1;
        }
    }
    sum
}
