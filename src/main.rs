use rand::Rng;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::io;
use std::io::Write;

struct DiceRoll {
    n_times: u16,
    sides: u8,
}

impl DiceRoll {
    fn from(value: &str) -> DiceRoll {
        // First validate the input
        let re = Regex::new(r"^\d*d*\d*$").unwrap();
        if !re.is_match(value.trim()) {
            panic!("Invalid dice roll definition: {}", value);
        }

        let mut n_times: u16 = 1;
        let mut sides: u8 = 20;
        let parts: Vec<&str> = value
            .trim()
            .split('d')
            .filter(|&part| !part.is_empty())
            .collect();
        if parts.len() == 1 {
            // d10 - number of sides
            sides = parts[0].trim().parse().unwrap();
        } else if parts.len() == 2 {
            // 1d20
            n_times = parts[0].trim().parse().unwrap();
            sides = parts[1].trim().parse().unwrap();
        }
        DiceRoll {
            n_times: n_times,
            sides: sides,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => {
            // If there is no argument then run in interactive mode
            println!("{}", "*".repeat(29));
            println!("Started in interactive mode!");
            println!("{}", "*".repeat(29));
            println!("Supported commands:");
            println!("1. dice roll: 1d20, 4d6, d6, 8");
            println!("2. display the statistics: stats");
            println!("3. no input means the last dice roll value");

            let mut stats: HashMap<u8, Vec<u8>> = HashMap::new();
            let mut last = String::from("1d20");
            loop {
                println!("{}", "=".repeat(20));
                print!("> ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input line");

                let input = input.trim();
                if input.eq("stats") {
                    println!();
                    for (key, value) in &stats {
                        println!("d{} ({}x): {:?}", key, value.len(), value);
                    }
                } else {
                    let dice_roll = if input.len() == 0 {
                        DiceRoll::from(&last)
                    } else {
                        last = input.to_string();
                        DiceRoll::from(&input)
                    };
                    // Perform the rolls
                    let rolls = roll(&dice_roll);
                    // Collect stats
                    let dice_stats: &mut Vec<u8> =
                        stats.entry(dice_roll.sides).or_insert_with(Vec::new);
                    for elem in rolls {
                        dice_stats.push(elem);
                    }
                }
            }
        }
        Some(arg) => {
            roll(&DiceRoll::from(arg));
        }
    }
}

fn roll(dice_roll: &DiceRoll) -> Vec<u8> {
    let mut roll_values: Vec<u8> = Vec::new();

    for _ in 0..dice_roll.n_times {
        let roll_value = rand::thread_rng().gen_range(1, dice_roll.sides);
        roll_values.push(roll_value);
    }
    println!("\nRolled {}d{}:", dice_roll.n_times, dice_roll.sides);
    for (idx, elem) in roll_values.iter().enumerate() {
        println!("({})\t{}", idx + 1, elem);
    }

    // Print sum for multiple rolls
    if roll_values.len() > 1 {
        // Try to avoid possible overflow
        let sum: u16 = roll_values.iter().map(|x| *x as u16).sum();
        println!("{}", "-".repeat(14));
        println!("Sum:\t{}", sum);
    }
    roll_values
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_dice_roll_from() {
        let dr1 = DiceRoll::from("d1");
        assert_eq!(dr1.n_times, 1);
        assert_eq!(dr1.sides, 1);
        let dr2 = DiceRoll::from("2d6");
        assert_eq!(dr2.n_times, 2);
        assert_eq!(dr2.sides, 6);
        let dr3 = DiceRoll::from("10");
        assert_eq!(dr3.n_times, 1);
        assert_eq!(dr3.sides, 10);
    }

    #[test]
    fn test_roll() {
        let dr = DiceRoll::from("3d6");
        let values = roll(&dr);
        assert_eq!(values.len(), 3);
        let sum: u16 = values.iter().map(|x| *x as u16).sum();
        debug_assert!(sum <= 18);
        let dr = DiceRoll::from("4d8");
        let values = roll(&dr);
        assert_eq!(values.len(), 4);
        let sum: u16 = values.iter().map(|x| *x as u16).sum();
        debug_assert!(sum <= 32);
    }
}
