use std::collections::HashMap;
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
    let mut all_nodes: Vec<String> = graph.keys().cloned().collect();
    for outputs in graph.values() {
        for output in outputs {
            if !all_nodes.contains(output) {
                all_nodes.push(output.clone());
            }
        }
    }
    all_nodes.sort();

    let node_defs: Vec<String> = all_nodes
        .iter()
        .map(|node| {
            if node == "svr" || node == "you" {
                format!(
                    "        {} [shape=doubleoctagon, color=pink, style=filled];",
                    node
                )
            } else if node == "fft" || node == "dac" {
                format!(
                    "        {} [shape=doubleoctagon, color=red, style=filled];",
                    node
                )
            } else if node == "out" {
                format!(
                    "        {} [shape=doubleoctagon, color=lightgreen, style=filled];",
                    node
                )
            } else if graph.get(node).map(|v| v.len()).unwrap_or(0) > 1 {
                format!(
                    "        {} [shape=diamond, color=lightyellow, style=filled];",
                    node
                )
            } else if !graph.contains_key(node) {
                format!(
                    "        {} [shape=ellipse, color=lightpink, style=filled];",
                    node
                )
            } else {
                format!(
                    "        {} [shape=box, color=lightblue, style=filled];",
                    node
                )
            }
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
