use std::str::FromStr;

fn main() {
    let (max, last): (u32, Option<u32>) = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            if line.len() > 0 {
                Some(u32::from_str(&line).unwrap())
            }
            else {
                None
            }
        })
        .fold((0, None), |(max, last), calories| {
            match (last, calories) {
                (Some(cur), Some(cal)) => (max, Some(cur + cal)),
                (Some(cur), None) => (max.max(cur), None),
                (None, Some(cal)) => (max, Some(cal)),
                (None, None) => (max, None),
            }
        });
    println!("{}", last.unwrap_or_default().max(max));
}
