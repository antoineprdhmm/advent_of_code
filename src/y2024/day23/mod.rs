use std::collections::{HashMap, HashSet};

use crate::read_input;

type Graph = HashMap<String, HashSet<String>>;

fn build_graph() -> Graph {
    let mut graph: Graph = HashMap::new();

    read_input(2024, 23)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let (n1, n2) = line.split_once("-").unwrap();
            (n1.to_string(), n2.to_string())
        })
        .for_each(|(n1, n2)| {
            graph.entry(n1.clone()).or_default().insert(n2.clone());
            graph.entry(n2).or_default().insert(n1);
        });

    graph
}

pub fn run_part_1() {
    let graph = build_graph();

    let mut unique_sets = HashSet::new();

    for a in graph.keys() {
        if let Some(a_children) = graph.get(a) {
            for b in a_children {
                if let Some(b_children) = graph.get(b) {
                    for c in b_children {
                        if let Some(c_children) = graph.get(c) {
                            if c_children.contains(a) {
                                let mut set = [a, b, c].to_vec();
                                set.sort();
                                unique_sets.insert(set);
                            }
                        }
                    }
                }
            }
        }
    }

    let count = unique_sets
        .iter()
        .filter(|set| set.iter().any(|v| v.starts_with("t")))
        .count();

    println!("{}", count);
}

fn bron_kerbosch(
    graph: &Graph,
    r: &mut HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    max_clique_size: usize,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() == max_clique_size {
            let mut values: Vec<String> = r.iter().cloned().collect();
            values.sort();
            println!("Found maximal clique: {:?}", values.join(","));
        }
    } else {
        for v in p.clone() {
            r.insert(v.clone());

            let neighbors = graph.get(&v).unwrap();
            let mut new_p = HashSet::new();
            for node in p.iter() {
                if neighbors.contains(node) {
                    new_p.insert(node.clone());
                }
            }

            let mut new_x = HashSet::new();
            for node in x.iter() {
                if neighbors.contains(node) {
                    new_x.insert(node.clone());
                }
            }

            bron_kerbosch(graph, r, &mut new_p, &mut new_x, max_clique_size);

            r.remove(&v);
            p.remove(&v);
            x.insert(v);
        }
    }
}

pub fn run_part_2() {
    let graph = build_graph();

    let max_clique_size = graph.values().fold(0, |acc, v| acc.max(v.len()));

    let mut r = HashSet::new();
    let mut x = HashSet::new();
    let mut p = HashSet::new();

    for node in graph.keys() {
        p.insert(node.clone());
    }

    bron_kerbosch(&graph, &mut r, &mut p, &mut x, max_clique_size);
}
