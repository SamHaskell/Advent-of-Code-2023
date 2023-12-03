fn main() {
    let input = include_str!("../../input/input1.txt");

    let tick = std::time::Instant::now();
    let output = part1(input);
    println!("Done in {:?}", tick.elapsed());
    
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut result = 0;

    let lines = input.trim().split('\n').collect::<Vec<_>>();
    let line_count: u32 = lines.len() as u32;

    let mut adjacencies: Vec<bool> = Vec::new();
    for line in &lines {
        for sym in line.chars() {
            if !sym.is_digit(10) && sym != '.' {
                adjacencies.push(true);
            } else {
                adjacencies.push(false);
            }
        }
    }

    let line_size: u32 = adjacencies.len() as u32 / line_count;

    let mut is_adjacent = false;
    let mut is_num = false;

    let mut current_num = 0;

    for j in 0..line_count {
        for (i, sym) in lines[j as usize].chars().enumerate() {
            // Check if this is a numeric symbol
            is_num = sym.is_digit(10);

            // If numeric slap it on the back of the current accumulator
            if is_num {
                current_num *= 10;
                current_num += sym.to_digit(10).unwrap();
            }

            // If numeric, check for adjacency
            if is_num {
                for dy in 0..3 {
                    for dx in 0..3 {
                        if is_adjacent {
                            break;
                        }
                        
                        let row: i32 = (j as i32 + dy as i32 - 1) as i32;
                        let col: i32 = (i as i32 + dx as i32 - 1) as i32;

                        if row < 0 || row >= line_count.try_into().unwrap() {
                            continue;
                        }

                        if col < 0 || col >= line_size.try_into().unwrap() {
                            continue;
                        }

                        is_adjacent = adjacencies[(row * (line_size as i32) + col) as usize];
                    }
                    if is_adjacent {
                        break;
                    }
                }
            }

            // If we hit a non-numeric value, reset counters
            if !is_num {
                if is_adjacent {
                    result += current_num;
                }

                is_adjacent = false;
                current_num = 0;
                is_num = false;
            }
        }
    }

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
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
        assert_eq!(result, "4361".to_string());
    }
}