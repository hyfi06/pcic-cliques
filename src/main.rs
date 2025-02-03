use graph::Graph;

mod examples;
mod graph;

fn main() {
    let mut graph_5: Graph = examples::load_5();
    graph_5.print_ascii_graph(50);
    find_cliques(&mut graph_5);
    // println!("{}", is_clique(29, &graph_5.adjacency));
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
    graph.iter_subgraph().for_each(|subgraph| {
        if is_clique(subgraph, &graph.adjacency) {
            let mut nodes: Vec<u32> = Vec::new();
            for i in 0..n {
                if (subgraph >> i) & 1 == 1 {
                    nodes.push(i as u32);
                }
            }
            graph
                .cliques
                .push((subgraph, subgraph.count_ones(), nodes.iter().sum()));
        }
    });
    graph.cliques.iter().for_each(|(subgraph, size, weight)| {
        println!("{:b} n: {} w: {}", *subgraph, *size, *weight)
    });
}
