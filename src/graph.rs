use std::f64::consts::PI;

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<usize>,
    pub adjacency: Vec<usize>,
    pub cliques: Vec<(usize, u32, u32)>,
}

impl Graph {
    pub fn new(input: (Vec<usize>, Vec<(usize, usize)>)) -> Self {
        let nodes = input.0;
        let edges = input.1;
        let n = nodes.len();

        let mut adjacency = vec![0; n];

        for (u, v) in edges {
            adjacency[u] |= 1 << v;
            adjacency[v] |= 1 << u;
        }

        Self {
            nodes,
            adjacency,
            cliques: Vec::new(),
        }
    }

    pub fn print_ascii_graph(&self, size: usize) {
        let n = self.nodes.len();
        let radius = size as f64 / 2.0;
        let center = (radius, radius);

        let mut grid = vec![vec![' '; size + 1]; size];

        let mut positions = vec![];
        for i in 0..n {
            let angle = 2.0 * PI * (i as f64) / (n as f64);
            let x = (center.0 + radius * angle.cos()).round() as usize;
            let y = (center.1 + radius * angle.sin()).round() as usize;
            positions.push((x, y));
            grid[y][x] = char::from_digit(self.nodes[i] as u32, 10).unwrap_or('?');
        }

        for (i, &adj) in self.adjacency.iter().enumerate() {
            let (x1, y1) = positions[i];

            for j in 0..n {
                if (adj >> j) & 1 == 1 {
                    let (x2, y2) = positions[j];
                    let dx = (x2 as isize - x1 as isize).abs();
                    let dy = (y2 as isize - y1 as isize).abs();
                    let steps = dx.max(dy);
                    for k in 0..=steps {
                        let px = x1 as isize + k * (x2 as isize - x1 as isize) / steps;
                        let py = y1 as isize + k * (y2 as isize - y1 as isize) / steps;
                        if (px, py) != (x1 as isize, y1 as isize)
                            && (px, py) != (x2 as isize, y2 as isize)
                        {
                            grid[py as usize][px as usize] = '.';
                        }
                    }
                }
            }
        }

        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
    pub fn iter_subgraph(&self) -> impl Iterator<Item = usize> {
        let mut subgraph: usize = 0;
        let max_state: usize = 1 << self.adjacency.len();
        return std::iter::from_fn(move || {
            if subgraph >= max_state {
                return None;
            }
            let result = subgraph.clone();
            subgraph += 1;
            return Some(result);
        });
    }
}
