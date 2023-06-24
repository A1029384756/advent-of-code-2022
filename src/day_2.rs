#[derive(Clone, Copy)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Win = 6,
    Tie = 3,
    Loss = 0,
}

fn get_match_outcome(opponent: Choice, player: Choice) -> Outcome {
    match (opponent, player) {
        (Choice::Rock, Choice::Rock) => Outcome::Tie,
        (Choice::Rock, Choice::Paper) => Outcome::Win,
        (Choice::Rock, Choice::Scissors) => Outcome::Loss,
        (Choice::Paper, Choice::Rock) => Outcome::Loss,
        (Choice::Paper, Choice::Paper) => Outcome::Tie,
        (Choice::Paper, Choice::Scissors) => Outcome::Win,
        (Choice::Scissors, Choice::Rock) => Outcome::Win,
        (Choice::Scissors, Choice::Paper) => Outcome::Loss,
        (Choice::Scissors, Choice::Scissors) => Outcome::Tie,
    }
}

fn get_choice_from_outcome(opponent: Choice, outcome: Outcome) -> Choice {
    match (opponent, outcome) {
        (Choice::Rock, Outcome::Win) => Choice::Paper,
        (Choice::Rock, Outcome::Tie) => Choice::Rock,
        (Choice::Rock, Outcome::Loss) => Choice::Scissors,
        (Choice::Paper, Outcome::Win) => Choice::Scissors,
        (Choice::Paper, Outcome::Tie) => Choice::Paper,
        (Choice::Paper, Outcome::Loss) => Choice::Rock,
        (Choice::Scissors, Outcome::Win) => Choice::Rock,
        (Choice::Scissors, Outcome::Tie) => Choice::Scissors,
        (Choice::Scissors, Outcome::Loss) => Choice::Paper,
    }
}

fn get_match_score(opponent: Choice, player: Choice) -> u32 {
    player as u32 + get_match_outcome(opponent, player) as u32
}

fn letter_to_choice(letter: char) -> Choice {
    match letter {
        'A' | 'X' => Choice::Rock,
        'B' | 'Y' => Choice::Paper,
        'C' | 'Z' => Choice::Scissors,
        _ => panic!(),
    }
}

fn letter_to_outcome(letter: char) -> Outcome {
    match letter {
        'X' => Outcome::Loss,
        'Y' => Outcome::Tie,
        'Z' => Outcome::Win,
        _ => panic!(),
    }
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            get_match_score(
                letter_to_choice(line.as_bytes()[0] as char),
                letter_to_choice(line.as_bytes()[2] as char),
            )
        })
        .sum::<u32>()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            get_match_score(
                letter_to_choice(line.as_bytes()[0] as char),
                get_choice_from_outcome(
                    letter_to_choice(line.as_bytes()[0] as char),
                    letter_to_outcome(line.as_bytes()[2] as char),
                ),
            )
        })
        .sum::<u32>()
}

fn main() {
    let input = include_str!("test_files/day_2.txt");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
fn part_1_test() {
    let input = "A Y
B X
C Z";

    assert_eq!(part_1(input), 15);
}

#[test]
fn part_2_test() {
    let input = "A Y
B X
C Z";

    assert_eq!(part_2(input), 12);
}
