use proconio::input;

fn main() {
    input! {
        n: usize,
        max_sweetness: i64,
        max_sourness: i64,
        mut sweetnesses: [i64; n],
        mut sournesses: [i64; n],
    }

    sweetnesses.sort();
    sweetnesses.reverse();
    sournesses.sort();
    sournesses.reverse();

    let mut sum_sweet = 0;
    let mut sum_sour = 0;
    for i in 0..n {
        sum_sweet += sweetnesses[i];
        sum_sour += sournesses[i];
        if sum_sweet > max_sweetness || sum_sour > max_sourness {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", n);
}
