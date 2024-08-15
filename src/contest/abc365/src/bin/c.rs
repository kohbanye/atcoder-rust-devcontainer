use std::cmp;

use proconio::input;

fn total_subsidy(subsidy: i64, expenses: Vec<i64>) -> i64 {
    expenses
        .iter()
        .map(|&expense| cmp::min(subsidy, expense))
        .sum()
}

fn main() {
    input! {
        n: i32,
        m: i64,
        expenses: [i64; n],
    }

    if expenses.iter().sum::<i64>() <= m {
        println!("infinite");
        return;
    }

    let mut ok = 0;
    let mut ng = (2e14 as i64) + 1;
    while ng - ok > 1 {
        let subsidy = (ok + ng) / 2;
        if total_subsidy(subsidy, expenses.clone()) <= m {
            ok = subsidy;
        } else {
            ng = subsidy;
        }
    }
    println!("{}", ok);
}
