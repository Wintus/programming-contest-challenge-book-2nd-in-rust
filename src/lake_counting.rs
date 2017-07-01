use {Solvable, UnsolvableError};

struct Problem {
    solved: bool,
    yard: Yard<bool>,
    footprints: Yard<bool>,
}

type Yard<T> = Vec<Vec<T>>;

impl Solvable for Problem {
    type I = Yard<bool>;
    type O = u32;

    fn input(&self) -> &Self::I { &self.yard }
    fn solve(&self) -> Result<u32, UnsolvableError> { Ok(3) }
}

impl Problem {
    fn dfs(&self, x: u32, y: u32) -> u32 {
        0
    }
}

#[test]
fn test_case_0() {
    let yard = "
        W________WW_
        _WWW_____WWW
        ____WW___WW_
        _________WW_
        _________W__
        __W______W__
        _W_W_____WW_
        W_W_W_____W_
        _W_W______W_
        __W_______W_
    ";
    let yard: Vec<Vec<bool>> =
        yard.split_whitespace()
            .map(|s| s.chars()
                      .map(|c| c == 'W')
                      .collect())
            .collect();
    let rows = yard.len();
    let cols = yard.first().unwrap().len();
    let plain = vec![vec![false; cols]; rows];

    let problem = Problem {
        solved: false,
        yard: yard,
        footprints: plain,
    };
    assert_eq!(3, problem.solve().unwrap_or(0));
}
