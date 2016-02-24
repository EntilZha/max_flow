use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph<V: Copy, E: Copy> {
  pub vertexes: HashMap<usize, Vertex<V>>,
  pub edges: HashMap<(usize, usize), E>,
  adjacency_matrix: Vec<Vec<bool>>
}

#[derive(Debug)]
pub struct Vertex<V: Copy> {
    pub value: V,
    pub neighbors: Vec<usize>
}

impl<V: Copy, E: Copy> Graph<V, E> {
    pub fn new(vertex_list: &Vec<(usize, V)>, edge_list: &Vec<(usize, usize, E)>) -> Graph<V, E> {
        let mut vertexes: HashMap<usize, Vertex<V>> = HashMap::new();
        for v in vertex_list {
            vertexes.insert(v.0, Vertex {value: v.1, neighbors: Vec::new()});
        }
        let n_vertexes: usize = vertexes.len();

        let mut adjacency_matrix: Vec<Vec<bool>> = vec![vec![false; n_vertexes]; n_vertexes];
        let mut edges: HashMap<(usize, usize), E> = HashMap::new();
        for edge in edge_list {
            adjacency_matrix[edge.0][edge.1] = true;
            vertexes.get_mut(&edge.0).unwrap().neighbors.push(edge.1);
            edges.insert((edge.0, edge.1), edge.2);
        }

        Graph { vertexes: vertexes, edges: edges, adjacency_matrix: adjacency_matrix }
    }

    pub fn find_path(&self) -> Option<Vec<Vertex<V>>> {
        None
    }
}
