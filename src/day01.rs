use std::collections::HashSet;

pub fn run() {
    println!("On the first day of Christmas, AoC gave to me...");

    let frequency_deltas = parse_input(include_str!("../input/day01.in"));
    println!("{}", get_final_frequency(0, &frequency_deltas));
    println!("{}", get_first_repeated_frequency(0, &frequency_deltas));

    println!("The last and first-repeated frequencies!");
    println!();
}

fn parse_input(raw_input: &str) -> Vec<i32> {
    raw_input.lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn get_final_frequency(initial_frequency: i32, frequency_deltas: &Vec<i32>) -> i32 {
    return initial_frequency + frequency_deltas.iter().sum::<i32>();
}

pub fn get_first_repeated_frequency(initial_frequency: i32, frequency_deltas: &Vec<i32>) -> i32 {
    let mut current_frequency = initial_frequency;
    let mut seen_frequencies = HashSet::new();
    seen_frequencies.insert(initial_frequency);

    loop {
        for delta in frequency_deltas {
            current_frequency += delta;

            if seen_frequencies.contains(&current_frequency) {
                return current_frequency;
            }

            seen_frequencies.insert(current_frequency);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_valid_input() {
        assert_eq!(vec![123, -45, 67], parse_input("123\n-45\n67"));
    }

    #[test]
    #[should_panic]
    fn test_does_not_parse_invalid_input() {
        parse_input("123\n-45z\n12");
    }

    #[test]
    fn test_gets_final_frequency() {
        assert_eq!(6, get_final_frequency(7, &vec![1, -5, 3]))
    }

    #[test]
    fn test_gets_final_frequency_with_no_deltas() {
        assert_eq!(7, get_final_frequency(7, &vec![]))
    }

    #[test]
    fn test_gets_first_repeated_frequency_within_one_cycle() {
        assert_eq!(0, get_first_repeated_frequency(0, &vec![1, -1, 3, -1, 1]))
    }

    #[test]
    fn test_gets_first_repeated_frequency_after_one_cycle() {
        assert_eq!(14, get_first_repeated_frequency(0, &vec![7, 7, -2, -7, -4]))
    }
}
