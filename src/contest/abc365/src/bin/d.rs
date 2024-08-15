use proconio::input;

enum Hand {
    Rock,
    Paper,
    Scissors,
}

fn movable_paths(current_hand: &Hand, next_hand: &Hand) -> Vec<(usize, usize)> {
    match (current_hand, next_hand) {
        (Hand::Rock, Hand::Scissors)
        | (Hand::Scissors, Hand::Paper)
        | (Hand::Paper, Hand::Rock) => vec![(0, 0), (1, 0), (1, 1)],
        (Hand::Rock, Hand::Paper)
        | (Hand::Scissors, Hand::Rock)
        | (Hand::Paper, Hand::Scissors) => vec![(0, 0), (0, 1), (1, 1)],
        _ => vec![(0, 1), (1, 0)],
    }
}

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let opponent_hands = s
        .chars()
        .map(|c| match c {
            'R' => Hand::Rock,
            'S' => Hand::Scissors,
            'P' => Hand::Paper,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let mut dp: Vec<Vec<i32>> = vec![vec![0; 2]; n];
    dp[0][0] = 0;
    dp[0][1] = 1;
    for i in 0..n - 1 {
        let paths = movable_paths(&opponent_hands[i], &opponent_hands[i + 1]);
        for (current_j, next_j) in paths {
            dp[i + 1][next_j] =
                dp[i + 1][next_j].max(dp[i][current_j] + if next_j == 1 { 1 } else { 0 });
        }
    }

    let ans = dp[n - 1].iter().max().unwrap();
    println!("{}", ans);
}
