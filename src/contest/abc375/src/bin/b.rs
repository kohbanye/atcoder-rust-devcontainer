use proconio::input;

fn main() {
    input! {
        n: usize,
        coords: [(f64, f64); n],
    }

    let mut cost = 0.;
    for i in 0..n - 1 {
        let dist = ((coords[i].0 - coords[i + 1].0).powi(2)
            + (coords[i].1 - coords[i + 1].1).powi(2))
        .sqrt();
        cost += dist;
    }
    cost += ((coords[0].0).powi(2) + (coords[0].1).powi(2)).sqrt();
    cost += ((coords[n - 1].0).powi(2) + (coords[n - 1].1).powi(2)).sqrt();

    println!("{}", cost);
}
