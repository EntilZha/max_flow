use std::collections::{HashMap, VecDeque};
use std::iter::Iterator;
use std::u64;
use std::usize;

/// Alias type to usize for VertexId attributes.
pub type VertexId = usize;

#[derive(Debug)]
pub struct Triplet<T: Property>(pub VertexId, pub T, pub VertexId);

/// Valid type to be used for a vertex or edge property.
pub trait Property: Copy {}
impl<T> Property for T where T: Copy {}

/// Represent a Graph structure.
#[derive(Debug)]
pub struct Graph<V: Property, E: Property> {
    pub vertexes: HashMap<VertexId, Vertex<V>>,
    pub edges: HashMap<(VertexId, VertexId), E>,
    adjacency_matrix: Vec<Vec<bool>>,
}

/// Represents a Vertex which has a value property and a list of neighbors.
#[derive(Debug)]
pub struct Vertex<V: Property> {
    pub value: V,
    pub neighbors: Vec<VertexId>,
}

/// Edge property that provides fields for a flow graph.
#[derive(Debug, Copy, Clone)]
pub struct FlowEdge {
    pub capacity: i64,
    pub flow: i64
}

/// Representation of breadth first search iterator.
pub struct BfsIterator<'a, V: 'a + Property, E: 'a + Property, F> {
    queue: VecDeque<VertexId>,
    graph: &'a Graph<V, E>,
    distances: Vec<u64>,
    parents: Vec<VertexId>,
    predicate: F
}

impl<'a, V: Property, E: Property, F> BfsIterator<'a, V, E, F> {
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
impl<'a, V: Property, E: Property, F> Iterator for BfsIterator<'a, V, E, F>
    where F: Fn(V, E, V) -> bool {
    type Item = (VertexId, u64, VertexId);
    fn next(&mut self) -> Option<(VertexId, u64, VertexId)> {
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
        BfsIterator::new(self, source, true_predicate)
    }
    pub fn bfs_iter_predicate<F>(&'a self, source: VertexId, predicate: F) -> BfsIterator<V, E, F>
    where F: Fn(V, E, V) -> bool {
        BfsIterator::new(self, source, predicate)
    }
}

/// Creates a path from a list of nodes from a tree search (BFS or DFS). The visited nodes are expected to be in the
/// format (vertex, distance_from_source, parent). The path is computed using the parent back pointers. It is assumed
/// that there does exist a path, it is a programming error which will cause a panic if that is not true
pub fn path_from_visited(source: VertexId,
                         sink: VertexId,
                         visited: &Vec<(VertexId, u64, VertexId)>) -> Vec<VertexId> {
    let mut node_parent_map: HashMap<VertexId, VertexId> = HashMap::new();
    for node in visited {
        node_parent_map.insert(node.0, node.2);
    }
    let mut path: Vec<VertexId> = Vec::new();
    let mut node = sink;
    loop {
        path.insert(0, node);
        if node == source {
            break;
        }
        node = node_parent_map[&node];
    }
    path
}

/// Special type of graph which has edges which can have flow and capacity.
pub trait FlowGraph<V> {
    fn augmenting_path(&self, source: VertexId, sink: VertexId) -> Option<Vec<VertexId>>;
    fn max_flow(&mut self, source: VertexId, sink: VertexId) -> (i64, Vec<Vec<Triplet<FlowEdge>>>);
}

impl<'a, V: Property> FlowGraph<V> for Graph<V, FlowEdge> {
    /// Returns a path from source to sink if one exists that has non-zero flow.
    fn augmenting_path(&self, source: VertexId, sink: VertexId) -> Option<Vec<VertexId>> {
        let bfs_edges: Vec<(VertexId, u64, VertexId)> = BfsIterator::new(self, source, flow_predicate).collect();
        match bfs_edges.iter().any(|element| element.0 == sink) {
            true => {
                Some(path_from_visited(source, sink, &bfs_edges))
            },
            _ => None
        }
    }

    /// Computes a vector of flow paths. Each path includes edges sequentially with the flow across that edge.
    fn max_flow(&mut self, source: VertexId, sink: VertexId) -> (i64, Vec<Vec<Triplet<FlowEdge>>>) {
        let mut flow_paths: Vec<Vec<Triplet<FlowEdge>>> = Vec::new();
        let mut total_flow = 0;
        loop {
            let path_option = self.augmenting_path(source, sink);
            match path_option {
                Some(path) => {
                    println!("Path: {:?}", path);
                    let mut edges: Vec<Triplet<FlowEdge>> = Vec::new();
                    for i in 0..path.len() {
                        if i + 1 != path.len() {
                            let v_0 = path[i];
                            let v_1 = path[i + 1];
                            edges.push(Triplet(v_0, self.edges[&(v_0, v_1)], v_1));
                        }
                    }
                    let flow: i64 = edges.iter().map(|triplet| triplet.1.capacity - triplet.1.flow).min().unwrap();
                    println!("Path flow: {}", flow);
                    total_flow += flow;
                    for edge in &edges {
                        let g_edge = self.edges.get_mut(&(edge.0, edge.2)).unwrap();
                        g_edge.flow = g_edge.flow + flow;
                    }
                    println!("{:?}", self.edges);
                    flow_paths.push(edges);
                },
                None => {
                    break;
                }
            }
        }
        (total_flow, flow_paths)
    }
}

fn true_predicate<'a, V: Property, E: Property>(_: V, _: E, _: V) -> bool {
    true
}

/// Ensure that there is available flow across the edge.
fn flow_predicate<'a, V: Property>(_: V, edge: FlowEdge, _: V) -> bool {
    edge.capacity - edge.flow > 0
}

#[cfg(test)]
mod tests {
    use Graph;
    use FlowGraph;
    use VertexId;
    use FlowEdge;
    use path_from_visited;
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

    #[test]
    fn test_augmenting_path() {
        let vertex_list = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)];
        let edge_list = vec![
            (0, 1, FlowEdge{flow: 0, capacity: 1}),
            (0, 2, FlowEdge{flow: 0, capacity: 1}),
            (1, 3, FlowEdge{flow: 0, capacity: 1}),
            (1, 5, FlowEdge{flow: 0, capacity: 1}),
            (2, 5, FlowEdge{flow: 0, capacity: 1}),
            (2, 6, FlowEdge{flow: 0, capacity: 1}),
            (3, 4, FlowEdge{flow: 0, capacity: 1})
        ];
        let mut g = Graph::new(&vertex_list, &edge_list);

        assert_eq!(g.augmenting_path(0, 4).unwrap(), [0, 1, 3, 4]);

        {
            let edge = g.edges.get_mut(&(1, 3)).unwrap();
            edge.flow = 1;
        }
        assert_eq!(g.augmenting_path(0, 4), None);
    }

    #[test]
    fn test_path_from_visited() {
        let source = 0;
        let sink = 4;
        let visited = vec![(0, 0, usize::MAX), (1, 1, 0), (2, 1, 0), (5, 2, 1), (3, 2, 1), (4, 3, 3), (6, 2, 2)];
        let path = path_from_visited(source, sink, &visited);
        assert_eq!(path, [0, 1, 3, 4]);
    }

    #[test]
    fn test_max_flow() {
        let vertex_list = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)];
        let edge_list = vec![
            (0, 1, FlowEdge{flow: 0, capacity: 1}),
            (0, 2, FlowEdge{flow: 0, capacity: 1}),
            (1, 3, FlowEdge{flow: 0, capacity: 1}),
            (1, 5, FlowEdge{flow: 0, capacity: 1}),
            (2, 5, FlowEdge{flow: 0, capacity: 1}),
            (2, 6, FlowEdge{flow: 0, capacity: 1}),
            (3, 4, FlowEdge{flow: 0, capacity: 1})
        ];
        let mut g = Graph::new(&vertex_list, &edge_list);
        let flow_result = g.max_flow(0, 4);
        let total_flow = flow_result.0;
        let flow_paths = flow_result.1;
    }
}
