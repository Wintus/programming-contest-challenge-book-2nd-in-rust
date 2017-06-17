struct Problem {
    premise: Premise
}

struct Premise {
    rod: Rod,
    ants: [Ant],
}

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
    fn solve(&self) -> Solution {}
}
