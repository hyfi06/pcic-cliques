#[derive(Debug)]
pub struct Graph {
    nodes: Vec<usize>,
    adjacency: Vec<usize>,
}

impl Graph {
    pub fn new(input:(Vec<usize>, Vec<(usize,usize)>)) -> Self {
        let nodes = input.0;
        let edges = input.1;
        let n = nodes.len();
        
        let mut adjacency = vec![0; n]; // Cada índice es un nodo y almacena conexiones en bits
        
        for (u, v) in edges {
            adjacency[u] |= 1 << v;
            adjacency[v] |= 1 << u;
        }

        Self { nodes, adjacency }
    }
}
