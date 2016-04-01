extern crate graph;

use graph::Graph;
use graph::FlowEdge;
use graph::FlowGraph;

fn main() {
    let vertex_list = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)];
    let edge_list = vec![
        (0, 1, FlowEdge{flow: 0, capacity: 1}),
        (0, 2, FlowEdge{flow: 0, capacity: 1}),
        (1, 3, FlowEdge{flow: 0, capacity: 1}),
        (1, 5, FlowEdge{flow: 0, capacity: 1}),
        (2, 5, FlowEdge{flow: 0, capacity: 1}),
        (2, 6, FlowEdge{flow: 0, capacity: 1}),
        (3, 4, FlowEdge{flow: 0, capacity: 1}),
        (6, 4, FlowEdge{flow: 0, capacity: 2})
    ];
    let mut g = Graph::new(&vertex_list, &edge_list);
    let flow_result = g.max_flow(0, 4);
    let total_flow = flow_result.0;
    let flow_paths = flow_result.1;

    println!("{:?}", g);
    println!("Total Flow: {}", total_flow);
    println!("Flow Paths: {:?}", flow_paths);
}
