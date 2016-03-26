use std::collections::{HashMap, VecDeque};
use std::iter::Iterator;
use std::u64;
use std::usize;

pub type VertexId = usize;

pub trait Property : Copy + Ord + Eq {}
impl<T> Property for T where T: Copy + Ord + Eq {}

/// Represent a Graph structure
#[derive(Debug)]
pub struct Graph<V: Property, E: Property> {
    pub vertexes: HashMap<VertexId, Vertex<V>>,
    pub edges: HashMap<(VertexId, VertexId), E>,
    adjacency_matrix: Vec<Vec<bool>>,
}

#[derive(Debug)]
pub struct Vertex<V: Property> {
    pub value: V,
    pub neighbors: Vec<VertexId>,
}

pub struct BfsIterator<'a, V: 'a + Property, E: 'a + Property, F> {
    queue: VecDeque<VertexId>,
    graph: &'a Graph<V, E>,
    distances: Vec<u64>,
    parents: Vec<VertexId>,
    predicate: F
}

pub impl<'a, V: Property, E: Property, F> BfsIterator<'a, V, E, F> {
    fn new(graph: &'a Graph<V, E>, source: VertexId, predicate: F) -> BfsIterator<'a, V, E, F> {
        let mut queue = VecDeque::new();
        queue.push_back(source);
        let mut distances = vec![u64::MAX; graph.n_vertexes()];
        let parents = vec![usize::MAX; graph.n_vertexes()];
        distances[source] = 0;
        BfsIterator {
            graph: graph,
            queue: queue,
            distances: distances,
            parents: parents,
            predicate: predicate
        }
    }
}

/// Iterator for a breadth first search over a graph
/// Returns in order a tuple of (vertex, distance, parent)
pub impl<'a, V: Property, E: Property, F> Iterator for BfsIterator<'a, V, E, F>
    where F: Fn(V, E, V) -> bool {
    type Item = (VertexId, u64, VertexId);
    pub fn next(&mut self) -> Option<(VertexId, u64, VertexId)> {
        let g = self.graph;
        match self.queue.pop_front() {
            Some(vertex) => {
                for v in self.graph.adjacent_vertexes(&vertex) {
                    let predicate = &self.predicate;
                    let pred = predicate(g.vertexes[&vertex].value, g.edges[&(vertex, *v)], g.vertexes[v].value);
                    if self.distances[*v] == u64::MAX && pred {
                        self.distances[*v] = self.distances[vertex] + 1;
                        self.parents[*v] = vertex;
                        self.queue.push_back(*v);
                    }
                }
                Some((vertex, self.distances[vertex], self.parents[vertex]))
            }
            _ => None
        }
    }
}

impl<'a, V: Property, E: Property> Graph<V, E> {
    pub fn new(vertex_list: &Vec<(VertexId, V)>, edge_list: &Vec<(VertexId, VertexId, E)>) -> Graph<V, E> {
        let mut vertexes: HashMap<VertexId, Vertex<V>> = HashMap::new();
        for v in vertex_list {
            vertexes.insert(v.0,
                            Vertex {
                                value: v.1,
                                neighbors: Vec::new(),
                            });
        }
        let n_vertexes: usize = vertexes.len();

        let mut adjacency_matrix: Vec<Vec<bool>> = vec![vec![false; n_vertexes]; n_vertexes];
        let mut edges: HashMap<(VertexId, VertexId), E> = HashMap::new();
        for edge in edge_list {
            adjacency_matrix[edge.0][edge.1] = true;
            vertexes.get_mut(&edge.0).unwrap().neighbors.push(edge.1);
            edges.insert((edge.0, edge.1), edge.2);
        }

        Graph {
            vertexes: vertexes,
            edges: edges,
            adjacency_matrix: adjacency_matrix,
        }
    }
    pub fn size(&self) -> (usize, usize) {
        (self.vertexes.len(), self.edges.len())
    }
    pub fn n_vertexes(&self) -> usize {
        self.vertexes.len()
    }
    pub fn n_edges(&self) -> usize {
        self.edges.len()
    }
    pub fn adjacent_vertexes(&self, vertex: &VertexId) -> &Vec<VertexId> {
        &self.vertexes[vertex].neighbors
    }
    pub fn bfs_iter(&self, source: VertexId) -> BfsIterator<V, E, fn(V, E, V) -> bool> {
        BfsIterator::new(self, source, bfs_true_predicate)
    }
    pub fn bfs_iter_predicate<F>(&'a self, source: VertexId, predicate: F) -> BfsIterator<V, E, F>
    where F: Fn(V, E, V) -> bool {
        BfsIterator::new(self, source, predicate)
    }
}

pub trait FlowGraph<V> {
    fn augmenting_path(&self, source: VertexId) -> Vec<VertexId>;
}

impl<'a, V: Property> FlowGraph<V> for Graph<V, i64> {
    pub fn augmenting_path(&self, source: VertexId) -> Vec<VertexId> {
        BfsIterator::new(self, source, flow_predicate).collect()
    }
}

fn bfs_true_predicate<'a, V: Property, E: Property>(_: V, _: E, _: V) -> bool {
    true
}

fn flow_predicate<'a, V: Property>(_: V, edge: i64, _: V) {
    edge > 0;
}

#[cfg(test)]
mod tests {
    use Graph;
    use VertexId;
    use std::collections::HashSet;
    use std::usize;

    #[test]
    fn test_new_graph() {
        let vertex_list = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)];
        let edge_list = vec![(0, 1, 5), (0, 2, 2), (2, 3, 3), (4, 3, 1)];
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

    #[test]
    fn test_bfs() {
        let vertex_list = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)];
        let edge_list = vec![(0, 1, 1), (1, 2, 1), (0, 3, 1), (3, 4, 1), (4, 1, 0), (4, 5, 1), (5, 2, 1)];
        let g = Graph::new(&vertex_list, &edge_list);
        let result: Vec<(VertexId, u64, VertexId)> = g.bfs_iter(0).collect();
        let mut result_set = HashSet::new();
        result_set.extend(result);
        let mut expect = HashSet::new();
        expect.insert((0, 0, usize::MAX));
        expect.insert((1, 1, 0));
        expect.insert((2, 2, 1));
        expect.insert((3, 1, 0));
        expect.insert((4, 2, 3));
        expect.insert((5, 3, 4));
        assert_eq!(result_set, expect);
    }
}
