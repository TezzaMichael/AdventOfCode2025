use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    //println!("Result: {}", firstPart(&input));
    println!("Result: {}", secondPart(&input));
}

fn firstPart(input: &String) -> usize {
    let graph = parse_graph(input);
    let mut count = 0;

    dfs("you", &graph, &mut count);

    count
}

fn secondPart(input: &String) -> usize {
    let mut count = 0;
    let mut memo = HashMap::new();
    let graph = parse_graph(input);
    dfs2("svr", &graph, &mut count, false, false, &mut memo);

    count
}

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (from, rest) = line.split_once(':').unwrap();
        let to: Vec<String> = rest.split_whitespace().map(|s| s.to_string()).collect();
        map.insert(from.trim().to_string(), to);
    }

    map
}

fn dfs(node: &str, graph: &HashMap<String, Vec<String>>, count: &mut usize) {
    if node == "out" {
        *count += 1;
        return;
    }

    if let Some(nexts) = graph.get(node) {
        for next in nexts {
            dfs(next, graph, count);
        }
    }
}

fn dfs2(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    count: &mut usize,
    fft: bool,
    dac: bool,
    mem: &mut HashMap<(String, bool, bool), usize>,
) {
    let mut fft = fft;
    let mut dac = dac;
    if node == "fft" {
        fft = true;
    }
    if node == "dac" {
        dac = true;
    }

    if node == "out" {
        if fft && dac {
            *count += 1;
        }
        return;
    }

    let val = (node.to_string(), fft, dac);
    
    //NO LOOP
    if let Some(&res) = mem.get(&val) {
        *count += res;
        return;
    }

    let before = *count;

    if let Some(nexts) = graph.get(node) {
        for next in nexts {
            dfs2(next, graph, count, fft, dac, mem);
        }
    }

    mem.insert(val, *count - before);
}