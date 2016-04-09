extern crate graph;

use std::env;
use graph::{flow_from_dicaps, flow_from_txt, FlowGraph};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[2];
    let file_type = args[1].as_str();
    let parsed_opt = match file_type {
        "dicaps" => {
            Some(flow_from_dicaps(&file_name))
        },
        "txt" => {
            Some(flow_from_txt(&file_name))
        },
        _ => {
            None
        }
    };
    let parsed = parsed_opt.expect("Expected either \"dicaps\" or \"txt\"");
    let source = parsed.0;
    let sink = parsed.1;
    let mut g = parsed.2;
    let flow_result = g.max_flow(source, sink);
    let total_flow = flow_result.0;
    let flow_paths = flow_result.1;

    println!("Total Flow: {}", total_flow);
    println!("Flow Paths: {:?}", flow_paths);
}
