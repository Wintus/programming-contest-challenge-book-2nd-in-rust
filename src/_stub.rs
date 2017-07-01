use {Solvable, UnsolvableError};

struct Problem {
    premise: Premise
}

struct Premise {}

#[derive(Debug, PartialEq)]
struct Solution {}

impl Solvable for Problem {
    type I = Premise;
    type O = Solution;

    fn input(&self) -> &Premise {
        &self.premise
    }
    fn solve(&self) -> Result<Solution, UnsolvableError> {
        Ok(Solution {})
    }
}

#[test]
fn test_case_0() {
    let premise = Premise {};
    let solution = Solution {};
    let problem = Problem { premise: premise };
    assert_eq!(solution, problem.solve().unwrap());
}
