struct Claim {
    id: i16,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

use std::str::FromStr;
impl FromStr for Claim {
    type Err = ();
    /// Format: "#NNN @ X,Y: XxY"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(())
    }
}

fn main() {
    let input : &'static str = include_str!("../input");
    let lines = input.lines();
    // Part 1

    println!("Hello, world!");
}
