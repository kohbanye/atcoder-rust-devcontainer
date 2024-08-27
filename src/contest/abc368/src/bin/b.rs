use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut cnt = 0;
    while a.iter().filter(|&x| *x > 0).count() > 1 {
        a.sort_by(|x, y| y.cmp(x));
        a[0] -= 1;
        a[1] -= 1;
        cnt += 1;
    }
    println!("{}", cnt);
}
