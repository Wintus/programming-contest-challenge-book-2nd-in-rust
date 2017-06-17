struct Problem {
    premise: Premise
}

struct Premise {
    sides: Vec<u32>
}

impl Problem {
    fn solve(&self) -> bool {}
}

#[test]
fn test_case_0() {
    let premise = Premise { sides: vec![2, 3, 4, 5, 10] };
    let problem = Problem { premise: premise };
    assert_eq!(12, problem.solve());
}
