use {Solvable, Result};
use std::cmp;

type Time = u32; // 時刻
type Task = (Time, Time);

struct Problem<'a> {
    tasks: &'a [Task]
}

impl<'a> Solvable for Problem<'a> {
    type I = &'a [Task];
    type O = u32; // number of tasks can be done

    fn input(&self) -> &Self::I {
        &self.tasks
    }

    fn solve(&self) -> Result<Self::O> {
        unimplemented!()
    }
}

mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let p = Problem {
            tasks: &[
                (1, 3),
                (2, 5),
                (4, 7),
                (6, 9),
                (8, 10),
            ]
        };
        assert_eq!(3, p.solve().unwrap_or(0));
    }
}
