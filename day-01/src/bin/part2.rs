fn main() {
    let input = include_str!("../../input/input1.txt");
    
    let tick = std::time::Instant::now();
    let output = part2(input);
    println!("Done in {:?}", tick.elapsed());
    
    dbg!(output);
}

const NUMBERS: &[&str] = &[
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

fn part2(input: &str) -> String {
    let lines = input.split('\n');
    let mut result: u32 = 0;
    for line in lines {
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        let length = line.len();

        for (i, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                first = char.to_digit(10).unwrap();
                break;
            }

            let mut found = false;
            for j in 0..NUMBERS.len() {
                if line[i..].starts_with(NUMBERS[j]) {
                    first = (j + 1) as u32;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        for (i, char) in line.chars().rev().enumerate() {
            if char.is_digit(10) {
                last = char.to_digit(10).unwrap();
                break;
            }
            
            let mut found = false;
            for j in 0..NUMBERS.len() {
                if line[length-i-1..].starts_with(NUMBERS[j]) {
                    last = (j + 1) as u32;
                    found = true;
                    break;
                }
            }
            if found {
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