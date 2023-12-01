fn main() {
    let input = include_str!("../../input/input1.txt");

    let tick = std::time::Instant::now();
    let output = part1(input);
    println!("Done in {:?}", tick.elapsed());
    
    dbg!(output);
}

fn part1(input: &str) -> String {
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
        let result = part1(
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
    );
        assert_eq!(result, "142".to_string());
    }
}