use std::cmp;

struct Problem {
    premise: Premise
}

struct Premise {
    sides: Vec<u32>
}

impl Problem {
    fn solve(&self) -> u32 {
        let mut max_perimeter = 0;
        let ref sides = self.premise.sides;

        // TODO: use combinations
        let len = sides.len();
        for i in 0..len {
            for j in (i + 1)..len {
                for k in (j + 1)..len {
                    let sides: [u32; 3] = [
                        *sides.get(i).unwrap(),
                        *sides.get(j).unwrap(),
                        *sides.get(k).unwrap(),
                    ];

                    let perimeter = sides.iter().sum::<u32>();
                    let max_side = *sides.iter().max().unwrap();
                    let rest = perimeter - max_side;

                    if max_side < rest {
                        max_perimeter = cmp::max(max_perimeter, perimeter);
                    };
                }
            }
        }
        max_perimeter
    }
}

#[test]
fn test_case_0() {
    let premise = Premise { sides: vec![2, 3, 4, 5, 10] };
    let problem = Problem { premise: premise };
    assert_eq!(12, problem.solve());
}

#[test]
fn test_case_1() {
    let premise = Premise { sides: vec![4, 5, 10, 20] };
    let problem = Problem { premise: premise };
    assert_eq!(0, problem.solve()); // = never
}
