extern crate regex;
use regex::Regex;
#[allow(dead_code)]
struct Claim {
    id: i16,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

#[allow(dead_code)]
impl Claim {
    /// Return the number of squares shared by the two claims.
    fn overlap(&self, other: &Self) -> i32 {
        let a = [
            0,
            (other.x2 - self.x1) * (other.y2 - self.y1),
            (other.x1 - self.x2) * (other.y2 - self.y1),
            (self.x1 - other.x2) * (self.y2 - other.y1),
            (self.x2 - other.x1) * (self.y2 - other.y1),
        ];
        *a.into_iter().max().unwrap()
    }
}

use std::str::FromStr;
impl FromStr for Matrix {
    type Err = ::std::num::ParseIntError;
    /// Format: "#NNN @ X,Y: XxY"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"#(?P<id>\d+) @ (?P<x1>\d+),(?P<y1>\d+): (?P<x2>\d+)x(?P<y2>\d+)")
            .unwrap();
        let captures = re.captures(s).unwrap();
        let _id = captures.name("id").unwrap().as_str().parse::<i16>()?;
        let x1 = captures.name("x1").unwrap().as_str().parse::<usize>()?;
        let y1 = captures.name("y1").unwrap().as_str().parse::<usize>()?;
        let x2 = captures.name("x2").unwrap().as_str().parse::<usize>()?;
        let y2 = captures.name("y2").unwrap().as_str().parse::<usize>()?;
        let mut m = Matrix::new();
        for x in x1..(x1 + x2) {
            for y in y1..(y1 + y2) {
                m.m[x][y] += 1;
            }
        }
        Ok(m)
    }
}

struct Matrix {
    m: [[i32; 1000]; 1000],
}

impl Matrix {
    fn new() -> Self {
        Matrix {
            m: [[0; 1000]; 1000],
        }
    }

    fn overlapping(&self) -> i32 {
        self.m.iter()
            .flat_map(|r| r.iter())
            .map(|&c| if c > 1 { 1 } else { 0 })
            .sum()
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;
    fn add(mut self, rhs: Matrix) -> Self::Output {
        for x in 0..1000 {
            for y in 0..1000 {
                self.m[x][y] += rhs.m[x][y];
            }
        }
        self
    }
}

fn main() {
    let input : &'static str = include_str!("../input");
    let lines = input.lines();
    // Part 1
    let mut total = Matrix::new();
    for line in lines {
        total = total + Matrix::from_str(line).unwrap()
    }
    println!("Squares: {}", total.overlapping());
}
