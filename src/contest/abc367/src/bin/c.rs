use proconio::input;
use std::iter::Iterator;

struct MultiDimIndexIterator {
    current: Vec<usize>,
    ranges: Vec<usize>,
}

impl MultiDimIndexIterator {
    fn new(ranges: Vec<usize>) -> Self {
        MultiDimIndexIterator {
            current: vec![0; ranges.len()],
            ranges,
        }
    }
}

impl Iterator for MultiDimIndexIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_empty() {
            return None;
        }

        let result = self.current.clone();

        for i in (0..self.current.len()).rev() {
            if self.current[i] < self.ranges[i] - 1 {
                self.current[i] += 1;
                break;
            } else {
                self.current[i] = 0;
                if i == 0 {
                    self.current = Vec::new(); // 全ての次元が最大値に達した場合
                }
            }
        }

        Some(result)
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        ranges: [usize; n],
    }

    let mut all_possible_nums: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n {
        for possible in 1..=ranges[i] {
            all_possible_nums[i].push(possible);
        }
    }

    let mut all_possible_arrs: Vec<Vec<usize>> = vec![];
    let iterator = MultiDimIndexIterator::new(ranges);
    for index in iterator {
        let mut arr = vec![];
        for i in 0..n {
            arr.push(all_possible_nums[i][index[i]]);
        }
        all_possible_arrs.push(arr);
    }

    let mut answers: Vec<Vec<usize>> = vec![];
    for arr in all_possible_arrs {
        let sum: usize = arr.iter().sum();
        if sum % k == 0 {
            answers.push(arr);
        }
    }

    answers.sort();
    for answer in answers {
        println!(
            "{}",
            answer
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
