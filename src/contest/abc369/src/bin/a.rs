use itertools::Itertools;
use proconio::input;

const MAX: i32 = 10000;

fn is_arithmetic_seq(v: Vec<&i32>) -> bool {
    v[1] - v[0] == v[2] - v[1]
}

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let mut cnt = 0;
    for x in -MAX..MAX {
        for vector in [a, b, x].iter().permutations(3) {
            if is_arithmetic_seq(vector) {
                cnt += 1;
                break;
            }
        }
    }

    println!("{}", cnt);
}
