use std::collections::{VecDeque, HashSet};
use std::iter::Iterator;
use std::u64;
use std::i64;
use std::usize;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::min;

/// Alias type to usize for VertexId attributes.
pub type VertexId = usize;

#[derive(Debug)]
pub struct Triplet<T: Property>(pub VertexId, pub T, pub VertexId);

#[derive(Debug)]
pub struct Edge(pub VertexId, pub VertexId);

/// Valid type to be used for a vertex or edge property.
pub trait Property: Copy + Default {}
impl<T> Property for T where T: Copy + Default {}

/// Represent a Graph structure.
#[derive(Debug)]
pub struct Graph<V: Property, E: Property> {
    pub vertexes: Vec<Vertex<V>>,
    pub edges: Vec<Vec<E>>,
    n_edges: usize
}

/// Represents a Vertex which has a value property and a list of neighbors.
#[derive(Debug)]
pub struct Vertex<V: Property> {
    pub value: V,
    pub neighbors: Vec<VertexId>,
}

/// Edge property that provides fields for a flow graph.
#[derive(Debug, Copy, Clone, Default)]
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
    where F: Fn(E) -> bool {
    type Item = (VertexId, u64, VertexId);
    fn next(&mut self) -> Option<(VertexId, u64, VertexId)> {
        let g = self.graph;
        match self.queue.pop_front() {
            Some(vertex) => {
                for v in &self.graph.vertexes[vertex].neighbors {
                    if self.distances[*v] == u64::MAX {
                        let predicate = &self.predicate;
                        if !predicate(g.edges[vertex][*v]) {
                            continue;
                        }
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
        let mut vertexes: Vec<Vertex<V>> = Vec::with_capacity(vertex_list.len());
        let mut i = 0;
        for v in vertex_list {
            assert!(v.0 == i, "Must provide vertexes in order from 0 to n - 1");
            vertexes.push(Vertex {value: v.1, neighbors: Vec::new()});
            i += 1;
        }

        let mut edges: Vec<Vec<E>> = vec![vec![Default::default(); vertexes.len()]; vertexes.len()];
        let mut n_edges = 0;
        for edge in edge_list {
            n_edges += 1;
            vertexes.get_mut(edge.0).unwrap().neighbors.push(edge.1);
            edges[edge.0][edge.1] = edge.2;
        }

        Graph {
            vertexes: vertexes,
            edges: edges,
            n_edges: n_edges
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.n_vertexes(), self.n_edges())
    }

    pub fn n_vertexes(&self) -> usize {
        self.vertexes.len()
    }

    pub fn n_edges(&self) -> usize {
        self.n_edges
    }

    pub fn bfs_iter(&self, source: VertexId) -> BfsIterator<V, E, fn(E) -> bool> {
        BfsIterator::new(self, source, true_predicate)
    }

    pub fn bfs_iter_predicate<F>(&'a self, source: VertexId, predicate: F) -> BfsIterator<V, E, F>
    where F: Fn(E) -> bool {
        BfsIterator::new(self, source, predicate)
    }
}

/// Creates a path from a list of nodes from a tree search (BFS or DFS). The visited nodes are expected to be in the
/// format (vertex, distance_from_source, parent). The path is computed using the parent back pointers. It is assumed
/// that there does exist a path, it is a programming error which will cause a panic if that is not true
pub fn path_from_visited(source: VertexId,
                         sink: VertexId,
                         visited: &Vec<(VertexId, u64, VertexId)>,
                         n_vertexes: usize) -> Vec<VertexId> {
    let mut node_parent_map = vec![usize::MAX; n_vertexes];
    for node in visited {
        node_parent_map[node.0] = node.2;
    }
    let mut path: Vec<VertexId> = Vec::new();
    let mut node = sink;
    loop {
        path.push(node);
        if node == source {
            break;
        }
        node = node_parent_map[node];
    }
    path.reverse();
    path
}

/// Special type of graph which has edges which can have flow and capacity.
pub trait FlowGraph<V: Property> {
    fn augmenting_path(&self, source: VertexId, sink: VertexId) -> Option<Vec<VertexId>>;
    fn max_flow(&mut self, source: VertexId, sink: VertexId) -> (i64, Vec<Vec<Edge>>);
}

impl<'a, V: Property> FlowGraph<V> for Graph<V, FlowEdge> {
    /// Returns a path from source to sink if one exists that has non-zero flow.
    fn augmenting_path(&self, source: VertexId, sink: VertexId) -> Option<Vec<VertexId>> {
        let bfs_iter = BfsIterator::new(self, source, flow_predicate);
        let mut bfs_edges: Vec<(VertexId, u64, VertexId)> = Vec::new();
        let mut sink_exists = false;
        for e in bfs_iter {
            bfs_edges.push(e);
            sink_exists = sink_exists || e.0 == sink;
        }
        match sink_exists {
            true => {
                Some(path_from_visited(source, sink, &bfs_edges, self.n_vertexes()))
            },
            _ => None
        }
    }

    /// Computes a vector of flow paths. Each path includes edges sequentially with the flow across that edge.
    fn max_flow(&mut self, source: VertexId, sink: VertexId) -> (i64, Vec<Vec<Edge>>) {
        let mut flow_paths: Vec<Vec<Edge>> = Vec::new();
        let mut total_flow = 0;
        loop {
            let path_option = self.augmenting_path(source, sink);
            match path_option {
                Some(path) => {
                    let mut edges: Vec<Triplet<FlowEdge>> = Vec::new();
                    let mut flow: i64 = i64::MAX;
                    for i in 0..path.len() {
                        if i + 1 != path.len() {
                            let v_0 = path[i];
                            let v_1 = path[i + 1];
                            let flow_edge = self.edges[v_0][v_1];
                            edges.push(Triplet(v_0, flow_edge, v_1));
                            flow = min(flow_edge.capacity - flow_edge.flow, flow);
                        }
                    }
                    total_flow += flow;
                    let mut flow_path: Vec<Edge> = Vec::new();
                    for edge in &edges {
                        let g_edge = self.edges.get_mut(edge.0).unwrap().get_mut(edge.2).unwrap();
                        g_edge.flow = g_edge.flow + flow;
                        flow_path.push(Edge(edge.0, edge.2));
                    }
                    flow_paths.push(flow_path);
                },
                None => {
                    break;
                }
            }
        }
        (total_flow, flow_paths)
    }
}

pub fn flow_from_dicaps(file_name: &str) -> (VertexId, VertexId, Graph<usize, FlowEdge>) {
    let f = File::open(file_name).expect(&format!("Input file does not exist: {}", file_name));
    let reader = BufReader::new(&f);
    let mut num_vertexes = 0;
    let mut num_edges = 0;
    let mut source = None;
    let mut sink = None;
    let mut edges: Vec<(VertexId, VertexId, FlowEdge)> = Vec::new();
    let mut num_parsed_edges = 0;
    for raw_line in reader.lines() {
        let line = raw_line.unwrap();
        let tokens = line.split_whitespace().collect::<Vec<_>>();
        match tokens.len() {
            4 => {
                match tokens[0] {
                    "p" => {
                        num_vertexes = tokens[2].parse::<_>().expect("Expected an integer for number of vertexes");
                        num_edges = tokens[3].parse::<_>().expect("Expected an integer for number of edges");
                    },
                    "a" => {
                        let u = tokens[1].parse::<VertexId>().expect("Expected an integer for source in edge");
                        let v = tokens[2].parse::<VertexId>().expect("Expected an integer for destination in edge");
                        let capacity = tokens[3].parse::<_>().expect("Expected an integer for capaicty");
                        if capacity > 0 {
                            edges.push((u, v, FlowEdge{flow: 0, capacity: capacity}));
                        }
                        num_parsed_edges += 1;
                    },
                    _ => {
                        panic!("Invalid line: {}", line);
                    }
                }
            },
            3 => {
                match tokens[0] {
                    "n" => {
                        match tokens[2] {
                            "s" => {
                                source = Some(
                                    tokens[1].parse::<VertexId>().expect("Expected an integer for source"));
                            },
                            "t" => {
                                sink = Some(
                                    tokens[1].parse::<VertexId>().expect("Expected an integer for sink"));
                            },
                            _ => {
                                panic!("Invalid line: {}", line);
                            }
                        }
                    }
                    _ => {
                        panic!("Invalid line: {}", line);
                    }
                }
            },
            1 => {
                if tokens[0] == "a" {
                    break;
                } else {
                    panic!("Invalid line: {}", line);
                }
            },
            0 => {
                break;
            }
            _ =>{
                panic!("Invalid line: {}", line)
            }
        }
    }
    assert!(num_parsed_edges == num_edges,
            "Number of edges specified and found are different: {} vs {}",
            num_parsed_edges, num_edges);
    let mut vertex_set: HashSet<VertexId> = HashSet::new();
    for e in &edges {
        vertex_set.insert(e.0);
        vertex_set.insert(e.1);
    }
    assert!(vertex_set.len() == num_vertexes,
            "Number of vertexes specified and found are different: {} vs {}",
            vertex_set.len(), num_vertexes);
    let vertexes = (0..num_vertexes).map(|x| (x, 0)).collect::<Vec<_>>();
    (source.expect("Must have a source"), sink.expect("Must have a sink"), Graph::new(&vertexes, &edges))
}

pub fn flow_from_txt(file_name: &str) -> (VertexId, VertexId, Graph<usize, FlowEdge>) {
    let f = File::open(file_name).expect(&format!("Input file does not exist: {}", file_name));
    let reader = BufReader::new(&f);
    let mut edges: Vec<(VertexId, VertexId, FlowEdge)> = Vec::new();
    let mut i = 0;
    let mut num_vertexes = 0;
    for raw_line in reader.lines() {
        let line = raw_line.unwrap();
        let tokens = line.split_whitespace().collect::<Vec<_>>();
        if i == 0 {
            num_vertexes = tokens[0].parse::<usize>().expect("Expected an integer for source in edge");
        } else {
            for v in tokens.iter().enumerate() {
                let capacity = v.1.parse::<i64>().expect("Expected an integer capacity");
                if capacity > 0 {
                    edges.push(
                        (i - 1, v.0,
                            FlowEdge{capacity: capacity, flow: 0})
                    );
                }
            }
        }
            i += 1;
    }
    let vertexes = (0..num_vertexes).map(|x| (x, 0)).collect::<Vec<_>>();
    (0, num_vertexes - 1, Graph::new(&vertexes, &edges))
}

fn true_predicate<'a, E: Property>(_: E) -> bool {
    true
}

/// Ensure that there is available flow across the edge.
fn flow_predicate<'a>(edge: FlowEdge) -> bool {
    edge.capacity - edge.flow > 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::usize;

    #[test]
    fn test_new_graph() {
        let vertex_list = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)];
        let edge_list = vec![(0, 1, 5), (0, 2, 2), (2, 3, 3), (4, 3, 1)];
        let g = Graph::new(&vertex_list, &edge_list);
        assert_eq!(g.size(), (5, 4));
        assert_eq!(g.vertexes.len(), vertex_list.len());
        assert_eq!(g.edges[0][1], 5);
        assert_eq!(g.edges[0][2], 2);
        assert_eq!(g.edges[2][3], 3);
        assert_eq!(g.edges[4][3], 1);
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
            let edge = g.edges.get_mut(1).unwrap().get_mut(3).unwrap();
            edge.flow = 1;
        }
        assert_eq!(g.augmenting_path(0, 4), None);
    }

    #[test]
    fn test_path_from_visited() {
        let source = 0;
        let sink = 4;
        let visited = vec![(0, 0, usize::MAX), (1, 1, 0), (2, 1, 0), (5, 2, 1), (3, 2, 1), (4, 3, 3), (6, 2, 2)];
        let path = path_from_visited(source, sink, &visited, 7);
        assert_eq!(path, [0, 1, 3, 4]);
    }

    #[test]
    fn test_max_flow_0() {
        let vertex_list = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)];
        let edge_list = vec![
            (0, 1, FlowEdge{flow: 0, capacity: 3}),
            (0, 2, FlowEdge{flow: 0, capacity: 1}),
            (1, 3, FlowEdge{flow: 0, capacity: 2}),
            (1, 5, FlowEdge{flow: 0, capacity: 1}),
            (2, 5, FlowEdge{flow: 0, capacity: 1}),
            (2, 6, FlowEdge{flow: 0, capacity: 1}),
            (3, 4, FlowEdge{flow: 0, capacity: 2}),
            (5, 6, FlowEdge{flow: 0, capacity: 1}),
            (6, 4, FlowEdge{flow: 0, capacity: 2})
        ];
        let mut g = Graph::new(&vertex_list, &edge_list);
        let flow_result = g.max_flow(0, 4);
        let total_flow = flow_result.0;
        assert_eq!(total_flow, 4);
    }

    #[test]
    fn test_max_flow_1() {
        let vertex_list = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
        let edge_list = vec![
            (0, 2, FlowEdge{flow: 0, capacity: 5}),
            (0, 3, FlowEdge{flow: 0, capacity: 5}),
            (2, 3, FlowEdge{flow: 0, capacity: 1}),
            (2, 1, FlowEdge{flow: 0, capacity: 5}),
            (3, 1, FlowEdge{flow: 0, capacity: 5}),
        ];
        let mut g = Graph::new(&vertex_list, &edge_list);
        let flow_result = g.max_flow(0, 1);
        let total_flow = flow_result.0;
        assert_eq!(total_flow, 10);
    }

    #[test]
    fn test_max_flow_2() {
        let vertex_list = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)];
        let edge_list = vec![
            (0, 1, FlowEdge{flow: 0, capacity: 11}),
            (0, 2, FlowEdge{flow: 0, capacity: 12}),
            (2, 1, FlowEdge{flow: 0, capacity: 1}),
            (1, 3, FlowEdge{flow: 0, capacity: 12}),
            (2, 4, FlowEdge{flow: 0, capacity: 11}),
            (4, 3, FlowEdge{flow: 0, capacity: 7}),
            (4, 5, FlowEdge{flow: 0, capacity: 4}),
            (3, 5, FlowEdge{flow: 0, capacity: 19}),
        ];
        let mut g = Graph::new(&vertex_list, &edge_list);
        let flow_result = g.max_flow(0, 5);
        let total_flow = flow_result.0;
        assert_eq!(total_flow, 23);
    }

    enum FileType {
        Dicaps,
        Text
    }

    fn test_flow_from_file(file_name: &str, flow: i64, file_type: FileType) {
        println!("Testing file: {}\n", file_name);
        let parsed = match file_type {
            FileType::Dicaps => flow_from_dicaps(file_name),
            FileType::Text => flow_from_txt(file_name)
        };
        let source = parsed.0;
        let sink = parsed.1;
        let mut g = parsed.2;
        println!("{:?}", g);
        let flow_result = g.max_flow(source, sink);
        let total_flow = flow_result.0;
        assert_eq!(total_flow, flow);
        println!("");
    }

    #[test]
    fn test_maxflow_from_files() {
        test_flow_from_file("data/dicaps/flow-graph.txt", 10, FileType::Dicaps);
        test_flow_from_file("data/dicaps/bipartite-flow.txt", 3, FileType::Dicaps);
        test_flow_from_file("data/dicaps/central.txt", 5, FileType::Dicaps);
        test_flow_from_file("data/txt/test_1.txt", 10, FileType::Text);
        test_flow_from_file("data/txt/test_2.txt", 23, FileType::Text);
        test_flow_from_file("data/txt/test_3.txt", 935, FileType::Text);
        test_flow_from_file("data/txt/test_4.txt", 2789, FileType::Text);
        test_flow_from_file("data/txt/test_5.txt", 2000000000, FileType::Text);
        test_flow_from_file("data/txt/test_6.txt", 23, FileType::Text);
        test_flow_from_file("data/txt/test_7.txt", 256, FileType::Text);
    }
}
