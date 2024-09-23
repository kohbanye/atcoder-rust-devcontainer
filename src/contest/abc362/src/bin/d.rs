use proconio::{input, marker::Usize1};

#[derive(Clone)]
struct Edge {
    to: usize,
    weight: i64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct MinDist {
    dist: i64,
    from: usize,
}

fn main() {
    input! {
        n: usize,
        m: usize,
        v_weights: [i64; n],
        edges: [(Usize1, Usize1, i64); m],
    }

    let mut graph: Vec<Vec<Edge>> = vec![vec![]; n];
    for (u, v, w) in edges {
        graph[u].push(Edge { to: v, weight: w });
        graph[v].push(Edge { to: u, weight: w });
    }

    let mut distances = vec![std::i64::MAX; n];
    let mut queue = std::collections::BinaryHeap::new();
    distances[0] = v_weights[0];
    queue.push(std::cmp::Reverse(MinDist {
        dist: v_weights[0],
        from: 0,
    }));

    while let Some(std::cmp::Reverse(MinDist { dist, from })) = queue.pop() {
        if distances[from] < dist {
            continue;
        }
        for edge in &graph[from] {
            let new_dist = dist + v_weights[edge.to] + edge.weight;
            if new_dist < distances[edge.to] {
                distances[edge.to] = new_dist;
                queue.push(std::cmp::Reverse(MinDist {
                    dist: new_dist,
                    from: edge.to,
                }));
            }
        }
    }

    println!(
        "{}",
        distances
            .iter()
            .skip(1)
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
