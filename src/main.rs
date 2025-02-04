use graph::Graph;
use cliques::find_cliques;

mod examples;
mod graph;
mod cliques;

fn main() {
    let mut graph_5: Graph = examples::load_5();
    find_cliques(&mut graph_5);

    let mut graph_10: Graph = examples::load_10();
    find_cliques(&mut graph_10);

    let mut graph_15: Graph = examples::load_15();
    find_cliques(&mut graph_15);

    let mut graph_25: Graph = examples::load_25();
    find_cliques(&mut graph_25);

    let mut graph_30: Graph = examples::load_30();
    find_cliques(&mut graph_30);
}


