use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../resources/day01.txt");
    let answer_a = get_answer_a(input);
    let answer_b = get_answer_b(input);

    println!("Answer for day 1 part 1 is: {answer_a}");
    println!("Answer for day 1 part 2 is: {answer_b}");
}

fn get_answer_a(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
            let first_digit = digits.first().expect("should be a valid digit");
            let last_digit = digits.last().expect("should be a valid digit");
            let number: usize = format!("{first_digit}{last_digit}")
                .parse()
                .expect("should be a valid number");
            number
        })
        .sum()
}

fn get_answer_b(input: &str) -> usize {
    let number_string_map = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3ree"),
        ("four", "f4ur"),
        ("five", "f5ve"),
        ("six", "s6x"),
        ("seven", "s7ven"),
        ("eight", "e8ght"),
        ("nine", "n9ne"),
    ]);

    input
        .lines()
        .map(|line| {
            let mut new_line = String::default();
            line.chars().for_each(|char| {
                new_line = format!("{new_line}{char}").to_string();
                number_string_map
                    .iter()
                    .for_each(|(k, v)| new_line = new_line.replace(k, v))
            });
            let digits: Vec<char> = new_line.chars().filter(|c| c.is_ascii_digit()).collect();
            let first_digit = digits.first().unwrap();
            let last_digit = digits.last().unwrap();

            format!("{first_digit}{last_digit}")
                .parse::<usize>()
                .expect("should be a valid number")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let expected_answer = 142;
        let actual_answer = get_answer_a(input);

        assert_eq!(expected_answer, actual_answer)
    }

    #[test]
    fn answer_a() {
        let input = include_str!("../resources/day01.txt");

        let expected_answer = 54940;
        let actual_answer = get_answer_a(input);
        assert_eq!(expected_answer, actual_answer)
    }

    #[test]
    fn example_b() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let expected_answer = 281;
        let actual_answer = get_answer_b(input);

        assert_eq!(expected_answer, actual_answer)
    }

    #[test]
    fn input_example_b() {
        let input = "lrqnqfncvvvrrpkfour92xsxfztwonehsb";

        let expected_answer = 41;
        let actual_answer = get_answer_b(input);

        assert_eq!(expected_answer, actual_answer)
    }

    #[test]
    fn edge_case_b() {
        let input = "iowu27dgdisi3eighthree";

        let expected_answer = 23;
        let actual_answer = get_answer_b(input);

        assert_eq!(expected_answer, actual_answer)
    }

    #[test]
    fn answer_b() {
        let input = include_str!("../resources/day01.txt");

        let expected_answer = 54208;
        let actual_answer = get_answer_b(input);

        assert_eq!(expected_answer, actual_answer)
    }
}
