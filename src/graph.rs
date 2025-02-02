pub struct Graph {
    nodes: Vec<usize>,
    adjacency: Vec<usize>,
}

pub struct SubGraph {
    nodes: Vec<usize>,
    adjacency: &Vec<usize>,
}

impl Graph {
    pub fn new() -> Self {
        
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
