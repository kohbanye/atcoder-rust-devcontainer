use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ranges: [(i64, i64); n],
    }

    let sum_left = ranges.iter().map(|(l, _)| l).sum::<i64>();
    let sum_right = ranges.iter().map(|(_, r)| r).sum::<i64>();
    if sum_left > 0 || sum_right < 0 {
        println!("No");
        return;
    }

    let mut ans_vec = ranges.iter().map(|(l, _)| *l).collect::<Vec<i64>>();
    let mut sum = sum_left;
    for i in 0..n {
        if sum == 0 {
            break;
        }
        let (l, r) = ranges[i];
        let diff = cmp::min(r - l, -sum);
        ans_vec[i] += diff;
        sum += diff;
    }

    println!("Yes");
    println!(
        "{}",
        ans_vec
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
