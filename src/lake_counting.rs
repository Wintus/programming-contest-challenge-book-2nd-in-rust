use {Solvable, UnsolvableError};

struct Problem {
    solved: bool,
    yard: Yard<bool>,
    footprints: Yard<bool>,
}

// &[&[T]] は lifetime 管理が面倒
type Yard<T> = Vec<Vec<T>>;

/// １次元の移動vector
/// 0 = 不動含む
fn mkdirs1d() -> [i8; 3] { [-1, 0, 1] }

/// 8方向のvectorを生成
/// (0, 0) = 不動含む
fn mkdirs2d() -> [(i8, i8); 9] {
    let dir1d = mkdirs1d();
    let dir2d: Vec<_> = iproduct![dir1d.to_vec(), dir1d.to_vec()].collect();
    let mut d9 = [(0, 0); 9];
    d9.copy_from_slice(&dir2d.as_slice());
    d9
}

// FIXME: nightly
//const DIRECTIONS: [(i8, i8); 8] = mkdirs2d();

impl Solvable for Problem {
    type I = Yard<bool>;
    type O = u32;

    fn input(&self) -> &Self::I { &self.yard }
    fn solve(&self) -> Result<u32, UnsolvableError> { Ok(3) }
}

impl Problem {
    /// Return: point (x, y) is lake or not.
    /// (x, y) = (row, column)
    fn dfs(&mut self, x: usize, y: usize) -> bool {
        // 現在地 (x, y) を訪問済みにする
        let mut visited = false;
        {
            if let Some(row) = self.footprints.get_mut(x) {
                let row: &mut Vec<_> = row; // type hinting
                if let Some(col) = row.get_mut(y) {
                    if *col { visited = true; }
                    *col = true;
                }
            }
        }
        if visited { return true }

        // 8方に訪問
        let _dirs = mkdirs2d();
        let dirs = _dirs
            .into_iter()
            .map(|&(x, y)| (x as isize, y as isize));
        // step all lakes
        for (dx, dy) in dirs {
            // get next (x, y)
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            let (nx, ny) = (nx as usize, ny as usize);

            let is_lake =
                self.yard.get(nx)
                         .map(|ref _row: &Vec<_>| {
                             *_row.get(ny)
                                  .unwrap_or(&false)
                         })
                         .unwrap_or(false);
            // &mut の又貸し
            if is_lake { self.dfs(nx, ny); }
        }
        // not visited
        false
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
