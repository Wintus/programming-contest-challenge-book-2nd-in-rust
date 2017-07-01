use {Solvable, UnsolvableError};

use std::cmp;

struct Problem {
    premise: Premise
}

struct Premise {
    rod: Rod,
    ants: Vec<Ant>,
}

#[derive(Debug, PartialEq)]
struct MinMaxTime {
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

impl Solvable for Problem {
    type I = Premise;
    type O = MinMaxTime;

    fn input(&self) -> &Premise { &self.premise }

    fn solve(&self) -> Result<MinMaxTime, UnsolvableError> {
        let input = self.input();
        let rod_len = input.rod.length;
        let ants = &input.ants;

        let min = ants.iter()
                      .map(|ref ant| cmp::min(ant.x, rod_len - ant.x))
                      .max()
                      .unwrap();
        let max = ants.iter()
                      .map(|ref ant| cmp::max(ant.x, rod_len - ant.x))
                      .max()
                      .unwrap();

        Ok(MinMaxTime {
            min: min / SPEED_OF_ANT,
            max: max / SPEED_OF_ANT,
        })
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
    let solution = MinMaxTime {
        // [-1, 1, 1]
        min: 4,
        // [1, 1, 1]
        max: 8,
    };
    let problem = Problem { premise: premise };
    assert_eq!(solution, problem.solve().unwrap());
}
