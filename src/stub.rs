use std::cmp;

struct Problem {
    premise: Premise
}

struct Premise {}

#[derive(Debug)]
#[derive(Eq)]
struct Solution {}

impl cmp::PartialEq for Solution {
    fn eq(&self, other: &Solution) -> bool {
        let _other = other;
        true
    }
}

impl Problem {
    fn solve(&self) -> Solution {
        Solution {}
    }
}

#[test]
fn test_case_0() {
    let premise = Premise {};
    let solution = Solution {};
    let problem = Problem { premise: premise };
    assert_eq!(solution, problem.solve());
}
