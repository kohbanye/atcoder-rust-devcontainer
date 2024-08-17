use proconio::input;

fn main() {
    input! {
        n: usize,
        strings: [String; n],
    }

    for i in 1..n - 1 {
        if strings[i - 1] == "sweet" && strings[i] == "sweet" {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
