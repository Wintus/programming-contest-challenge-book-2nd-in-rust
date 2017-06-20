extern crate itertools;

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
        let ns = &self.premise.ns;

        // 2 + 2 = 4
        let mut ns = iproduct![ns, ns]
            .map(|(n0, n1)| n0 + n1)
            .collect::<Vec<_>>();
        ns.sort(); // for binary search
        let ns: &Vec<_> = &ns.iter()
                             .map(|&n| n as i16)
                             .collect();

        ns.iter()
          .map(|&n| sum as i16 - n)
          .any(|n| match ns.binary_search(&n) {
              Ok(_) => true,
              _ => false
          })
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
