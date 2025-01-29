pub struct Graph {
    nodes: Vec<usize>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    pub fn iter_subgraph(self) -> impl Iterator<Item = Graph> {
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
            let edges: Vec<(usize, usize)> = self
                .edges
                .iter()
                .filter(|(a, b)| nodes.contains(a) && nodes.contains(b))
                .map(|edge| edge.clone())
                .collect();

            let subgraph = Self { nodes, edges };
            state += 1;
            return Some(subgraph);
        });
    }
}
