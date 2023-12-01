pub fn solve() {
    let input = include_str!("../resources/day01.txt");
    let answer_a = get_answer_a(input);

    println!("Answer for the first part is: {answer_a}")
}

fn get_answer_a(input: &str) -> usize {
    let lines = input.split_whitespace();
    let mut numbers = vec![];

    for line in lines {
        let digits: Vec<char> = line.
            chars()
            .filter(|c| c.is_ascii_digit())
            .collect();
        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();
        let number: usize = format!("{first_digit}{last_digit}").parse().unwrap();
        numbers.push(number)
    }
    numbers.iter().sum()
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
}