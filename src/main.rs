mod graph;

fn main() {
    let cliques:Vec<graph::Graph> = graph.iter_subgraph()
    .filter(|subgraph| graph::is_clique(subgraph))
    .collect()
    
}
