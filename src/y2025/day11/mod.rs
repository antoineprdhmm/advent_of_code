use std::collections::{HashMap, VecDeque};

use crate::read_input;

pub mod dot;

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

pub fn run_part_1() {
    let graph = read_graph();

    let mut paths_count = 0;
    let mut deque: VecDeque<&str> = graph
        .get("you")
        .unwrap()
        .iter()
        .map(String::as_str)
        .collect();

    while let Some(device) = deque.pop_front() {
        if device == "out" {
            paths_count += 1;
        } else {
            for device in graph.get(device).unwrap() {
                deque.push_back(device);
            }
        }
    }

    assert_eq!(paths_count, 796);
}

fn dfs<'a>(
    device: &'a str,
    fft: bool,
    dac: bool,
    graph: &'a HashMap<String, Vec<String>>,
    memo: &mut HashMap<(&'a str, bool, bool), usize>,
) -> usize {
    if device == "out" {
        return if fft && dac { 1 } else { 0 };
    }

    if let Some(&cached) = memo.get(&(device, fft, dac)) {
        return cached;
    }

    let next_fft = fft || device == "fft";
    let next_dac: bool = dac || device == "dac";

    let paths_count = graph
        .get(device)
        .unwrap()
        .iter()
        .map(|next_device| dfs(next_device, next_fft, next_dac, graph, memo))
        .sum();

    memo.insert((device, fft, dac), paths_count);

    paths_count
}

pub fn run_part_2() {
    let graph = read_graph();
    let paths_count = dfs("svr", false, false, &graph, &mut HashMap::new());
    assert_eq!(paths_count, 294053029111296);
}
