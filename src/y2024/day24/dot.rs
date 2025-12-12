use std::fs::File;
use std::io::prelude::*;

use super::{CrossedWiresSystem, Gate};

pub fn generate_dot_file(crossed_wires_system: &CrossedWiresSystem) {
    let ands: Vec<String> = crossed_wires_system
        .operations
        .iter()
        .filter(|operation| operation.gate == Gate::And)
        .map(|operation| {
            format!(
                "{}_AND_{} [label=\"AND {}\", shape=invtriangle];",
                operation.key1, operation.key2, operation.destination_key
            )
        })
        .collect();
    let ors: Vec<String> = crossed_wires_system
        .operations
        .iter()
        .filter(|operation| operation.gate == Gate::Or)
        .map(|operation| {
            format!(
                "{}_OR_{} [label=\"OR {}\", shape=diamond];",
                operation.key1, operation.key2, operation.destination_key
            )
        })
        .collect();
    let xors: Vec<String> = crossed_wires_system
        .operations
        .iter()
        .filter(|operation| operation.gate == Gate::Xor)
        .map(|operation| {
            format!(
                "{}_XOR_{} [label=\"XOR {}\", shape=hexagon];",
                operation.key1, operation.key2, operation.destination_key
            )
        })
        .collect();

    let connections: Vec<String> = crossed_wires_system
        .operations
        .iter()
        .flat_map(|operation| {
            let gate = format!("{}_{:?}_{}", operation.key1, operation.gate, operation.key2);
            [
                format!("{} -> {};", operation.key1, gate),
                format!("{} -> {};", operation.key2, gate),
                format!("{} -> {};", gate, operation.destination_key),
            ]
            .to_vec()
        })
        .collect();

    let graph = format!(
        "
        digraph LogicCircuit {{
            // Set graph properties
            rankdir=LR;
            node [shape=box, style=filled, color=lightblue];
            
            // Input nodes
            subgraph cluster_inputs {{
                label=\"Inputs\";
                node [shape=ellipse, color=lightgreen];
                
                // x inputs
                x00 x01 x02 x03 x04 x05 x06 x07 x08 x09 x10;
                x11 x12 x13 x14 x15 x16 x17 x18 x19 x20;
                x21 x22 x23 x24 x25 x26 x27 x28 x29 x30;
                x31 x32 x33 x34 x35 x36 x37 x38 x39 x40;
                x41 x42 x43 x44;
                
                // y inputs
                y00 y01 y02 y03 y04 y05 y06 y07 y08 y09 y10;
                y11 y12 y13 y14 y15 y16 y17 y18 y19 y20;
                y21 y22 y23 y24 y25 y26 y27 y28 y29 y30;
                y31 y32 y33 y34 y35 y36 y37 y38 y39 y40;
                y41 y42 y43 y44;
            }}
            
            // Output nodes
            subgraph cluster_outputs {{
                label=\"Outputs\";
                node [shape=ellipse, color=lightpink];
                
                z00 z01 z02 z03 z04 z05 z06 z07 z08 z09 z10;
                z11 z12 z13 z14 z15 z16 z17 z18 z19 z20;
                z21 z22 z23 z24 z25 z26 z27 z28 z29 z30;
                z31 z32 z33 z34 z35 z36 z37 z38 z39 z40;
                z41 z42 z43 z44 z45;
            }}
            
            // Gate nodes
            // XOR gates (hexagon shape)
            {}
            
            // AND gates (triangle shape)
            {}
            
            // OR gates (diamond shape)
            {}
            
            // Connections (a small subset for illustration)
            {}
        }}
    ",
        ands.join(" "),
        xors.join(" "),
        ors.join(" "),
        connections.join(" ")
    );

    let mut file = File::create("src/y2024/day24/day24.dot").unwrap();
    file.write_all(graph.as_bytes()).unwrap();
}
