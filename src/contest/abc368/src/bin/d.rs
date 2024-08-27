use proconio::{input, marker::Usize1};

struct Graph {
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize, pairs: &[(usize, usize)]) -> Self {
        let mut edges = vec![vec![]; n];
        for &(a, b) in pairs {
            edges[a].push(b);
            edges[b].push(a);
        }
        Self { edges }
    }

    fn subgraph_contains(
        &self,
        targets: &[usize],
        results: &mut [bool],
        current: usize,
        prev: usize,
    ) -> bool {
        if targets.binary_search(&current).is_ok() {
            results[current] = true;
        }
        for &next in self.edges[current].iter() {
            if next == prev {
                continue;
            }
            if results[next] || self.subgraph_contains(targets, results, next, current) {
                results[current] = true;
            }
        }
        results[current]
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(Usize1, Usize1); n - 1],
        mut targets: [Usize1; k],
    }

    let graph = Graph::new(n, &ab);

    targets.sort();
    let mut results = vec![false; n];
    graph.subgraph_contains(&targets, &mut results, targets[0], n);

    println!("{}", results.iter().filter(|&&x| x).count());
}
