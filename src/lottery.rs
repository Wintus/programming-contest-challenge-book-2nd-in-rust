struct Problem {
    premise: Premise
}

struct Premise {
    sum: u32,
    ns: Vec<u16>
}

impl Problem {
    fn solve(&self) -> bool { false }
}

#[test]
fn test_case_0() {
    let premise = Premise {
        sum: 10,
        ns: vec![1, 3, 5]
    };
    let problem = Problem { premise: premise };
    assert!(problem.solve());
}

#[test]
fn test_case_1() {
    let premise = Premise {
        sum: 9,
        ns: vec![1, 3, 5]
    };
    let problem = Problem { premise: premise };
    assert!(!problem.solve());
}
