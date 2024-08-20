use itertools::all;
use permutohedron::LexicalPermutation;
use proconio::input;

fn is_palindrome(s: &Vec<char>) -> bool {
    let n = s.len();
    for i in 0..n / 2 {
        if s[i] != s[n - i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: String,
    }

    let mut s = s.chars().collect::<Vec<char>>();
    s.sort();
    let mut cnt = 0;
    loop {
        if all(s.windows(k), |slice| !is_palindrome(&slice.to_vec())) {
            cnt += 1;
        }
        if !s.next_permutation() {
            break;
        }
    }

    println!("{}", cnt);
}
