use {Solvable, Result};
use std::cmp;

const N_COIN_TYPES: usize = 6;
const COIN_VALUES: [CoinValue; N_COIN_TYPES] = [1, 5, 10, 50, 100, 500];

type CoinValue = u16;
type Price = u32;
type Count = u32;
type GivenCoinCounts = [Count; N_COIN_TYPES];
type CoinCounts = [(CoinValue, Count); N_COIN_TYPES];

struct Problem {
    coins: GivenCoinCounts,
    total: Price,
}

impl Solvable for Problem {
    type I = Price;
    type O = CoinCounts;

    fn input(&self) -> &Self::I { &self.total }

    /// 貪欲法
    fn solve(&self) -> Result<Self::O> {
        let coin_values: &[CoinValue] = &COIN_VALUES;
        let rev_value_count_pairs = coin_values
            .iter()
            .zip(self.coins.iter())
            .rev();

        // state = price remaining & used coin count
        let price = *self.input();
        let counts = rev_value_count_pairs
            .scan((price, 0), |state, (&value, &count)| {
                let (ref mut remaining, _) = *state;
                let value = value as Price;
                let count = cmp::min(count, *remaining / value);
                *remaining -= value * count;
                let new_state = (*remaining, count);
                Some(new_state)
            });
        let counts: Vec<_> = counts.collect(); // fix size to rev
        let counts = counts.iter().rev();
        let used_coins: Vec<_> = coin_values
            .iter()
            .zip(counts)
            .map(|(&value, &(_, count))| (value, count))
            .collect();
        let mut used_coin_counts: CoinCounts = Default::default();
        used_coin_counts.copy_from_slice(used_coins.as_slice());
        Ok(used_coin_counts)
    }
}

impl Problem {
    fn count_coins(counts: CoinCounts) -> Count {
        counts.iter().map(|&(_, count)| count).sum()
    }
}

#[test]
fn test_case_0() {
    let problem = Problem {
        coins: [3, 2, 1, 3, 0, 2],
        total: 620,
    };
    let used_coins = problem.solve().unwrap();
    assert_eq!(6, Problem::count_coins(used_coins));
}

#[test]
fn test_case_1() {
    let problem = Problem {
        coins: [1, 1, 4, 5, 1, 4],
        total: 810,
    };
    let used_coins = problem.solve().unwrap();
    assert_eq!(7, Problem::count_coins(used_coins));
}
