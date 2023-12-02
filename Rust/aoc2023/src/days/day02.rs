use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../resources/day02.txt");
    let answer_a = get_answer_a(input);

    println!("Answer for day 2 part 1 is: {answer_a}");
}

#[derive(Eq, PartialEq, Hash)]
enum CubeColour {
    Red,
    Green,
    Blue,
}

struct Game {
    id: u8,
    rounds: Vec<Round>,
}

impl Game {
    fn new(game_id: u8, rounds: Vec<Round>) -> Game {
        Game {
            id: game_id,
            rounds,
        }
    }

    fn maximum_for(&self, colour: &CubeColour) -> usize {
        self.rounds
            .iter()
            .map(|r| r.total_for(colour))
            .max()
            .unwrap()
    }
}

struct Round {
    counts: HashMap<CubeColour, usize>,
}

impl Round {
    fn parse(round_string: &str) -> Round {
        let mut counts = HashMap::default();
        round_string.split(',').for_each(|c| {
            let (number_str, colour) = c.trim().split_once(" ").unwrap();
            let number: usize = number_str.parse().unwrap();

            match colour {
                "red" => counts.insert(CubeColour::Red, number),
                "green" => counts.insert(CubeColour::Green, number),
                "blue" => counts.insert(CubeColour::Blue, number),
                _ => panic!("Could not parse"),
            };
        });

        Round { counts }
    }

    fn total_for(&self, colour: &CubeColour) -> usize {
        self.counts.get(colour).unwrap_or(&0).to_owned()
    }
}

fn get_answer_a(input: &str) -> usize {
    let games: Vec<Game> = input
        .lines()
        .map(|line| {
            let (game_str, rounds_str) = line.split_once(":").expect("Should be a : on the line");
            let (_, game_id_str) = game_str
                .split_once(" ")
                .expect("Should be a space in the string");

            let rounds: Vec<Round> = rounds_str.split(";").map(|s| Round::parse(s)).collect();

            return Game::new(game_id_str.parse().expect("Should be a number"), rounds);
        })
        .collect();

    let maximum_red = 12;
    let maximum_green = 13;
    let maximum_blue = 14;

    games
        .iter()
        .filter(|game| {
            game.maximum_for(&CubeColour::Red) <= maximum_red
                && game.maximum_for(&CubeColour::Green) <= maximum_green
                && game.maximum_for(&CubeColour::Blue) <= maximum_blue
        })
        .map(|game| game.id as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let expected_answer = 8;
        let actual_answer = get_answer_a(input);

        assert_eq!(expected_answer, actual_answer)
    }
}
