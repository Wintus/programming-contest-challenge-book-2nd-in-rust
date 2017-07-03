use {Solvable, Result};
use std::cmp;

const COIN_VALUES: [u16; 6] = [1, 5, 10, 50, 100, 500];

type Total = u32;
type Count = u32;
type CoinCounts = [u32; 6];

struct Problem {
    coins: CoinCounts,
    price: Total,
}

impl Solvable for Problem {
    type I = Self;
    type O = Count;

    fn input(&self) -> &Self::I { &self }

    /// 貪欲法
    fn solve(&self) -> Result<Self::O> {
        let mut price = self.price;
        let mut using_count = 0;
        for (&value, &count) in COIN_VALUES.iter().zip(self.coins.iter()).rev() {
            let value = value as u32;
            let count = cmp::min(count, price / value);
            using_count += count;
            price -= value * count;
        }
        debug_assert_eq!(0, price);
        Ok(using_count)
    }
}

#[test]
fn test_case_0() {
    let problem = Problem {
        coins: [3, 2, 1, 3, 0, 2],
        price: 620,
    };
    assert_eq!(6, problem.solve().unwrap());
}

#[test]
fn test_case_1() {
    let problem = Problem {
        coins: [1, 1, 4, 5, 1, 4],
        price: 810,
    };
    assert_eq!(7, problem.solve().unwrap());
}
