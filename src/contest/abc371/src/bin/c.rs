use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn make_graph(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n];
    for &(u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m_g: usize,
        edges_g: [(Usize1, Usize1); m_g],
        m_h: usize,
        edges_h: [(Usize1, Usize1); m_h],
    }

    let mut costs = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        for j in i + 1..n {
            input! {
                cost: i32,
            }
            costs[i][j] = cost;
            costs[j][i] = cost;
        }
    }

    let graph_g = make_graph(n, &edges_g);
    let graph_h = make_graph(n, &edges_h);

    let mut min_cost = std::i32::MAX;
    for mapping in (0..n).permutations(n) {
        let mut new_graph_h = vec![vec![]; n];
        for from in 0..n {
            for &to in &graph_h[from] {
                new_graph_h[mapping[from]].push(mapping[to]);
            }
        }

        let mut current_cost = 0;
        for from in 0..n {
            for to in 0..n {
                if from == to {
                    continue;
                }

                if graph_g[from].contains(&to) && !new_graph_h[from].contains(&to) {
                    current_cost += costs[from][to];
                    new_graph_h[from].push(to);
                    new_graph_h[to].push(from);
                }
                if new_graph_h[from].contains(&to) && !graph_g[from].contains(&to) {
                    current_cost += costs[from][to];
                    let index = new_graph_h[from].iter().position(|&x| x == to).unwrap();
                    new_graph_h[from].remove(index);
                    let index = new_graph_h[to].iter().position(|&x| x == from).unwrap();
                    new_graph_h[to].remove(index);
                }
            }
        }
        min_cost = min_cost.min(current_cost);
    }

    println!("{}", min_cost);
}
