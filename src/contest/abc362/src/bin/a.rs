use std::cmp;

use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        c: String,
    }

    match c.as_str() {
        "Red" => {
            println!("{}", cmp::min(g, b));
        },
        "Green" => {
            println!("{}", cmp::min(r, b));
        },
        "Blue" => {
            println!("{}", cmp::min(r, g));
        },
        _ => unreachable!(),
    }
}
