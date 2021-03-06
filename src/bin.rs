extern crate graph;
extern crate time;

use std::env;
use graph::{flow_from_dicaps, flow_from_txt, FlowGraph, DFS, BFS};

fn main() {
    let args: Vec<String> = env::args().collect();
    let search_str = args[1].as_str();
    let search = match search_str {
        "bfs" => Some(BFS),
        "dfs" => Some(DFS),
        _ => None
    }.expect("Expected 'bfs' or 'dfs'");
    let file_type = args[2].as_str();
    let file_name = &args[3];
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
    let start_time = time::get_time();
    let total_flow = g.max_flow(source, sink, search);
    let end_time = time::get_time();
    let diff = end_time - start_time;
    println!("Algorithm:{}\tVertexes:{}\tEdges:{}\tFlow:{}\tRuntime:{}s",
        search_str, g.n_vertexes(), g.n_edges(), total_flow, diff.num_milliseconds() as f64 / 1000.0);
}
