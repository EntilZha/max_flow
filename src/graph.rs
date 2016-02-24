#[derive(Debug)]
pub struct Graph<V: Copy> {
  pub vertexes: Vec<Vertex<V>>,
  adjacency_matrix: Vec<Vec<u64>>
}

#[derive(Debug)]
pub struct Vertex<V: Copy> {
    pub id: u64,
    pub value: V
}

pub struct Edge {
    pub source: u64,
    pub destination: u64,
    pub weight: u64
}

impl<V: Copy> Graph<V> {
    pub fn new(vertex_list: &Vec<(u64, V)>, adjacency_list: &Vec<Edge>) -> Graph<V> {
        let vertexes: Vec<Vertex<V>> = vertex_list.iter().map(|v| {
            Vertex::<V>{ id: v.0, value: v.1 }
        }).collect();

        let n_vertexes: usize = vertexes.len();
        let mut adjacency_matrix: Vec<Vec<u64>> = Vec::with_capacity(n_vertexes);
        for _ in 0..n_vertexes {
            adjacency_matrix.push(vec![0; n_vertexes]);
        }
        for edge in adjacency_list {
            adjacency_matrix[edge.source as usize][edge.destination as usize] = edge.weight;
        }

        Graph { vertexes: vertexes, adjacency_matrix: adjacency_matrix }
    }

    pub fn find_path(&self) -> Option<Vec<Vertex<V>>> {
        None
    }
}
