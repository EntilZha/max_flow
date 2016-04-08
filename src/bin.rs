extern crate graph;

use std::env;
use graph::flowgraph_from_file;
use graph::FlowGraph;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let parsed = flowgraph_from_file(&file_name);
    let source = parsed.0;
    let sink = parsed.1;
    let mut g = parsed.2;
    let flow_result = g.max_flow(source, sink);
    let total_flow = flow_result.0;
    let flow_paths = flow_result.1;

    println!("Total Flow: {}", total_flow);
    println!("Flow Paths: {:?}", flow_paths);
}
