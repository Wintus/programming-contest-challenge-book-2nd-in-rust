use {Solvable, Result};
use std::cmp::Ordering;
use std::collections::VecDeque;

struct Problem<'a> {
    line: &'a str
}

impl<'a> Solvable for Problem<'a> {
    type I = &'a str;
    type O = String;

    fn input(&self) -> &Self::I { &self.line }

    /// 貪欲法
    fn solve(&self) -> Result<Self::O> {
        let mut line = String::new();

        let mut deque: VecDeque<char> = self.input().chars().collect();
        while !deque.is_empty() {
            // 左と右のうち小さいほうを取る。
            // ただし同値ならば、内側のペアで比較する。
            let deq = deque.clone();
            let pairs = fold_half_once(deq);
            let left = pairs.into_iter()
                            .map(|(first, last)| first.cmp(&last))
                            .find(|&ord| ord != Ordering::Equal)
                            .map(|ord| ord < Ordering::Equal)
                            .unwrap_or(true);
            let c = if left { deque.pop_front() } else { deque.pop_back() };
            line.push(c.unwrap());
        }
        Ok(line)
    }
}

/// arg will be moved
fn fold_half_once<T>(vec: VecDeque<T>) -> Vec<(T, T)> {
    let mut vec: Vec<_> = vec.into_iter().collect();
    let len = vec.len() / 2;
    let last_half = vec.split_off(len);
    let from_last = last_half.into_iter().rev();
    let from_first = vec.into_iter();
    let pairs = from_first.zip(from_last);
    pairs.collect()
}

#[test]
fn test_fold_half_once() {
    let deque: VecDeque<_> = (0..4).collect();
    let pairs = fold_half_once(deque);
    assert_eq!(vec![(0, 3), (1, 2)], pairs);
}


#[test]
fn test_case_0() {
    let p = Problem { line: "ACDBCB" };
    assert_eq!("ABCBCD", &p.solve().unwrap_or("".to_string()));
}

#[test]
fn test_case_1() {
    let p = Problem { line: "ABCCBA" };
    assert_eq!("AABBCC", &p.solve().unwrap_or("".to_string()));
}
