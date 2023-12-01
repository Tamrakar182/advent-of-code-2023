fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut result: Vec<char> = Vec::new();
    for char in input.chars() {
        if (char.is_ascii_alphabetic() && char.is_ascii_lowercase()) || char == ' ' {
            continue;
        }
        result.push(char);
    }

    let result: String = result.into_iter().collect();
    let mut answer: u32 = 0;
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    for char in result.chars() {
        if char == '\n' {
            if let (Some(first_char), Some(last_char)) = (first, last) {
                let two_digit_number = format!("{}{}", first_char, last_char).parse::<u32>().unwrap();
                answer += two_digit_number;
            }
            first = None;
            last = None;
        } else if char.is_numeric() {
            if first.is_none() {
                first = Some(char);
            }
            last = Some(char);
        }
    }

    if let (Some(first_char), Some(last_char)) = (first, last) {
        let two_digit_number = format!("{}{}", first_char, last_char).parse::<u32>().unwrap();
        answer += two_digit_number;
    }
    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // insert input here
        let result = part1("");
        // insert expected result here
        assert_eq!(result, "4".to_string());
    }
}
