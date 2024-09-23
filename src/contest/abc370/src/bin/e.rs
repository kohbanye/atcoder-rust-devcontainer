use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut sums = vec![0; n + 1];
    for i in 0..n {
        sums[i + 1] = (sums[i] + a[i]) % MOD;
    }

    let mut ranges: Vec<(usize, usize)> = vec![];
    let mut left = 0;
    let mut right = 0;
    while left < n {
        while right < n && sums[left] - sums[right] <= k {
            right += 1;
        }
        ranges.push((left, right));
        left = right;
    }
}
