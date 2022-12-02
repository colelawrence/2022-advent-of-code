use super::puzzle;

enum TheyThrow {
    Rock,
    Paper,
    Scissors,
}

impl TheyThrow {
    fn parse(ch: &str) -> Result<Self, String> {
        match ch {
            "A" => Ok(TheyThrow::Rock),
            "B" => Ok(TheyThrow::Paper),
            "C" => Ok(TheyThrow::Scissors),
            other => Err(format!("Unrecognized throw {other:?}")),
        }
    }
}
enum IThrow {
    Rock,
    Paper,
    Scissors,
}

impl IThrow {
    fn parse(ch: &str) -> Result<Self, String> {
        match ch {
            "X" => Ok(IThrow::Rock),
            "Y" => Ok(IThrow::Paper),
            "Z" => Ok(IThrow::Scissors),
            other => Err(format!("Unrecognized throw {other:?}")),
        }
    }
}

pub(crate) fn part_2() {
    let throws_it = puzzle::INPUT.trim().lines().map(|line| {
        let mut it = line.split(' ').into_iter();
        let they_throw = it
            .next()
            .ok_or(String::from("Expected a letter for what they threw"))
            .and_then(TheyThrow::parse)
            .expect("parsed");
        let i_throw = it
            .next()
            .ok_or(String::from("Expected a letter for what I threw"))
            .and_then(IThrow::parse)
            .expect("parsed");
        (they_throw, i_throw)
    });

    let mut my_total_score = 0i32;

    for (theirs, mine) in throws_it {
        my_total_score += calculate_my_score(theirs, mine);
    }

    println!("My total score would have been {my_total_score}");
}

/// The score for a single round is the score for the
/// shape you selected (1 for Rock, 2 for Paper, and
/// 3 for Scissors) plus the score for the outcome of
/// the round (0 if you lost, 3 if the round was a draw,
/// and 6 if you won).
fn calculate_my_score(theirs: TheyThrow, mine: IThrow) -> i32 {
    const I_WIN: i32 = 6;
    const TIE: i32 = 3;
    const I_LOSE: i32 = 0;
    let score_for_my_throw = match mine {
        IThrow::Rock => 1,
        IThrow::Paper => 2,
        IThrow::Scissors => 3,
    };
    let score_for_outcome = match (mine, theirs) {
        (IThrow::Rock, TheyThrow::Scissors)
        | (IThrow::Paper, TheyThrow::Rock)
        | (IThrow::Scissors, TheyThrow::Paper) => I_WIN,
        (IThrow::Rock, TheyThrow::Rock)
        | (IThrow::Paper, TheyThrow::Paper)
        | (IThrow::Scissors, TheyThrow::Scissors) => TIE,
        (IThrow::Rock, TheyThrow::Paper)
        | (IThrow::Paper, TheyThrow::Scissors)
        | (IThrow::Scissors, TheyThrow::Rock) => I_LOSE,
    };

    score_for_my_throw + score_for_outcome
}
