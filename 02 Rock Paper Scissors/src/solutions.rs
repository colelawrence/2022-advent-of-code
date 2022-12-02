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
    /// Used in first interpretation
    fn parse_from_xyz(ch: &str) -> Result<Self, String> {
        match ch {
            "X" => Ok(IThrow::Rock),
            "Y" => Ok(IThrow::Paper),
            "Z" => Ok(IThrow::Scissors),
            other => Err(format!("Unrecognized throw {other:?}")),
        }
    }
}

enum WinnerIs {
    Them,
    Me,
    Tie,
}

impl WinnerIs {
    /// Used in second interpretation
    ///
    /// > The Elf finishes helping with the tent and sneaks
    /// back over to you. "Anyway, the second column says
    /// how the round needs to end: X means you need to
    /// lose, Y means you need to end the round in a draw,
    /// and Z means you need to win. Good luck!"
    fn parse_from_xyz(ch: &str) -> Result<Self, String> {
        match ch {
            "X" => Ok(WinnerIs::Them),
            "Y" => Ok(WinnerIs::Tie),
            "Z" => Ok(WinnerIs::Me),
            other => Err(format!("Unrecognized throw {other:?}")),
        }
    }
}

pub(crate) fn part_1() {
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
            .and_then(IThrow::parse_from_xyz)
            .expect("parsed");
        (they_throw, i_throw)
    });

    let mut my_total_score = 0i32;

    for (theirs, mine) in throws_it {
        my_total_score += calculate_my_score(theirs, mine);
    }

    println!("My total score would have been {my_total_score}");
}

pub(crate) fn part_2() {
    let throws_it = puzzle::INPUT.trim().lines().map(|line| {
        let mut it = line.split(' ').into_iter();
        let they_throw = it
            .next()
            .ok_or(String::from("Expected a letter for what they threw"))
            .and_then(TheyThrow::parse)
            .expect("parsed");
        let expect_winner = it
            .next()
            .ok_or(String::from(
                "Expected a letter for what outcome is expected",
            ))
            .and_then(WinnerIs::parse_from_xyz)
            .expect("parsed");
        (they_throw, expect_winner)
    });

    let mut my_total_score = 0i32;

    for (theirs, expected_winner) in throws_it {
        let mine = determine_what_i_need_to_throw_to_get_outcome(&theirs, &expected_winner);
        my_total_score += calculate_my_score(theirs, mine);
    }

    println!("In the second interpretation, my total score would have been {my_total_score}");
}

fn determine_what_i_need_to_throw_to_get_outcome(
    theirs: &TheyThrow,
    expected_winner: &WinnerIs,
) -> IThrow {
    match (theirs, expected_winner) {
        (TheyThrow::Rock, WinnerIs::Them) => IThrow::Scissors,
        (TheyThrow::Rock, WinnerIs::Me) => IThrow::Paper,
        (TheyThrow::Rock, WinnerIs::Tie) => IThrow::Rock,
        (TheyThrow::Paper, WinnerIs::Them) => IThrow::Rock,
        (TheyThrow::Paper, WinnerIs::Me) => IThrow::Scissors,
        (TheyThrow::Paper, WinnerIs::Tie) => IThrow::Paper,
        (TheyThrow::Scissors, WinnerIs::Them) => IThrow::Paper,
        (TheyThrow::Scissors, WinnerIs::Me) => IThrow::Rock,
        (TheyThrow::Scissors, WinnerIs::Tie) => IThrow::Scissors,
    }
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
