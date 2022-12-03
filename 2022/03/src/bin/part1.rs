fn split_line_in_half(line: &str) -> (&str, &str) {
    assert_eq!(line.len() % 2, 0);
    line.split_at(line.len() / 2)
}

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
        .map(|line| {
            let (a, b) = split_line_in_half(&line);
            a.chars().filter(|c| b.contains(*c)).next().expect("at least one duplicate char")
        })
        .map(char_to_priority)
        .sum();
    println!("{}", result);
}