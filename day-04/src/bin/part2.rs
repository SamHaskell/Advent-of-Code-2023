use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/input1.txt");

    let tick = std::time::Instant::now();
    let output = part2(input);
    println!("Done in {:?}", tick.elapsed());
    
    dbg!(output);
}

fn part2(input: &str) -> String {
    let unique_cards = input.trim().lines().count();
    let mut card_counts: Vec<i32> = vec![1; unique_cards];
    let mut result: i32 = 0;

    for (i, line) in input.trim().lines().enumerate() {
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

        // Now add one to the next "matches" number of card counts
        for j in 1..=matches {
            if i + j >= unique_cards {
                break;
            } else {
                card_counts[i + j] += card_counts[i];
            }
        }
        result += card_counts[i];
    }
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(
"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
    );
        assert_eq!(result, "30".to_string());
    }
}