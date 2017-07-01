use {Solvable, UnsolvableError};

struct Problem {}

struct Premise {}

#[derive(Debug, PartialEq)]
struct Solution {}

impl Solvable for Problem {
    type I = Premise;
    type O = Solution;

    fn solve(&self, _: &Premise) -> Result<Solution, UnsolvableError> {
        Ok(Solution {})
    }
}

#[test]
fn test_case_0() {
    let problem = Problem {};
    let premise = Premise {};
    let solution = Solution {};
    assert_eq!(solution, problem.solve(&premise).unwrap());
}
