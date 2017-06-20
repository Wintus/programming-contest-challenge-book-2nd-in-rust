struct Problem {
    premise: Premise
}

struct Premise {
    sum: u32,
    ns: Vec<u16>
}

impl Problem {
    fn solve(&self) -> bool {
        let sum = self.premise.sum as i32;
        let ns = &mut self.premise.ns.clone();
        ns.sort(); // for binary search
        let ns = &ns.iter()
                    .map(|&n| n as i16)
                    .collect::<Vec<_>>();

        for n0 in ns {
            for n1 in ns {
                for n2 in ns {
                    let ns3: Vec<_> = vec![n0, n1, n2];
                    let sum3 = ns3.into_iter().sum::<i16>() as i32;
                    let n = (sum - sum3) as i16;
                    if let Ok(_) = ns.binary_search(&n) { return true }
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
