use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/input1.txt");

    let tick = std::time::Instant::now();
    let output = part2(input);
    println!("Done in {:?}", tick.elapsed());
    
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut result = 0;

    

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(
"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
    );
        assert_eq!(result, "467835".to_string());
    }
}