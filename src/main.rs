use graph::Graph;
use rayon::prelude::*;
use std::time::Instant;

mod examples;
mod graph;

fn main() {
    let mut graph_5: Graph = examples::load_5();
    find_cliques(&mut graph_5);

    let mut graph_10: Graph = examples::load_10();
    find_cliques(&mut graph_10);

    let mut graph_15: Graph = examples::load_15();
    find_cliques(&mut graph_15);

    let mut graph_20: Graph = examples::load_20();
    find_cliques(&mut graph_20);

    let mut graph_25: Graph = examples::load_25();
    find_cliques(&mut graph_25);

    let mut graph_30: Graph = examples::load_30();
    find_cliques(&mut graph_30);
}

pub fn is_clique(subgraph: usize, adjacency: &Vec<usize>) -> bool {
    let mut state = true;
    let mut nodes: Vec<usize> = Vec::new();
    let n = adjacency.len();
    for i in 0..n {
        if (subgraph >> i) & 1 == 1 {
            nodes.push(i);
        }
    }
    for (idx, i) in nodes.iter().enumerate() {
        for j in nodes.iter().skip(idx + 1) {
            state &= (adjacency[*i] >> *j) & 1 == 1;
            if !state {
                break;
            }
        }
        if !state {
            break;
        }
    }
    state
}

fn find_cliques(graph: &mut Graph) {
    let n = graph.adjacency.len();
    let start_time = Instant::now();

    let cliques: Vec<(usize, u32, u32)> = graph
        .iter_subgraph()
        .par_bridge()
        .filter(|subgraph| is_clique(*subgraph, &graph.adjacency))
        .map(|subgraph| {
            let mut nodes: Vec<u32> = Vec::new();
            for i in 0..n {
                if (subgraph >> i) & 1 == 1 {
                    nodes.push(i as u32);
                }
            }
            (subgraph, subgraph.count_ones(), nodes.iter().sum())
        })
        .collect();
    let max_clique = cliques.iter().map(|(_, size, _)| *size).max().unwrap_or(0);
    let max_weight = cliques
        .iter()
        .map(|(_, _, weight)| *weight)
        .max()
        .unwrap_or(0);
    let max_cliques: Vec<&usize> = cliques
        .iter()
        .filter(|(_, size, _)| *size == max_clique)
        .map(|(subgraph, _, _)| subgraph)
        .collect();
    let max_weight_cliques: Vec<&usize> = cliques
        .iter()
        .filter(|(_, _, weight)| *weight == max_weight)
        .map(|(subgraph, _, _)| subgraph)
        .collect();
    let elapsed_time = start_time.elapsed();

    println!("{}", "=".repeat(50));
    graph.print_ascii_graph(80);
    println!("{:?}", graph.nodes);
    println!("Cardinalidad de cliques máximos: {}.", max_clique);
    print!("{} cliques máximos: ", max_cliques.len());
    max_cliques
        .iter()
        .for_each(|subgraph| print!("| {:b} ", *subgraph));
    println!("|");
    println!("");
    println!("Peso máximo: {}.", max_weight);
    print!("{} cliques de peso máximo: ", max_weight_cliques.len(),);
    max_weight_cliques
        .iter()
        .for_each(|subgraph| print!("| {:b} ", *subgraph));
    println!("|");
    println!("");
    println!("Elapsed time: {:.2?}", elapsed_time);
    println!("");
}
