#![feature(iter_array_chunks)]

fn char_to_priority(c: char) -> u32 {
    assert!(c.is_ascii_alphabetic());
    match c {
        'A'..='Z' => c as u32 - 38,
        'a'..='z' => c as u32 - 96,
        _ => unreachable!()
    }
}

fn main() {
    let result: u32 = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .array_chunks()
        .map(|[first, second, third]| {
            first.chars()
                .filter(|&c| second.contains(c) && third.contains(c))
                .next()
                .expect("at least one common char")
        })
        .map(char_to_priority)
        .sum();
    println!("{}", result);
}