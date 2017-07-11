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
            let mut take_left = true;

            // 左と右のうち小さいほうを取る。
            // ただし同値ならば、内側のペアで比較する。
            let mut deq = deque.clone();
            while deq.len() >= 2 {
                let first = deq.pop_front().unwrap();
                let last = deq.pop_back().unwrap();
                match first.cmp(&last) {
                    Ordering::Less => {
                        take_left = true;
                        break;
                    },
                    Ordering::Greater => {
                        take_left = false;
                        break;
                    },
                    _ => true,
                };
            }

            let c = if take_left { deque.pop_front() } else { deque.pop_back() };
            line.push(c.unwrap());
        }
        Ok(line)
    }
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
