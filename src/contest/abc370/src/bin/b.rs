use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..=i {
            input! {
                num: Usize1,
            }
            a[i][j] = num;
        }
    }

    let mut element = 0;
    for i in 0..n {
        element = if i >= element {
            a[i][element]
        } else {
            a[element][i]
        };
    }

    println!("{}", element + 1);
}
