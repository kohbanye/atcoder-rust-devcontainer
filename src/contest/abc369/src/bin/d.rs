use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut dp_even: Vec<i64> = vec![0; n + 1];
    let mut dp_odd: Vec<i64> = vec![0; n + 1];

    dp_odd[0] = i64::MIN;
    for i in 0..n {
        dp_even[i + 1] = cmp::max(dp_even[i], dp_odd[i] + 2 * a[i]);
        dp_odd[i + 1] = cmp::max(dp_odd[i], dp_even[i] + a[i]);
    }

    println!("{}", cmp::max(dp_even[n], dp_odd[n]));
}
