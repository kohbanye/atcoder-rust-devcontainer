use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        d: i32,
        map: [String; h],
    }

    let map = map
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut dist = vec![vec![-1; w]; h];

    let humidifier_coords = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == 'H')
                .map(move |(j, _)| (i, j))
        })
        .collect::<Vec<(usize, usize)>>();

    let mut queue = VecDeque::<(usize, usize)>::new();
    humidifier_coords.iter().for_each(|&coord| {
        queue.push_back(coord);
        dist[coord.0][coord.1] = 0;
    });

    while let Some((i, j)) = queue.pop_front() {
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if map[ni][nj] == '.' && dist[ni][nj] == -1 && dist[i][j] < d {
                dist[ni][nj] = dist[i][j] + 1;
                queue.push_back((ni, nj));
            }
        }
    }

    println!("{:?}", dist.iter().flatten().filter(|&&d| d != -1).count());
}
