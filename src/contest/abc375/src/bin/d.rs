use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut char_index: HashMap<char, BTreeSet<usize>> = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        char_index.entry(c).or_insert(BTreeSet::new()).insert(i);
    }

    let mut cnt = 0;
    for set in char_index.values() {
        cnt -= set.len() as i64 * (set.len() as i64 - 1) / 2;
        for (i, v) in set.iter().enumerate() {
            cnt += (*v as i64) * (i as i64 - (set.len() as i64 - i as i64 - 1));
        }
    }

    println!("{}", cnt);
}
