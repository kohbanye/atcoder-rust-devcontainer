use bitvec::vec;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; m],
        a: [i32; m],
    }

    let mut stones = vec![0; n];
    for i in 0..m {
        stones[x[i]] = a[i];
    }

    let mut current_stone = 0;
    for i in 0..n {
        if stones[i] == 0 {
            current_stone -= 1;
        } else {
            current_stone += stones[i];
        }
        if current_stone < 0 {
            println!("-1");
            return;
        }
    }
}
