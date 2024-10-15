use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        babies: [(Usize1, char); m],
    }

    let mut has_boy = vec![false; n];
    for (family, gender) in babies {
        if !has_boy[family] && gender == 'M' {
            println!("Yes");
            has_boy[family] = true;
        } else {
            println!("No");
        }
    }
}
