use ac_library::ModInt998244353 as Mint;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut dp = vec![vec![vec![Mint::from(0); n + 1]; n + 1]; n + 1];
    for i in (0..n - 1).rev() {
        for j in (i..n).rev() {
            for len in 2..=n - j + 1 {
                if len == 2 {
                    dp[i][j][len] += Mint::from(1);
                    continue;
                }
                for k in j + 1..n {
                    if a[j] - a[i] == a[k] - a[j] {
                        let val = dp[j][k][len - 1];
                        dp[i][j][len] += val;
                    }
                }
            }
        }
    }

    let mut ans = vec![Mint::from(0); n + 1];
    ans[1] = Mint::from(n);
    for len in 2..=n {
        for i in 0..n {
            for j in i + 1..n {
                ans[len] += dp[i][j][len];
            }
        }
    }

    println!(
        "{}",
        ans.iter()
            .skip(1)
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
