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