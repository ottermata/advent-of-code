#[derive(Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_char(s: char) -> Result<Self, String> {
        match s {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            invalid => Err(format!("invalid character: {}", invalid))
        }
    }

    fn from_outcome(outcome: char, other: &Self) -> Result<Self, String> {
        match (outcome, other) {
            ('X', Self::Rock) => Ok(Self::Scissors),
            ('X', Self::Paper) => Ok(Self::Rock),
            ('X', Self::Scissors) => Ok(Self::Paper),
            ('Y', _) => Ok(*other),
            ('Z', Self::Rock) => Ok(Self::Paper),
            ('Z', Self::Paper) => Ok(Self::Scissors),
            ('Z', Self::Scissors) => Ok(Self::Rock),
            (invalid, _) => Err(format!("invalid character: {}", invalid)),
        }
    }
    
    fn from_line(line: &str) -> Result<[Self; 2], String> {
        if line.len() != 3 {
            return Err(format!("expected 3 characters per line, got: '{}'", line))
        }
        let mut chars = line.chars();
        let opponent = Self::from_char(chars.next().unwrap())?;
        Ok([
            opponent,
            Self::from_outcome(chars.nth(1).unwrap(), &opponent)?,
        ])
    }

    fn hand_score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn round_score(&self, other: &Self) -> u32 {
        match (self, other) {
            (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => 6,
            (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => 0,
            (_, _) => 3,
        }
    }

    fn get_score_against(&self, other: &Self) -> u32 {
        self.hand_score() + self.round_score(other)
    }
}


fn main() {
    let score: u32 = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| RPS::from_line(&line).unwrap())
        .map(|[opponent, you]| you.get_score_against(&opponent))
        .sum();
    println!("{}", score);
}