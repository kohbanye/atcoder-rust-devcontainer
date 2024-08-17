use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        (initial_h, initial_w): (usize, usize),
        grid: [String; h],
        query: String,
    }

    let grid = grid
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut current_h = initial_h - 1;
    let mut current_w = initial_w - 1;

    for q in query.chars() {
        match q {
            'L' => {
                if current_w > 0 && grid[current_h][current_w - 1] != '#' {
                    current_w -= 1;
                }
            }
            'R' => {
                if current_w < w - 1 && grid[current_h][current_w + 1] != '#' {
                    current_w += 1;
                }
            }
            'U' => {
                if current_h > 0 && grid[current_h - 1][current_w] != '#' {
                    current_h -= 1;
                }
            }
            'D' => {
                if current_h < h - 1 && grid[current_h + 1][current_w] != '#' {
                    current_h += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{} {}", current_h + 1, current_w + 1);
}
