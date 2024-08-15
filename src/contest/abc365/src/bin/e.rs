use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i64; n],
    }

    let mut xor_cumsum: Vec<i64> = vec![0; n as usize + 1];
    xor_cumsum[0] = 0;
    for i in 0..n as usize {
        xor_cumsum[i + 1] = xor_cumsum[i] ^ a[i];
    }

    let mut ans: i64 = 0;
    for bit in 0..60 {
        let mut one_count: i64 = 0;
        for s in &xor_cumsum {
            if s >> bit & 1 == 1 {
                one_count += 1;
            }
        }
        let zero_count = xor_cumsum.len() as i64 - one_count;
        ans += (1 << bit) * one_count * zero_count;
    }
    ans -= a.iter().sum::<i64>();

    println!("{}", ans);
}
