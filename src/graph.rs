pub struct Graph {
    nodes: Vec<usize>,
    adjacency: Vec<usize>,
}


pub struct SubGraph<'a> {
    nodes: Vec<usize>,
    adjacency: &'a Vec<usize>,
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
    
    pub fn iter_subgraph(self) -> impl Iterator<Item = SubGraph> {
        let mut state: usize = 0;
        let max_state: usize = 1 << self.edges.len();
        return std::iter::from_fn(move || {
            if state >= max_state {
                return None;
            }
            let nodes: Vec<usize> = self
                .nodes
                .iter()
                .enumerate()
                .filter(|(i, _)| (state >> i) & 1 == 1)
                .map(|(_, item)| item.clone())
                .collect();

            let subgraph = SubGraph { nodes, &self.adjacency };
            state += 1;
            return Some(subgraph);
        });
    }
}

pub fn is_clique(graph: Graph) -> bool {}
