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
        // alias names
        let ref rod = self.premise.rod;
        let ref ants = self.premise.ants;

        let min = ants.into_iter().fold(0, |min, ref ant| {
            let l = cmp::min(ant.x, &rod.length - ant.x);
            cmp::max(min, l)
        });
        let max = ants.into_iter().fold(0, |max, ref ant| {
            let l = cmp::max(ant.x, &rod.length - ant.x);
            cmp::max(max, l)
        });

        Solution {
            min: min / SPEED_OF_ANT,
            max: max / SPEED_OF_ANT,
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
