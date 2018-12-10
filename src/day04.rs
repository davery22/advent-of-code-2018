use regex::Regex;
use chrono::NaiveDateTime;
use chrono::Timelike;
use std::collections::HashMap;

pub fn run() {
    println!("On the fourth day of Christmas, AoC gave to me...");

    let sleeps_per_minute_by_guard_id = get_sleeps_per_minute_by_guard_id(&get_ordered_night_shift_logs(include_str!("../input/day04.in")));
    let max_by_most_sleep = get_sleepiest_guard_at_sleepiest_minute(&sleeps_per_minute_by_guard_id, |sleeps| sleeps.iter().sum::<usize>());
    let max_by_sleepiest_minute = get_sleepiest_guard_at_sleepiest_minute(&sleeps_per_minute_by_guard_id, |sleeps| *sleeps.iter().max().unwrap());
    println!("{:?}", max_by_most_sleep.0 * max_by_most_sleep.1);
    println!("{:?}", max_by_sleepiest_minute.0 * max_by_sleepiest_minute.1);

    println!("The product of the sleepiest guards on night shift and their favorite minute to sleep!");
    println!();
}

fn get_ordered_night_shift_logs(nightly_records: &str) -> Vec<(NaiveDateTime, &str)> {
    let mut night_shift_logs = Vec::new();

    // Capture timestamp and message in logs, like:
    // [1518-08-02 00:02] Guard #1223 begins shift
    // -> (1518-08-02T00:02, "Guard #1223 begins shift")
    let record_decoder = Regex::new(r"\[([^\]]*)] ([^\r\n]*)").unwrap();

    for record in record_decoder.captures_iter(nightly_records) {
        let timestamp = NaiveDateTime::parse_from_str(&record[1], "%Y-%m-%d %R").unwrap();
        let message = record.get(2).unwrap().as_str();

        night_shift_logs.push((timestamp, message));
    }

    night_shift_logs.sort_unstable_by_key(|record| record.0);
    night_shift_logs
}

fn get_sleeps_per_minute_by_guard_id(ordered_night_shift_logs: &Vec<(NaiveDateTime, &str)>) -> HashMap<usize, Vec<usize>> {
    let guard_id_decoder = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    let mut sleeps_per_minute_by_guard_id = HashMap::new();

    // These initial values will not be used if the input is valid
    // Yes, like other places, this too should have error handling
    let mut guard_id = 0;
    let mut sleep_minute = 0;

    for log in ordered_night_shift_logs {
        if guard_id_decoder.is_match(log.1) {
            guard_id = guard_id_decoder.captures(log.1).unwrap()[1].parse().unwrap();
        } else if log.1 == "falls asleep" {
            sleep_minute = log.0.minute() as usize;
        } else if log.1 == "wakes up" {
            let wake_minute = log.0.minute() as usize;
            let mut sleeps_per_minute = sleeps_per_minute_by_guard_id.entry(guard_id).or_insert(vec![0_usize; 60]);

            for minute in sleep_minute .. wake_minute {
                sleeps_per_minute[minute] += 1;
            }
        }
    }

    sleeps_per_minute_by_guard_id
}

pub fn get_sleepiest_guard_at_sleepiest_minute<F>(
    sleeps_per_minute_by_guard_id: &HashMap<usize, Vec<usize>>, max_finder: F) -> (usize, usize)
    where F: Fn(&Vec<usize>) -> (usize) {
    let sleepiest_guards_id = sleeps_per_minute_by_guard_id.iter()
        .map(|(guard_id, sleeps)| (guard_id, max_finder(sleeps)))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap().0;

    let sleepiest_minute = sleeps_per_minute_by_guard_id.get(&sleepiest_guards_id).unwrap().iter().enumerate()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap().0;

    (*sleepiest_guards_id, sleepiest_minute)
}

#[cfg(test)]
mod tests {
    use super::*;
}