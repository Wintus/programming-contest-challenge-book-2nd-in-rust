use {Solvable, UnsolvableError};

struct Problem {
    maze: Maze,
}

type Maze = Vec<Vec<MazePart>>;

enum MazePart {
    Wall,
    Path,
    Start,
    Goal,
}

impl Solvable for Problem {
    type I = Maze;
    type O = u32;

    fn input(&self) -> &Self::I {
        unimplemented!()
    }

    fn solve(&self) -> Result<Self::O, UnsolvableError> {
        unimplemented!()
    }
}

#[test]
#[should_panic(expected = "not yet implemented")]
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
    let maze = vec![];
    let problem = Problem {
        maze: maze
    };
    assert_eq!(22, problem.solve().unwrap());
}
