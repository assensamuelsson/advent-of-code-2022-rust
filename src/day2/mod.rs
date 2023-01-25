use super::DayResults;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;
    
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("Not a valid move: {}", c)),
        }
    }
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(self))
            .unwrap()
    }

    fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| self.beats(*m))
            .unwrap()
    }

    fn drawing_move(self) -> Self {
        self
    }

    fn inherent_points(self) -> u128 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, theirs: Move) -> bool {
        matches!(
            (self, theirs),
            (Self::Rock, Self::Scissors) | (Self::Paper, Self::Rock) | (Self::Scissors, Self::Paper)
        )
    }

    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Lose
        } else {
            Outcome::Draw
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn inherent_points(self) -> u128 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }

    fn matching_move(self, theirs: Move) -> Move {
        match self {
            Outcome::Win => theirs.winning_move(),
            Outcome::Draw => theirs.drawing_move(),
            Outcome::Lose => theirs.losing_move(),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(color_eyre::eyre::eyre!("Not a valid outcome: {c:?}"))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("Expected '<theirs> <ours>EOF' - got {}", s));
        };

        Ok(Self {
            theirs: theirs.try_into()?,
            ours: ours.try_into()?,
        })
    }
}

impl Round {
    fn outcome(self) -> Outcome {
        self.ours.outcome(self.theirs)
    }
    
    fn our_score(self) -> u128 {
        self.outcome().inherent_points() + self.ours.inherent_points()
    }

    fn from_str_2(s: &str) -> Result<Self, color_eyre::Report> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("Expected '<theirs> <ours>EOF' - got {}", s));
        };

        let theirs: Move = theirs.try_into()?;
        let outcome: Outcome = ours.try_into()?;
        let ours = outcome.matching_move(theirs);

        Ok(Self {
            theirs,
            ours,
        })
    }
}

fn part1(contents: &str) -> u128 {
    contents
        .lines()
        .map(|line| line.parse::<Round>().unwrap())
        .map(|round| round.our_score())
        .sum()
}

fn part2(contents: &str) -> u128 {
    contents
        .lines()
        .map(|line| Round::from_str_2(line).unwrap())
        .map(|round| round.our_score())
        .sum()
}

pub fn run(contents: &String) -> DayResults {
    DayResults {
        part1: part1(contents),
        part2: Some(part2(contents)),
    }
}