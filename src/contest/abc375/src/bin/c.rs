use proconio::input;

fn main() {
    input! {
        n: usize,
        grid: [String; n],
    }

    let grid = grid
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut new_grid = vec![vec!['.'; n]; n];

    // 90 deg
    // (x, y) -> (y, n - x - 1)
    // 180 deg
    // (x, y) -> (n - x - 1, n - y - 1)
    // 270 deg
    // (x, y) -> (n - y - 1, x)

    for x in 0..n {
        for y in 0..n {
            let values = [x, y, n - x - 1, n - y - 1];
            let i = values.iter().min().unwrap();
            match i % 4 {
                0 => new_grid[y][n - x - 1] = grid[x][y],
                1 => new_grid[n - x - 1][n - y - 1] = grid[x][y],
                2 => new_grid[n - y - 1][x] = grid[x][y],
                3 => new_grid[x][y] = grid[x][y],
                _ => unreachable!(),
            }
        }
    }

    for s in new_grid {
        println!("{}", s.iter().collect::<String>());
    }
}
