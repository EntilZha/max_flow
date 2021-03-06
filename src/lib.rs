use std::collections::{VecDeque, HashSet};
use std::iter::Iterator;
use std::{i32, usize, u32};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::min;

/// Alias type to usize for `VertexId` attributes.
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
pub struct Graph<E: Property> {
    pub edges: Vec<Vec<E>>,
    pub neighbors: Vec<Vec<VertexId>>,
    n_edges: usize,
    n_vertexes: usize
}

/// Edge property that provides fields for a flow graph.
#[derive(Debug, Copy, Clone, Default)]
pub struct FlowEdge {
    pub capacity: i32,
    pub flow: i32
}

#[derive(Copy, Clone)]
pub enum Search {
    Bfs, Dfs
}

pub const BFS: Search = Search::Bfs;
pub const DFS: Search = Search::Dfs;

/// Representation of breadth first search iterator.
pub struct GraphIterator<'a, E: 'a + Property, F> {
    queue: VecDeque<VertexId>,
    stack: Vec<VertexId>,
    graph: &'a Graph<E>,
    distances: Vec<u32>,
    parents: Vec<VertexId>,
    predicate: F,
    search: Search,
    sink: VertexId,
    sink_found: bool
}

impl<'a, E: Property, F> GraphIterator<'a, E, F>
    where F: Fn(E) -> bool {
    fn new(graph: &'a Graph<E>, source: VertexId, sink: VertexId, predicate: F, search: Search) -> GraphIterator<'a, E, F> {
        let mut queue = VecDeque::new();
        let mut stack = Vec::new();
        match search {
            Search::Bfs => {
                queue.push_back(source);
            },
            Search::Dfs => {
                stack.push(source);
            }
        }
        let mut distances = vec![u32::MAX; graph.n_vertexes()];
        let parents = vec![usize::MAX; graph.n_vertexes()];
        distances[source] = 0;
        GraphIterator {
            graph: graph,
            queue: queue,
            stack: stack,
            distances: distances,
            parents: parents,
            predicate: predicate,
            search: search,
            sink: sink,
            sink_found: false
        }
    }

    fn pop(&mut self) -> Option<VertexId> {
        match self.search {
            Search::Bfs => self.queue.pop_front(),
            Search::Dfs => self.stack.pop()
        }
    }

    fn push(&mut self, v: VertexId) {
        match self.search {
            Search::Bfs => self.queue.push_back(v),
            Search::Dfs => self.stack.push(v)
        }
    }

    fn evaluate_predicate(&self, edge: E) -> bool {
        let predicate = &self.predicate;
        predicate(edge)
    }
}

/// Iterator for a breadth first search over a graph
/// Returns in order a tuple of (vertex, distance, parent)
impl<'a, E: Property, F> Iterator for GraphIterator<'a, E, F>
    where F: Fn(E) -> bool {
    type Item = (VertexId, u32, VertexId);
    fn next(&mut self) -> Option<(VertexId, u32, VertexId)> {
        if self.sink_found {
            None
        }
        else {
            match self.pop() {
                Some(vertex) => {
                    if vertex == self.sink {
                        self.sink_found = true;
                    } else {
                        for v in &self.graph.neighbors[vertex] {
                            if self.distances[*v] == u32::MAX &&
                                (self.evaluate_predicate(self.graph.edges[vertex][*v])) {
                                self.distances[*v] = self.distances[vertex] + 1;
                                self.parents[*v] = vertex;
                                self.push(*v);
                            }
                        }
                    }
                    Some((vertex, self.distances[vertex], self.parents[vertex]))
                }
                _ => None
            }
        }
    }
}

impl<'a, E: Property> Graph<E> {
    pub fn new(vertex_list: &[VertexId], edge_list: &[(VertexId, VertexId, E)]) -> Graph<E> {
        let mut neighbors: Vec<Vec<VertexId>> = vec![Vec::new(); vertex_list.len()];
        let mut v_len = 0;
        for v in vertex_list {
            assert!(*v == v_len, "Must provide vertexes in order from 0 to n - 1");
            v_len += 1;
        }

        let mut edges: Vec<Vec<E>> = vec![vec![Default::default(); v_len]; v_len];
        let mut n_edges = 0;
        for edge in edge_list {
            n_edges += 1;
            neighbors.get_mut(edge.0).unwrap().push(edge.1);
            edges[edge.0][edge.1] = edge.2;
        }

        Graph {
            edges: edges,
            neighbors: neighbors,
            n_edges: n_edges,
            n_vertexes: v_len
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.n_vertexes(), self.n_edges())
    }

    pub fn n_vertexes(&self) -> usize {
        self.n_vertexes
    }

    pub fn n_edges(&self) -> usize {
        self.n_edges
    }

    pub fn bfs_iter(&self, source: VertexId, sink: VertexId) -> GraphIterator<E, fn(E) -> bool> {
        GraphIterator::new(self, source, sink, true_predicate, BFS)
    }

    pub fn dfs_iter(&self, source: VertexId, sink: VertexId) -> GraphIterator<E, fn(E) -> bool> {
        GraphIterator::new(self, source, sink, true_predicate, DFS)
    }
}

/// Creates a path from a list of nodes from a tree search (BFS or DFS). The visited nodes are expected to be in the
/// format (vertex, `distance_from_source`, parent). The path is computed using the parent back pointers. It is assumed
/// that there does exist a path, it is a programming error which will cause a panic if that is not true
pub fn path_from_visited(source: VertexId,
                         sink: VertexId,
                         node_parent_map: &[VertexId]) -> Vec<VertexId> {
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
pub trait FlowGraph {
    fn augmenting_path(&self, source: VertexId, sink: VertexId, search: Search) -> Option<Vec<VertexId>>;
    fn max_flow(&mut self, source: VertexId, sink: VertexId, search: Search) -> i32;
}

impl<'a> FlowGraph for Graph<FlowEdge> {
    /// Returns a path from source to sink if one exists that has non-zero flow.
    fn augmenting_path(&self, source: VertexId, sink: VertexId, search: Search) -> Option<Vec<VertexId>> {
        let iter = GraphIterator::new(self, source, sink, flow_predicate, search);
        let mut node_parent_map = vec![usize::MAX; self.n_vertexes()];
        let mut sink_exists = false;
        for node in iter {
            node_parent_map[node.0] = node.2;
            sink_exists = sink_exists || node.0 == sink;
        }
        if sink_exists {
            Some(path_from_visited(source, sink, &node_parent_map))
        } else {
            None
        }
    }

    /// Computes a vector of flow paths. Each path includes edges sequentially with the flow across that edge.
    fn max_flow(&mut self, source: VertexId, sink: VertexId, search: Search) -> i32 {
        let mut total_flow = 0;
        loop {
            let path_option: Option<Vec<VertexId>> = self.augmenting_path(source, sink, search);
            match path_option {
                Some(path) => {
                    let mut edges: Vec<Triplet<FlowEdge>> = Vec::new();
                    let mut flow: i32 = i32::MAX;
                    for i in 0..path.len() {
                        if i + 1 != path.len() {
                            let v_0 = path[i];
                            let v_1 = path[i + 1];
                            let flow_edge = self.edges[v_0][v_1];
                            edges.push(Triplet(v_0, flow_edge, v_1));
                            flow = min(flow_edge.capacity - flow_edge.flow, flow);
                        }
                    }
                    let mut flow_path: Vec<Edge> = Vec::new();
                    for edge in &edges {
                        {
                            let uv_edge = self.edges.get_mut(edge.0).unwrap().get_mut(edge.2).unwrap();
                            uv_edge.flow += flow;
                        }
                        {
                            let vu_edge = self.edges.get_mut(edge.2).unwrap().get_mut(edge.0).unwrap();
                            vu_edge.flow -= flow;
                        }
                        flow_path.push(Edge(edge.0, edge.2));
                    }
                },
                None => {
                    for v in &self.neighbors[source] {
                        if self.edges[source][*v].capacity != 0 {
                            total_flow += self.edges[source][*v].flow;
                        }
                    }
                    break;
                }
            }
        }

        total_flow
    }
}

pub fn create_residual_edges(edge_list: &mut Vec<(VertexId, VertexId, FlowEdge)>) {
    let mut residuals: Vec<(VertexId, VertexId, FlowEdge)> = Vec::with_capacity(edge_list.len());
    for e in edge_list.iter() {
        residuals.push((e.1, e.0, FlowEdge {capacity: 0, flow: 0}));
    }
    edge_list.extend(residuals);
}

pub fn flow_from_dicaps(file_name: &str) -> (VertexId, VertexId, Graph<FlowEdge>) {
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
    let vertexes = (0..num_vertexes).collect::<Vec<_>>();
    create_residual_edges(&mut edges);
    (source.expect("Must have a source"), sink.expect("Must have a sink"), Graph::new(&vertexes, &edges))
}

pub fn flow_from_txt(file_name: &str) -> (VertexId, VertexId, Graph<FlowEdge>) {
    let f = File::open(file_name).expect(&format!("Input file does not exist: {}", file_name));
    let reader = BufReader::new(&f);
    let mut edges: Vec<(VertexId, VertexId, FlowEdge)> = Vec::new();
    let mut i = 0;
    let mut num_vertexes = 0;
    let mut flow_parsed = false;
    for raw_line in reader.lines() {
        let line = raw_line.unwrap();
        let tokens = line.split_whitespace().collect::<Vec<_>>();
        if !flow_parsed {
            num_vertexes = tokens[0].parse::<usize>().expect("Expected an integer for number of edges");
            flow_parsed = true;
        } else {
            for v in tokens.iter().enumerate() {
                let capacity = v.1.parse::<i32>().expect("Expected an integer capacity");
                if capacity > 0 {
                    edges.push(
                        (i, v.0, FlowEdge{capacity: capacity, flow: 0})
                    );
                }
            }
            i += 1;
        }
    }
    let vertexes = (0..num_vertexes).collect::<Vec<_>>();
    create_residual_edges(&mut edges);
    (0, num_vertexes - 1, Graph::new(&vertexes, &edges))
}

fn true_predicate<E: Property>(_: E) -> bool {
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
        let vertex_list = vec![0, 1, 2, 3, 4];
        let edge_list = vec![(0, 1, 5), (0, 2, 2), (2, 3, 3), (4, 3, 1)];
        let g = Graph::new(&vertex_list, &edge_list);
        assert_eq!(g.size(), (5, 4));
        assert_eq!(g.n_vertexes(), vertex_list.len());
        assert_eq!(g.edges[0][1], 5);
        assert_eq!(g.edges[0][2], 2);
        assert_eq!(g.edges[2][3], 3);
        assert_eq!(g.edges[4][3], 1);
    }

    #[test]
    fn test_bfs() {
        let vertex_list = vec![0, 1, 2, 3, 4, 5];
        let edge_list = vec![(0, 1, 1), (1, 2, 1), (0, 3, 1), (3, 4, 1), (4, 1, 0), (4, 5, 1), (5, 2, 1)];
        let g = Graph::new(&vertex_list, &edge_list);
        let result: Vec<(VertexId, u32, VertexId)> = g.bfs_iter(0, 2).collect();
        let mut result_set = HashSet::new();
        result_set.extend(result);
        let mut expect = HashSet::new();
        expect.insert((0, 0, usize::MAX));
        expect.insert((1, 1, 0));
        expect.insert((2, 2, 1));
        expect.insert((3, 1, 0));
        assert_eq!(result_set, expect);
    }

    #[test]
    fn test_dfs() {
        let vertex_list = vec![0, 1, 2, 3, 4];
        let edge_list = vec![ (0, 3, 1), (0, 1, 1), (1, 2, 1), (2, 4, 1), (3, 4, 1)];
        let g = Graph::new(&vertex_list, &edge_list);
        let result: Vec<_> = g.dfs_iter(0, 4).collect();
        let mut result_set = HashSet::new();
        result_set.extend(result);
        let mut expect = HashSet::new();
        expect.insert((0, 0, usize::MAX));
        expect.insert((1, 1, 0));
        expect.insert((2, 2, 1));
        expect.insert((4, 3, 2));
        assert_eq!(result_set, expect);
    }

    #[test]
    fn test_augmenting_path() {
        let vertex_list = vec![0, 1, 2, 3, 4, 5, 6];
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

        assert_eq!(g.augmenting_path(0, 4, Search::Bfs).unwrap(), [0, 1, 3, 4]);

        {
            let edge = g.edges.get_mut(1).unwrap().get_mut(3).unwrap();
            edge.flow = 1;
        }
        assert_eq!(g.augmenting_path(0, 4, Search::Bfs), None);
    }

    #[test]
    fn test_path_from_visited() {
        let source = 0;
        let sink = 4;
        let visited = vec![(0, 0, usize::MAX), (1, 1, 0), (2, 1, 0), (5, 2, 1), (3, 2, 1), (4, 3, 3), (6, 2, 2)];
        let mut node_parent_map = vec![usize::MAX; 7];
        for n in visited {
            node_parent_map[n.0] = n.2;
        }
        let path = path_from_visited(source, sink, &node_parent_map);
        assert_eq!(path, [0, 1, 3, 4]);
    }

    #[test]
    fn test_max_flow_0() {
        let vertex_list = vec![0, 1, 2, 3, 4, 5, 6];
        let mut edge_list = vec![
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
        create_residual_edges(&mut edge_list);
        let mut g = Graph::new(&vertex_list, &edge_list);
        let total_flow = g.max_flow(0, 4, Search::Bfs);
        assert_eq!(total_flow, 4);
    }

    #[test]
    fn test_max_flow_1() {
        let vertex_list = vec![0, 1, 2, 3];
        let mut edge_list = vec![
            (0, 2, FlowEdge{flow: 0, capacity: 5}),
            (0, 3, FlowEdge{flow: 0, capacity: 5}),
            (2, 3, FlowEdge{flow: 0, capacity: 1}),
            (2, 1, FlowEdge{flow: 0, capacity: 5}),
            (3, 1, FlowEdge{flow: 0, capacity: 5}),
        ];
        create_residual_edges(&mut edge_list);
        let mut g = Graph::new(&vertex_list, &edge_list);
        let total_flow = g.max_flow(0, 1, Search::Bfs);
        assert_eq!(total_flow, 10);
    }

    #[test]
    fn test_max_flow_2() {
        let vertex_list = vec![0, 1, 2, 3, 4, 5];
        let mut edge_list = vec![
            (0, 1, FlowEdge{flow: 0, capacity: 11}),
            (0, 2, FlowEdge{flow: 0, capacity: 12}),
            (2, 1, FlowEdge{flow: 0, capacity: 1}),
            (1, 3, FlowEdge{flow: 0, capacity: 12}),
            (2, 4, FlowEdge{flow: 0, capacity: 11}),
            (4, 3, FlowEdge{flow: 0, capacity: 7}),
            (4, 5, FlowEdge{flow: 0, capacity: 4}),
            (3, 5, FlowEdge{flow: 0, capacity: 19}),
        ];
        create_residual_edges(&mut edge_list);
        let mut g = Graph::new(&vertex_list, &edge_list);
        let total_flow = g.max_flow(0, 5, Search::Bfs);
        assert_eq!(total_flow, 23);
    }

    enum FileType {
        Dicaps,
        Text
    }

    fn test_flow_from_file(file_name: &str, flow: i32, file_type: FileType, search: Search) {
        println!("Testing file: {}\n", file_name);
        let parsed = match file_type {
            FileType::Dicaps => flow_from_dicaps(file_name),
            FileType::Text => flow_from_txt(file_name)
        };
        let source = parsed.0;
        let sink = parsed.1;
        let mut g = parsed.2;
        println!("{:?}", g);
        let total_flow = g.max_flow(source, sink, search);
        assert_eq!(total_flow, flow);
        println!("");
    }

    #[test]
    fn test_maxflow_from_files() {
        test_flow_from_file("data/dicaps/flow-graph.txt", 10, FileType::Dicaps, BFS);
        test_flow_from_file("data/dicaps/flow-graph.txt", 10, FileType::Dicaps, DFS);
        test_flow_from_file("data/dicaps/bipartite-flow.txt", 3, FileType::Dicaps, BFS);
        test_flow_from_file("data/dicaps/bipartite-flow.txt", 3, FileType::Dicaps, DFS);
        test_flow_from_file("data/dicaps/central.txt", 5, FileType::Dicaps, BFS);
        test_flow_from_file("data/dicaps/central.txt", 5, FileType::Dicaps, DFS);
        test_flow_from_file("data/txt/test_1.txt", 10, FileType::Text, BFS);
        test_flow_from_file("data/txt/test_1.txt", 10, FileType::Text, DFS);
        test_flow_from_file("data/txt/test_2.txt", 23, FileType::Text, BFS);
        test_flow_from_file("data/txt/test_2.txt", 23, FileType::Text, DFS);
        test_flow_from_file("data/txt/test_3.txt", 2000000000, FileType::Text, BFS);
        test_flow_from_file("data/txt/test_3.txt", 2000000000, FileType::Text, DFS);
        test_flow_from_file("data/txt/test_4.txt", 23, FileType::Text, BFS);
        test_flow_from_file("data/txt/test_4.txt", 23, FileType::Text, DFS);
        test_flow_from_file("data/txt/test_5.txt", 256, FileType::Text, BFS);
        test_flow_from_file("data/txt/test_5.txt", 256, FileType::Text, DFS);
        test_flow_from_file("data/txt/test_6.txt", 20, FileType::Text, BFS);
        test_flow_from_file("data/txt/test_6.txt", 20, FileType::Text, DFS);
    }
}
