use {Solvable, UnsolvableError};

use itertools::{Combinations, Itertools};

struct Problem {}

struct Premise {
    sides: Vec<u32>
}

const N_TRIANGLE_SIDES: u8 = 3;

impl Solvable for Problem {
    type I = Premise;
    type O = u32;

    fn solve(&self, premise: &Premise) -> Result<u32, UnsolvableError> {
        let ref sides = premise.sides;
        let combs: Combinations<_> =
            sides.into_iter()
                 .combinations(N_TRIANGLE_SIDES as usize);

        let max_perimeter = combs.filter_map(|ns| {
            let sides = ns.into_iter();
            let perimeter = sides.clone().sum::<u32>();
            let max_side = *sides.clone().max().unwrap();
            let rest = perimeter - max_side;

            if max_side < rest { Some(perimeter) } else { None }
        }).max();

        match max_perimeter {
            Some(n) => Ok(n),
            None => Err(UnsolvableError::NoSolution),
        }
    }
}

#[test]
fn test_case_0() {
    let premise = Premise { sides: vec![2, 3, 4, 5, 10] };
    assert_eq!(12, Problem {}.solve(&premise).unwrap_or(0));
}

#[test]
fn test_case_1() {
    let premise = Premise { sides: vec![4, 5, 10, 20] };
    assert_eq!(0, Problem {}.solve(&premise).unwrap_or(0)); // = no solution
}
