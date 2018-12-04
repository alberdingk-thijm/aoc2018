use std::collections::HashSet;
fn main() {
    let input : &'static str = include_str!("../input");
    let nums = input.lines().map(|i| i.parse::<i32>().unwrap());
    // Part One
    let a : i32 = nums.clone().sum();
    println!("{}", a);
    // Part Two
    // get the sum at each point in the chain
    let sums = nums.clone().cycle().scan(0, |sum, n| {
        *sum = *sum + n;
        Some(*sum)
    });
    let mut seen = HashSet::new();
    for sum in sums {
        if seen.contains(&sum) {
            println!("{}", sum);
            break;
        } else {
            seen.insert(sum);
        }
    }
}
