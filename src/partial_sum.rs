use {Solvable, UnsolvableError};

struct Problem {
    premise: Premise
}

struct Premise {
    ns: Vec<i32>/*len = 1..20*/,
    sum: i32,
}

impl Solvable for Problem {
    type I = Premise;
    type O = bool;

    fn input(&self) -> &Premise {
        &self.premise
    }

    fn solve(&self) -> Result<bool, UnsolvableError> {
        if self.dfs(0, 0) { Ok(true) } else { Err(UnsolvableError::NoSolution) }
    }
}

impl Problem {
    /// Deep First Search
    fn dfs(&self, i: usize, sum: i32) -> bool {
        let p = self.input();

        if i == p.ns.len() {
            sum == p.sum
        } else if self.dfs(i + 1, sum) {
            // don't use n
            true
        } else if self.dfs(i + 1, sum + *p.ns.get(i).unwrap_or(&0)) {
            // use n
            true
        } else {
            false
        }
    }
}

#[test]
fn test_case_0() {
    let premise = Premise {
        ns: vec![1, 2, 4, 7],
        sum: 13,
    };
    let problem = Problem { premise: premise };
    assert!(problem.solve().unwrap_or(false));
}

#[test]
fn test_case_1() {
    let premise = Premise {
        ns: vec![1, 2, 4, 7],
        sum: 15,
    };
    let problem = Problem { premise: premise };
    assert!(!problem.solve().unwrap_or(false));
}
