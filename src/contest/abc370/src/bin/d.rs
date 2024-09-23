use proconio::input;

fn is_in_range(range: (i64, i64), target: i64) -> bool {
    range.0 <= target && target <= range.1
}

fn main() {
    input! {
        h: i64,
        w: i64,
        q: usize,
        queries: [(i64, i64); q],
    }

    let mut broken_ranges_row = vec![(0, 0); h as usize];
    let mut broken_ranges_col = vec![(0, 0); w as usize];
    for (row, col) in queries {
        let row = row as usize - 1;
        let col = col as usize - 1;

        if !is_in_range(broken_ranges_row[row], col as i64)
            && !is_in_range(broken_ranges_col[col], row as i64)
        {
            broken_ranges_row[row] = (col as i64, col as i64);
            broken_ranges_col[col] = (row as i64, row as i64);
            continue;
        }
    }

    // let mut broken_grids: HashSet<(i64, i64)> = HashSet::new();
    // let mut ans = 0;
    // for (i, j) in queries {
    //     if !broken_grids.contains(&(i, j)) {
    //         ans += 1;
    //         broken_grids.insert((i, j));
    //         continue;
    //     }

    //     let diffs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    //     let mut empty_cnt = 0;
    //     for (di, dj) in diffs {
    //         let ni = i + di;
    //         let nj = j + dj;
    //         if ni < 1 || ni > h || nj < 1 || nj > w {
    //             continue;
    //         }

    //         if broken_grids.contains(&(ni, nj)) {
    //             empty_cnt += 1;
    //         }
    //     }

    //     ans += 4 - empty_cnt;
    // }
}
