struct Problem {
    premise: Premise
}

struct Premise {
    ns: Vec<i32>/*len = 1..20*/,
    sum: i32,
}

impl Problem {
    fn solve(&self) -> bool { false }
}

#[test]
fn test_case_0() {
    let premise = Premise {
        ns: vec![1, 2, 4, 7],
        sum: 13,
    };
    let problem = Problem { premise: premise };
    assert!(problem.solve());
}

#[test]
fn test_case_1() {
    let premise = Premise {
        ns: vec![1, 2, 4, 7],
        sum: 15,
    };
    let problem = Problem { premise: premise };
    assert!(!problem.solve());
}
