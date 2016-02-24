mod graph;

use graph::Graph;
use graph::Edge;

fn main() {
    let vertex_list = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)];
    let adjacency_list = vec![
        Edge {source: 0, destination: 1, weight: 5},
        Edge {source: 0, destination: 2, weight: 2},
        Edge {source: 2, destination: 3, weight: 3},
        Edge {source: 4, destination: 3, weight: 1}
    ];
    let g: Graph<i64> = Graph::new(&vertex_list, &adjacency_list);
    println!("Hello, world!");
    println!("{:?}", g);
}
