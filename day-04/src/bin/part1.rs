use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/input1.txt");

    let tick = std::time::Instant::now();
    let output = part1(input);
    println!("Done in {:?}", tick.elapsed());
    
    dbg!(output);
}

fn part1(input: &str) -> String {
    let result: i32 = input.trim()
        .lines()
        .fold(0, |acc, line| {
            let mut matches = 0;
            let mut is_result = false;
            let mut chosen_numbers: HashSet<i32> = HashSet::new();
            for token in line.split(' ') {
                if token == "|" {
                    is_result = true;
                    continue;
                }
                let val = token.parse::<i32>().unwrap_or(0);
                if !is_result && val != 0 {
                    chosen_numbers.insert(val);
                } else if chosen_numbers.contains(&val) {
                    chosen_numbers.remove(&val);
                    matches += 1;
                }
            }
            if matches > 0 { acc + i32::pow(2, matches - 1) } else { acc }
        });
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
    );
        assert_eq!(result, "13".to_string());
    }
}