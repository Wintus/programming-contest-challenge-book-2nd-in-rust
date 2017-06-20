extern crate itertools;

use itertools::Itertools;

struct Problem {
    premise: Premise
}

struct Premise {
    sides: Vec<u32>
}

const N_TRIANGLE_SIDES: u8 = 3;

impl Problem {
    fn solve(&self) -> u32 {
        let none = 0;
        let ref sides = self.premise.sides;

        let combs: itertools::Combinations<_> =
            sides.into_iter()
                 .combinations(N_TRIANGLE_SIDES as usize);

        let max_perimeter = combs
            .filter_map(|ns| {
                let sides = ns.into_iter();
                let perimeter = sides.clone().sum::<u32>();
                let max_side = *sides.clone().max().unwrap();
                let rest = perimeter - max_side;

                if max_side < rest { Some(perimeter) } else { None }
            })
            .max()
            .unwrap_or(none);

        max_perimeter
    }
}

#[test]
fn test_case_0() {
    let premise = Premise { sides: vec![2, 3, 4, 5, 10] };
    let problem = Problem { premise: premise };
    assert_eq!(12, problem.solve());
}

#[test]
fn test_case_1() {
    let premise = Premise { sides: vec![4, 5, 10, 20] };
    let problem = Problem { premise: premise };
    assert_eq!(0, problem.solve()); // = never
}
