use {Solvable, UnsolvableError};

struct Problem {}

struct Premise {
    sum: u32,
    ns: Vec<u16>
}

impl Solvable for Problem {
    type I = Premise;
    type O = bool;

    fn solve(&self, premise: &Premise) -> Result<bool, UnsolvableError> {
        let sum = premise.sum as i32;
        let ns = &premise.ns;

        // 2 + 2 = 4
        let mut ns: Vec<_> =
            iproduct![ns, ns]
                .map(|(n0, n1)| n0 + n1)
                .collect();
        ns.sort(); // for binary search
        // make immutable
        let ns: &Vec<_> =
            &ns.iter()
               .map(|&n| n as i16)
               .collect();

        let winnable =
            ns.iter()
              .map(|&n| sum as i16 - n)
              .any(|n| match ns.binary_search(&n) {
                  Ok(_) => true,
                  _ => false
              });
        if winnable {
            Ok(winnable)
        } else {
            Err(UnsolvableError::NoSolution)
        }
    }
}

#[test]
fn test_case_0() {
    let premise = Premise {
        sum: 10,
        ns: vec![1, 3, 5]
    };
    assert!(Problem {}.solve(&premise).unwrap_or(false));
}

#[test]
fn test_case_1() {
    let premise = Premise {
        sum: 9,
        ns: vec![1, 3, 5]
    };
    assert!(!Problem {}.solve(&premise).unwrap_or(false));
}
