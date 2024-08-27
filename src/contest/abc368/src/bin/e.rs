use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x1: usize,
        trains: [(Usize1, Usize1, usize, usize); m],
    }

    let mut graph = vec![vec![]; m];
    // for i in 0..m {
    //     for j in i + 1..m {
    //         let (_, arrive_to, _, arrival_time) = trains[i];
    //         let (depart_from, _, departure_time, _) = trains[j];
    //         if arrive_to == depart_from && arrival_time <= departure_time {
    //             graph[i].push(j);
    //         }
    //     }
    // }

    let mut trains_arrive_to = vec![vec![]; n];
    let mut trains_depart_from = vec![vec![]; n];
    for (i, train) in trains.iter().enumerate() {
        let (arrive_to, depart_from, _, _) = train;
        trains_arrive_to[*arrive_to].push(i);
        trains_depart_from[*depart_from].push(i);
    }

    for i in 0..n {
        for &train1 in trains_arrive_to[i].iter() {
            for &train2 in trains_depart_from[i].iter() {
                let (_, _, arrival_time, _) = trains[train1];
                let (_, _, _, departure_time) = trains[train2];
                if arrival_time <= departure_time {
                    graph[train2].push(train1);
                }
            }
        }
    }
    println!("{:?}", graph);

    let mut delays = vec![0; m];
    delays[0] = x1;

    let mut queue = VecDeque::new();
    let mut visited = vec![false; m];
    queue.push_back(0);
    while let Some(current) = queue.pop_front() {
        if visited[current] {
            continue;
        }
        visited[current] = true;

        for next in graph[current].iter() {
            let arrival_time = trains[current].3;
            let departure_time = trains[*next].2;
            if arrival_time + delays[current] > departure_time + delays[*next] {
                delays[*next] = arrival_time + delays[current] - departure_time;
            }
            queue.push_back(*next);
        }
    }

    println!("{}", delays.iter().skip(1).join(" "));
}
