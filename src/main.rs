mod graph;

use graph::Graph;

fn main() {
    let vertex_list = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)];
    let edge_list = vec![
        (0, 1, 5),
        (0, 2, 2),
        (2, 3, 3),
        (4, 3, 1)
    ];
    let g = Graph::new(&vertex_list, &edge_list);
    println!("Hello, world!");
    println!("{:?}", g);
}
