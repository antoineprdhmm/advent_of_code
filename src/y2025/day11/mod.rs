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
    let mut deque: VecDeque<&String> = VecDeque::new();

    for device in graph.get("you").unwrap() {
        deque.push_back(device);
    }

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

fn dfs(
    device: String,
    fft: bool,
    dac: bool,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    if device == "out" {
        return if fft && dac { 1 } else { 0 };
    }

    let mut paths_count = 0;

    let next_fft = fft || device == "fft";
    let next_dac: bool = dac || device == "dac";

    for next_device in graph.get(&device).unwrap() {
        if let Some(m) = memo.get(&(next_device.to_string(), next_fft, next_dac)) {
            paths_count += m;
        } else {
            let r = dfs(next_device.to_string(), next_fft, next_dac, graph, memo);
            paths_count += r;
            memo.insert((next_device.clone(), next_fft, next_dac), r);
        }
    }

    paths_count
}

pub fn run_part_2() {
    let graph = read_graph();
    let paths_count = dfs("svr".to_string(), false, false, &graph, &mut HashMap::new());
    assert_eq!(paths_count, 294053029111296);
}
