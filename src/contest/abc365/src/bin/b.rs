use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [i64; n],
    }

    let max = arr.iter().max().unwrap();
    let second_max = arr.iter().filter(|&x| x != max).max().unwrap();
    let second_max_idx = arr.iter().position(|&x| x == *second_max).unwrap();
    println!("{}", second_max_idx + 1);
}
