extern crate regex;
use regex::Regex;
use std::collections::HashMap;

enum Entry {
    Id(i16),
    X,
}

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
impl FromStr for Claim {
    type Err = ::std::num::ParseIntError;
    /// Format: "#NNN @ X,Y: XxY"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"#(?P<id>\d+) @ (?P<x1>\d+),(?P<y1>\d+): (?P<x2>\d+)x(?P<y2>\d+)")
            .unwrap();
        let captures = re.captures(s).unwrap();
        let id = captures.name("id").unwrap().as_str().parse::<i16>()?;
        let x1 = captures.name("x1").unwrap().as_str().parse::<i32>()?;
        let y1 = captures.name("y1").unwrap().as_str().parse::<i32>()?;
        let x2 = captures.name("x2").unwrap().as_str().parse::<i32>()?;
        let y2 = captures.name("y2").unwrap().as_str().parse::<i32>()?;
        let claim = Claim {
            id: id,
            x1: x1,
            y1: y1,
            x2: x1 + x2,
            y2: y1 + y2,
        };
        Ok(claim)
    }
}

impl<'a> From<&'a Claim> for Matrix {
    /// Format: "#NNN @ X,Y: XxY"
    fn from(c: &'a Claim) -> Self {
        let mut m = Matrix::new();
        for x in c.x1..c.x2 {
            for y in c.y1..c.y2 {
                m.m.insert((x, y), Entry::Id(c.id));
            }
        }
        m
    }
}

struct Matrix {
    m: HashMap<(i32, i32), Entry>
}

impl Matrix {
    fn new() -> Self {
        Matrix {
            m: HashMap::new(),
        }
    }

    fn overlapping(&self) -> i32 {
        self.m.iter()
            .map(|(_, v)| match v {
                Entry::Id(_) => 0,
                Entry::X => 1,
            })
            .sum()
    }

    fn intact(&self, claims: &[(i16, usize)]) -> i16 {
        let mut map = HashMap::new();
        let singles = self.m.iter()
            .filter_map(|(k, v)| match v {
                Entry::Id(id) => Some((id, k)),
                Entry::X => None,
            });
        for (id, coords) in singles {
            let entry = map.entry(id).or_insert(vec![]);
            entry.push(coords);
        }
        for (id, num) in claims {
            if let Some(intact_coords) = map.get(&id) {
                if intact_coords.len() == *num {
                    return *id;
                }
            }
        }
        0
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;
    fn add(mut self, rhs: Matrix) -> Self::Output {
        for (k, v) in rhs.m.into_iter() {
            if self.m.contains_key(&k) {
                if let Some(x) = self.m.get_mut(&k) {
                    *x = Entry::X;
                }
            } else {
                self.m.insert(k, v);
            }
        }
        self
    }
}

fn main() {
    let input : &'static str = include_str!("../input");
    let lines = input.lines();
    let claims = lines
        .map(|c| Claim::from_str(c).unwrap());
    // Part 1
    let mut total = Matrix::new();
    for claim in claims.clone() {
        total = total + Matrix::from(&claim)
    }
    println!("Squares: {}", total.overlapping());
    // Part 2
    let sizes = claims.map(|c| (c.id, ((c.x2 - c.x1) * (c.y2 - c.y1)) as usize))
        .collect::<Vec<_>>();
    println!("Intact: {}", total.intact(sizes.as_slice()));
}
