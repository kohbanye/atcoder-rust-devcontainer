use proconio::input;

fn main() {
    input! {
        n: usize,
        schedules: [(i32, i32); n],
    }

    let mut previous_time = 0;
    let mut current_amount = 0;
    for (time, amount) in schedules {
        current_amount -= time - previous_time;
        current_amount = current_amount.max(0);
        current_amount += amount;
        previous_time = time;
    }

    println!("{:?}", current_amount);
}
