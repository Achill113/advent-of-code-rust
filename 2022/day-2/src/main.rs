use std::fs;

#[derive(Copy, Clone, PartialEq)]
enum RoundResult {
    Draw,
    First,
    Second,
}

trait Points {
    fn get_points(&self) -> i32;
}

impl Points for RoundResult {
    fn get_points(&self) -> i32 {
        match *self {
            RoundResult::First => 0,
            RoundResult::Draw => 3,
            RoundResult::Second => 6
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for Shape {
    fn beats(&self) -> Self {
        match *self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper
        }
    }
}

impl Points for Shape {
    fn get_points(&self) -> i32 {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    let rows = input.split('\n');

    let rounds: Vec<(char, char)> = rows
        .filter(|row| !row.is_empty())
        .map(|row| {
            let plays: Vec<_> = row.split(' ').collect();

            let mut first_chars = plays[0].chars();

            let first_move = match first_chars.nth(0) {
                Some(it) => it,
                None => panic!("Could not parse first move")
            };

            let mut second_chars = plays[1].chars();

            let second_move = match second_chars.nth(0) {
                Some(it) => it,
                None => panic!("Could not parse second move")
            };

            return (first_move, second_move);
        }).collect();

    return rounds;
}

fn evaluate_shape(letter: &char) -> Shape {
    match letter {
        'A' | 'X' => Shape::Rock,
        'B' | 'Y' => Shape::Paper,
        'C' | 'Z' => Shape::Scissors,
        _ => panic!("Unexpected character: {}", letter)
    }
}

fn evaluate_round(round: &(char, char)) -> RoundResult {
    let first_shape = evaluate_shape(&round.0);
    let second_shape = evaluate_shape(&round.1);

    let (first_beats, second_beats) = (first_shape.beats(), second_shape.beats());

    match (first_beats, second_beats) {
        _ if first_beats == second_shape => RoundResult::First,
        _ if second_beats == first_shape => RoundResult::Second,
        _                            => RoundResult::Draw,
    }
}

fn evaluate_game(rounds: Vec<(char, char)>) -> i32 {
    let mut total_score: i32 = 0;

    for round in rounds {
        let result = evaluate_round(&round);
        let my_shape = evaluate_shape(&round.1);

        total_score += result.get_points() + my_shape.get_points();
    }

    return total_score;
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Error reading input.");

    let rounds = parse_input(&input);

    let total_score = evaluate_game(rounds);

    dbg!(total_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_should_return_slice_of_tuple() {
        let input = fs::read_to_string("./example-input.txt").expect("Error reading input.");

        let parsed = parse_input(&input);

        assert!(parsed.len() == 3);
    }

    #[test]
    fn rock_beats_scissors() {
        let first: char = 'A';
        let second: char = 'Z';
        assert!(evaluate_round(&(first, second)) == RoundResult::First);
    }

    #[test]
    fn paper_beats_rock() {
        let first: char = 'B';
        let second: char = 'X';
        assert!(evaluate_round(&(first, second)) == RoundResult::First);
    }

    #[test]
    fn scissors_beats_papper() {
        let first: char = 'C';
        let second: char = 'Y';
        assert!(evaluate_round(&(first, second)) == RoundResult::First);
    }

    #[test]
    fn evaluate_game_should_return_total_score() {
        let input = fs::read_to_string("./example-input.txt").expect("Error reading input.");

        let rounds = parse_input(&input);

        let total_score = evaluate_game(rounds);

        assert!(total_score == 15);
    }
}
