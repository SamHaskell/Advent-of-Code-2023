use core::cmp::max;

fn main() {
    let input = include_str!("../../input/input1.txt");

    let tick = std::time::Instant::now();
    let output = part2(input);
    println!("Done in {:?}", tick.elapsed());
    
    dbg!(output);
}

fn part2(input: &str) -> String {
    let lines = input.trim().split('\n');
    let mut result = 0;

    for line in lines {
        let (mut r, mut g, mut b) = (0, 0, 0);
        let parts = line.split([':', ';', ',']).collect::<Vec<_>>();

        for i in 1..parts.len() {
            let mut iter = parts[i].trim().split(' ');

            let count = iter.next().unwrap().parse::<i32>().unwrap();
            let color = iter.next().unwrap();
            match color {
                "red" => r = max(r, count),
                "green" => g = max(g, count),
                "blue" => b = max(b, count),
                _ => {}
            }
        }

        result += r * g * b;
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
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
    );
        assert_eq!(result, "2286".to_string());
    }
}