use crate::graph::Graph;
use rayon::prelude::*;
use std::time::{Duration, Instant};

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

pub fn backtrack_find_cliques(graph: &Graph) {
    let mut subgraph: usize = 0;
    let n = graph.nodes.len();
    let mut cliques: Vec<usize> = Vec::new();
    let start = 0;
    let start_time = Instant::now();
    backtrack(n, start, &mut subgraph, &mut cliques, graph);
    let elapsed_time: Duration = start_time.elapsed();
    print_result(cliques, graph, elapsed_time);
}

fn backtrack(
    n: usize,
    start: usize,
    subgraph: &mut usize,
    cliques: &mut Vec<usize>,
    graph: &Graph,
) {
    if !is_clique(*subgraph, &graph.adjacency) {
        return;
    }
    cliques.push(*subgraph);
    for i in start..n {
        *subgraph |= 1 << i;
        backtrack(n, i + 1, subgraph, cliques, graph);
        *subgraph &= !(1 << i);
    }
}

pub fn find_cliques(graph: &Graph) {
    let start_time = Instant::now();

    let cliques: Vec<usize> = graph
        .iter_subgraph()
        .par_bridge()
        .filter(|subgraph| is_clique(*subgraph, &graph.adjacency))
        .collect();
    let elapsed_time: Duration = start_time.elapsed();

    print_result(cliques, graph, elapsed_time);
}

pub fn print_result(cliques: Vec<usize>, graph: &Graph, elapsed: Duration) {
    let n = graph.adjacency.len();
    let cliques: Vec<(usize, u32, u32)> = cliques
        .iter()
        .map(|subgraph| {
            let mut nodes: Vec<u32> = Vec::new();
            for i in 0..n {
                if (subgraph >> i) & 1 == 1 {
                    nodes.push(i as u32);
                }
            }
            (*subgraph, subgraph.count_ones(), nodes.iter().sum())
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
    println!("Elapsed time: {:.2?}", elapsed);
    println!("");
}
