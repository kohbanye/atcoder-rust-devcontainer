use proconio::input;

fn main() {
    input! {
        r: usize,
    }

    let ranges = vec![(1, 100), (100, 200), (200, 300), (300, 400)];

    for (min, max) in ranges {
        if min <= r && r < max {
            println!("{}", max - r);
            return;
        }
    }
}
