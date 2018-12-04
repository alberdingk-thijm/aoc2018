use std::collections::HashMap;
/// Count letters to check if any appear
/// exactly twice or exactly thrice.
fn count_letters(s: &str) -> (bool, bool) {
    s.chars().fold(HashMap::new(), |mut letters, c| {
        {
            let count = letters.entry(c).or_insert(0);
            *count += 1;
        }
        letters
        // Some(match count {
        //     3 => (false, true),
        //     2 => (true, false),
        //     _ => (false, false),
        // })
    }).iter().map(|(_, &v)| (v == 2, v == 3))
    .fold((false, false), |(result2, result3), (twice, thrice)| {
        (result2 | twice, result3 | thrice)
    })
}

#[test]
fn counts() {
    assert_eq!((false, false), count_letters("abcdef"));
    assert_eq!((true, true), count_letters("bababc"));
    assert_eq!((true, false), count_letters("abbcde"));
    assert_eq!((false, true), count_letters("abcccd"));
    assert_eq!((true, false), count_letters("aabcdd"));
    assert_eq!((false, true), count_letters("ababab"));
}

fn similar_str<'a>(s: &'a str, lines: impl Iterator<Item = &'a str>) -> Option<&'a str> {
    lines.map(|l| {
        if l.chars().zip(s.chars()).filter(|(c, d)| c != d).count() == 1 {
            Some(l)
        } else {
            None
        }
    }).fold(None, |result, s| {
        result.or(s)
    })
}

fn main() {
    let input: &'static str = include_str!("../input");
    let lines = input.lines();
    // Part 1
    let (twice, thrice) = lines.clone().map(count_letters)
        .fold((0, 0), |(result2, result3), (twice, thrice)| {
            (result2 + twice as i32, result3 + thrice as i32)
        });
    println!("Checksum: {}", twice * thrice);
    // Part 2
    let (id1, id2) = lines.clone()
        .map(|s| similar_str(s, lines.clone()).map(|sim| (s, sim)))
        .fold(None, |result, s| {
            result.or(s)
        }).unwrap();
    println!("Match: {}", id1.chars().zip(id2.chars())
             .filter_map(|(c, d)| if c == d { Some(c) } else { None })
             .collect::<String>());
}
