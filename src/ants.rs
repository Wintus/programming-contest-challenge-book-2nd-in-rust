use std::cmp;

struct Problem {
    premise: Premise
}

struct Premise {
    rod: Rod,
    ants: Vec<Ant>,
}

#[derive(Debug)]
#[derive(Eq)]
struct Solution {
    min: u32,
    max: u32,
}

const SPEED_OF_ANT: u32 = 1; // cm/s

struct Rod {
    length: u32
}

struct Ant {
    x: u32
}

impl Problem {
    fn solve(&self) -> Solution {
        Solution {
            min: 0,
            max: 0,
        }
    }
}

impl cmp::PartialEq for Solution {
    fn eq(&self, other: &Solution) -> bool {
        self.min == other.min && self.max == other.max
    }
}

#[test]
fn test_case_0() {
    let premise = Premise {
        rod: Rod { length: 10 },
        ants: vec![
            Ant { x: 2 },
            Ant { x: 6 },
            Ant { x: 7 },
        ]
    };
    let solution = Solution {
        // [-1, 1, 1]
        min: 4,
        // [1, 1, 1]
        max: 8,
    };
    let problem = Problem { premise: premise };
    assert_eq!(solution, problem.solve());
}
