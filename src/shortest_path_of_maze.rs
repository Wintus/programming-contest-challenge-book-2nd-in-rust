use {Solvable, Result};
use std::collections;
use std::u32;

const DIRS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const MAX: u32 = u32::MAX >> 2;

type Grid2D<T> = Vec<Vec<T>>;
type Maze = Grid2D<MazePart>;
type Point = (usize, usize);
type Queue<T> = collections::VecDeque<T>;

struct Problem {
    maze: Maze,
    start: Point,
    goal: Point,
}

#[derive(PartialEq)]
enum MazePart {
    Wall,
    Path,
    Start,
    Goal,
}

impl Solvable for Problem {
    type I = Maze;
    type O = u32;

    fn input(&self) -> &Self::I { &self.maze }

    fn solve(&self) -> Result<Self::O> {
        let mut mem = self.mk_mem();
        let dist = self.bfs(&mut mem, self.start, self.goal);
        Ok(dist)
    }
}

impl Problem {
    fn mk_mem(&self) -> Grid2D<u32> {
        let ref maze = self.maze;
        let rows = maze.len();
        let cols = maze.first().unwrap().len();
        vec![vec![MAX; cols]; rows]
    }

    /// 幅優先探索。
    /// (x, y) = (row, column).
    /// `mem` で状態を引き回す。
    ///
    /// Return: distance from start to goal.
    fn bfs(&self, mem: &mut Grid2D<u32>, start: Point, goal: Point) -> u32 {
        let mut queue: Queue<Point> = Queue::new();
        queue.push_back(start);
        // set distance of start point to 0
        {
            let mut start_dist = get_mut_at(mem, start);
            *start_dist = 0;
        }

        // traverse all path
        while let Some(point) = queue.pop_front() {
            if point == goal { break; } // reached goal

            let current_dist = *get_at(mem, point);

            // explorer 4 directions
            let dirs: &[_] = &DIRS;
            for dir in dirs {
                // no whole match in pattern match
                // let p @ (x, y)
                let (x, y) = (
                    point.0 as isize + dir.0 as isize,
                    point.1 as isize + dir.1 as isize,
                );
                let (x, y) = (x as usize, y as usize);
                if let Some(row) = mem.get_mut(x) {
                    if let Some(col) = row.get_mut(y) {
                        let mut new_dist = col;
                        let visited = *new_dist != MAX;

                        let next_part = get_at(&self.maze, (x, y));
                        let movable = *next_part != MazePart::Wall;

                        // 1 step apart from here
                        if movable && !visited {
                            queue.push_back((x, y));
                            *new_dist = current_dist + 1;
                        }
                    }
                }
            }
        }

        // distance to goal
        *get_at(mem, goal)
    }
}

// FIXME: impl Grid2D is disallowed. #E0116

fn get_at<T>(grid: &Grid2D<T>, point: Point) -> &T {
    grid.get(point.0).unwrap()
        .get(point.1).unwrap()
}

fn get_mut_at<T>(grid: &mut Grid2D<T>, point: Point) -> &mut T {
    grid.get_mut(point.0).unwrap()
        .get_mut(point.1).unwrap()
}

fn mk_maze(maze_str: &str) -> Grid2D<MazePart> {
    maze_str
        .split_whitespace()
        .map(|s: &str| {
            s.chars()
             .map(|c| match c {
                 '#' => MazePart::Wall,
                 '.' => MazePart::Path,
                 'S' => MazePart::Start,
                 'G' => MazePart::Goal,
                 _ => MazePart::Path,
             })
             .collect()
        })
        .collect()
}

#[test]
fn test_case_0() {
    let maze = "\
        #S######.#
        ......#..#
        .#.##.##.#
        .#........
        ##.##.####
        ....#....#
        .#######.#
        ....#.....
        .####.###.
        ....#...G#";
    let maze: Maze = mk_maze(maze);
    let problem = Problem {
        maze: maze,
        start: (0, 1),
        goal: (9, 8),
    };
    assert_eq!(22, problem.solve().unwrap());
}

#[test]
fn test_case_1() {
    let maze = "\
        S#######.#
        ......#..#
        .#.##.##.#
        .#........
        ##.##.####
        ....#....#
        .#######.#
        ....#.....
        .####.###.
        ...G#....#";
    let maze: Maze = mk_maze(maze);
    let problem = Problem {
        maze: maze,
        start: (0, 0),
        goal: (9, 3),
    };
    assert_eq!(16, problem.solve().unwrap());
}
