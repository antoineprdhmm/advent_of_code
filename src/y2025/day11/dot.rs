use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

use crate::read_input;

fn read_graph() -> HashMap<String, Vec<String>> {
    read_input(2025, 11)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let (device, outputs_str) = line.split_once(": ").unwrap();
            let outputs = outputs_str
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            (device.to_string(), outputs)
        })
        .collect()
}

pub fn generate_dot_file() {
    let graph = read_graph();

    // Collect all unique nodes
    let mut all_nodes_set: HashSet<String> = graph.keys().cloned().collect();
    all_nodes_set.extend(graph.values().flatten().cloned());
    let mut all_nodes: Vec<_> = all_nodes_set.into_iter().collect();
    all_nodes.sort();

    fn get_node_style(
        node: &str,
        graph: &HashMap<String, Vec<String>>,
    ) -> (&'static str, &'static str) {
        match node {
            "svr" | "you" => ("doubleoctagon", "pink"),
            "fft" | "dac" => ("doubleoctagon", "red"),
            "out" => ("doubleoctagon", "lightgreen"),
            _ if graph.get(node).map_or(0, |v| v.len()) > 1 => ("diamond", "lightyellow"),
            _ if !graph.contains_key(node) => ("ellipse", "lightpink"),
            _ => ("box", "lightblue"),
        }
    }

    let node_defs: Vec<String> = all_nodes
        .iter()
        .map(|node| {
            let (shape, color) = get_node_style(node, &graph);
            format!("        {node} [shape={shape}, color={color}, style=filled];")
        })
        .collect();

    let edges: Vec<String> = graph
        .iter()
        .flat_map(|(source, outputs)| {
            outputs
                .iter()
                .map(|target| format!("        {} -> {};", source, target))
                .collect::<Vec<_>>()
        })
        .collect();

    let dot_content = format!(
        r#"digraph Day11 {{
    // Graph settings
    rankdir=TB;
    splines=ortho;
    nodesep=0.5;
    ranksep=0.8;
    
    // Node defaults
    node [fontname="Helvetica", fontsize=12];
    edge [color=gray50];
    
    // Nodes
{}

    // Edges
{}
}}
"#,
        node_defs.join("\n"),
        edges.join("\n")
    );

    let output_path = "src/y2025/day11/day11.dot";
    let mut file = File::create(output_path).unwrap();
    file.write_all(dot_content.as_bytes()).unwrap();
    println!("Generated DOT file: {}", output_path);
}
