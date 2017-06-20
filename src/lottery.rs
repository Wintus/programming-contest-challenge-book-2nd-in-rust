struct Problem {
    premise: Premise
}

struct Premise {
    sum: u32,
    ns: Vec<u16>
}

impl Problem {
    fn solve(&self) -> bool {
        let sum = self.premise.sum;
        let ns = &self.premise.ns;

        for n0 in ns {
            for n1 in ns {
                for n2 in ns {
                    for n3 in ns {
                        let ns: Vec<_> = vec![n0, n1, n2, n3];
                        if ns.into_iter().sum::<u16>() as u32
                            == sum { return true }
                    }
                }
            }
        }
        false
    }
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
