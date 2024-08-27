use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a[n - k..n].iter().for_each(|&x| print!("{} ", x));
    a[0..n - k - 1].iter().for_each(|&x| print!("{} ", x));
    println!("{}", a[n - k - 1]);
}
