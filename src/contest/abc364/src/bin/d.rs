use proconio::input;
use superslice::Ext;

fn count_elements(sorted_arr: &[i64], left: i64, right: i64) -> usize {
    let left_idx = sorted_arr.lower_bound(&left);
    let right_idx = sorted_arr.upper_bound(&right);
    right_idx - left_idx
}

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        queries: [(i64, usize); q],
    }

    a.sort();

    let mut answers = Vec::new();
    for (b, k) in queries {
        let mut ok = 2e8 as i64;
        let mut ng = -1;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            let count = count_elements(&a, b - mid, b + mid);
            if count >= k {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        answers.push(ok);
    }

    for ans in answers {
        println!("{}", ans);
    }
}
