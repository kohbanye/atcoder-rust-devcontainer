use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let diffs = a.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i64>>();

    let mut cnt = 1;
    let mut ans = n as i64;
    for i in 0..n - 2 {
        if diffs[i] == diffs[i + 1] {
            cnt += 1;
        } else {
            ans += cnt * (cnt + 1) / 2;
            cnt = 1;
        }
    }
    ans += cnt * (cnt + 1) / 2;

    println!("{}", ans);
}
