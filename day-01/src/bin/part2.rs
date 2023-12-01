fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let lines = input.split('\n');
    let mut result: u32 = 0;
    for line in lines {
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        for char in line.chars() {
            if char.is_ascii_digit() {
                first = char.to_digit(10).unwrap();
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_ascii_digit() {
                last = char.to_digit(10).unwrap();
                break;
            }
        }
        result += (first * 10) + last;
    }
    return result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"
    );
        assert_eq!(result, "281".to_string());
    }
}