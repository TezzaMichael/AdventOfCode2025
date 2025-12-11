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
    println!("Work in progress...");
    0
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
