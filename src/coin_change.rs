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
        let price = self.price;
        let coin_values: &[u16] = &COIN_VALUES;

        let rev_value_count_pairs = coin_values
            .iter()
            .zip(self.coins.iter())
            .rev();
        // state = price remaining & used coin count
        let counts = rev_value_count_pairs
            .scan((price, 0), |state, (&value, &count)| {
                println!("state: {:?}", state);
                let (ref mut remaining, _) = *state;
                let value = value as u32;
                let count = cmp::min(count, *remaining / value);
                *remaining -= value * count;
                Some((*remaining, count))
            });
        let counts: Vec<_> = counts.collect();
        println!("calc counts: {:?}", counts);
        let used_coins: Vec<_> = coin_values
            .iter()
            .rev()
            .zip(counts)
            .map(|(value, (_, count))| (value, count))
            .collect();
        println!("used coins: {:?}", used_coins);
        let count = used_coins
            .iter()
            .map(|&(_, count)| count)
            .sum();
        Ok(count)
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
