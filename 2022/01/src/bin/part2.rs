use std::convert::identity;
use std::str::FromStr;

fn main() {
    let (mut elves, last): (Vec<u32>, Option<u32>) = std::io::stdin()
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
        .fold((Vec::new(), None), |(mut elves, last), calories| {
            match (last, calories) {
                (Some(cur), Some(cal)) => (elves, Some(cur + cal)),
                (Some(cur), None) => ({ elves.push(cur); elves }, None),
                (None, Some(cal)) => (elves, Some(cal)),
                (None, None) => (elves, None),
            }
        });
    let top_three = {
        if let Some(last) = last {
            elves.push(last);
        }
        elves.sort();
        [elves.pop(), elves.pop(), elves.pop()]
    };
    println!("{}", top_three.into_iter().filter_map(identity).sum::<u32>());
}
