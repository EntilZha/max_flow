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
    pub fn size(&self) -> (usize, usize) {
        (self.vertexes.len(), self.edges.len())
    }
    pub fn number_of_vertexes(&self) -> usize {
        self.vertexes.len()
    }
    pub fn number_of_edges(&self) -> usize {
        self.edges.len()
    }
}

#[cfg(test)]
mod tests {
    use graph::Graph;

    #[test]
    fn test_new_graph() {
        let vertex_list = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)];
        let edge_list = vec![
            (0, 1, 5),
            (0, 2, 2),
            (2, 3, 3),
            (4, 3, 1)
        ];
        let g = Graph::new(&vertex_list, &edge_list);
        assert_eq!(g.size(), (5, 4));
        for k in 0..5 {
            assert!(g.vertexes.contains_key(&k));
        }
        assert_eq!(g.edges[&(0, 1)], 5);
        assert_eq!(g.edges[&(0, 2)], 2);
        assert_eq!(g.edges[&(2, 3)], 3);
        assert_eq!(g.edges[&(4, 3)], 1);
    }
}
