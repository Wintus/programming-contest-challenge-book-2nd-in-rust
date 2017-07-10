use {Solvable, Result};

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

    /// 貪欲法:
    /// 競合しない終了時間が最近のものを選ぶ。
    fn solve(&self) -> Result<Self::O> {
        let tasks: &mut Vec<Task> = &mut vec![];
        tasks.extend_from_slice(self.tasks);
        tasks.sort_by(|&(s0, e0), &(s1, e1)| (e0, s0).cmp(&(e1, s1)));
        // state = (end time of last selected task, number of task can be done so far)
        let (_, n) = tasks
            .iter()
            .fold((0, 0), |(last_end_time, n_tasks), task| {
                if task.0 > last_end_time {
                    (task.1, n_tasks + 1) // one task done
                } else {
                    (last_end_time, n_tasks)
                }
            });
        Ok(n)
    }
}


#[test]
fn test_case_0() {
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

#[test]
fn test_case_1() {
    let p = Problem {
        tasks: &[
            (1, 3),
            (1, 5),
            (2, 8),
            (3, 13),
            (5, 21),
        ]
    };
    assert_eq!(2, p.solve().unwrap_or(0));
}
