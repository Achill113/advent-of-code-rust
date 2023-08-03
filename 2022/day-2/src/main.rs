use std::fs;

#[derive(Copy, Clone, PartialEq)]
enum RoundResult {
    Draw,
    Lose,
    Win,
}

trait Character {
    fn get_necessary_result(&self) -> RoundResult;
}

impl Character for char {
    fn get_necessary_result(&self) -> RoundResult {
        match *self {
            'X' => RoundResult::Lose,
            'Y' => RoundResult::Draw,
            'Z' => RoundResult::Win,
            _ => panic!("Unexpected character")
        }
    }
}

trait Points {
    fn get_points(&self) -> i32;
}

impl Points for RoundResult {
    fn get_points(&self) -> i32 {
        match *self {
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

trait Char {
    fn get_char(&self) -> char;
}

impl Char for Shape {
    fn get_char(&self) -> char {
        match *self {
            Shape::Rock => 'A',
            Shape::Paper => 'B',
            Shape::Scissors => 'C'
        }
    }
}

trait Beats {
    fn beats(&self) -> Self;
    fn beats_me(&self) -> Self;
}

impl Beats for Shape {
    fn beats(&self) -> Self {
        match *self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper
        }
    }

    fn beats_me(&self) -> Self {
        match *self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock
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

            let their_move = match first_chars.nth(0) {
                Some(it) => it,
                None => panic!("Could not parse their move")
            };

            let mut my_chars = plays[1].chars();

            let my_move = match my_chars.nth(0) {
                Some(it) => it,
                None => panic!("Could not parse my move")
            };

            return (their_move, my_move);
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
    let their_shape = evaluate_shape(&round.0);
    let my_shape = evaluate_shape(&round.1);

    let (their_beats, my_beats) = (their_shape.beats(), my_shape.beats());

    match (their_beats, my_beats) {
        _ if their_beats == my_shape => RoundResult::Lose,
        _ if my_beats == their_shape => RoundResult::Win,
        _                            => RoundResult::Draw,
    }
}

fn get_necessary_hand(their_hand: &char, result: &RoundResult) -> Shape {
    let their_shape = evaluate_shape(&their_hand);

    let their_beats = their_shape.beats();

    match *result {
        RoundResult::Lose => their_beats,
        RoundResult::Draw => their_shape,
        RoundResult::Win => their_shape.beats_me()
    }
}

fn evaluate_game_part_one(rounds: &Vec<(char, char)>) -> i32 {
    let mut total_score: i32 = 0;

    for round in rounds {
        let result = evaluate_round(&round);
        let my_shape = evaluate_shape(&round.1);

        total_score += result.get_points() + my_shape.get_points();
    }

    return total_score;
}

fn evaluate_game_part_two(rounds: &Vec<(char, char)>) -> i32 {
    let mut total_score: i32 = 0;

    for round in rounds {
        let necessary_result = round.1.get_necessary_result();
        let necessary_shape = get_necessary_hand(&round.0, &necessary_result);

        let result = evaluate_round(&(round.0, necessary_shape.get_char()));

        total_score += result.get_points() + necessary_shape.get_points();
    }

    return total_score;
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Error reading input.");

    let rounds = parse_input(&input);

    let total_score_part_one = evaluate_game_part_one(&rounds);

    dbg!(total_score_part_one);

    let total_score_part_two = evaluate_game_part_two(&rounds);

    dbg!(total_score_part_two);
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
        let other: char = 'A';
        let mine: char = 'Z';
        assert!(evaluate_round(&(other, mine)) == RoundResult::Lose);
    }

    #[test]
    fn paper_beats_rock() {
        let other: char = 'B';
        let mine: char = 'X';
        assert!(evaluate_round(&(other, mine)) == RoundResult::Lose);
    }

    #[test]
    fn scissors_beats_papper() {
        let other: char = 'C';
        let mine: char = 'Y';
        assert!(evaluate_round(&(other, mine)) == RoundResult::Lose);
    }

    #[test]
    fn evaluate_game_part_one_should_return_total_score() {
        let input = fs::read_to_string("./example-input.txt").expect("Error reading input.");

        let rounds = parse_input(&input);

        let total_score = evaluate_game_part_one(rounds);

        assert!(total_score == 15);
    }

    #[test]
    fn evaluate_game_part_two_should_return_total_score() {
        let input = fs::read_to_string("./example-input.txt").expect("Error reading input.");

        let rounds = parse_input(&input);

        let total_score = evaluate_game_part_two(rounds);

        assert!(total_score == 12);
    }
}
