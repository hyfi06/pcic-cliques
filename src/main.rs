use graph::Graph;
use cliques::backtrack_find_cliques;

mod examples;
mod graph;
mod cliques;

fn main() {
    let mut graph_5: Graph = examples::load_5();
    backtrack_find_cliques(&mut graph_5);

    let mut graph_10: Graph = examples::load_10();
    backtrack_find_cliques(&mut graph_10);

    let mut graph_15: Graph = examples::load_15();
    backtrack_find_cliques(&mut graph_15);

    let mut graph_25: Graph = examples::load_25();
    backtrack_find_cliques(&mut graph_25);

    let mut graph_30: Graph = examples::load_30();
    backtrack_find_cliques(&mut graph_30);
}


