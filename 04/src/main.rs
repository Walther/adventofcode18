use std::collections::HashMap;
extern crate regex;
use regex::Regex;

struct Guard {
    id: i32,
    sleeps: Vec<Sleep>,
}

impl Guard {
    pub fn new(id: i32) -> Guard {
        Guard {
            id,
            sleeps: Vec::new(),
        }
    }

    pub fn sleep_total(&self) -> i32 {
        self.sleeps.iter().fold(0, |acc, sleep| acc + sleep.total())
    }

    fn minute_sleep_counts(&self) -> HashMap<i32, i32> {
        // key: minute, value: total times that minute has been slept over the history
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for sleep in &self.sleeps {
            for minute in sleep.start..sleep.end {
                *counts.entry(minute).or_insert(0) += 1;
            }
        }
        counts
    }

    pub fn most_slept_minute(&self) -> i32 {
        let sleep_stats = &self.minute_sleep_counts();
        let mut minute = 0;
        let mut count = 0;
        for (stat_minute, stat_count) in sleep_stats.iter() {
            if count < *stat_count {
                minute = *stat_minute;
                count = *stat_count;
            }
        }
        minute
    }

    fn times_slept_at_minute(&self, minute: i32) -> i32 {
        let sleep_stats = &self.minute_sleep_counts();
        *sleep_stats.get(&minute).unwrap()
    }

    pub fn strategy2(&self) -> i32 {
        self.times_slept_at_minute(self.most_slept_minute())
    }
}

struct Sleep {
    // Tried to have Instant:s first but serializing would've been a pain
    // not even serde serializes times :/
    start: i32,
    end: i32,
}

impl Sleep {
    pub fn total(&self) -> i32 {
        &self.end - &self.start
    }
}

impl Sleep {
    pub fn new(start: i32, end: i32) -> Sleep {
        Sleep { start, end }
    }
}

enum EntryType {
    GuardStart,
    SleepStart,
    SleepEnd,
}

fn get_minute(entry: &str) -> i32 {
    // TODO: prettify
    let regex = Regex::new(r":(\d\d)").unwrap();
    // oh dear what horror. there must be a better way
    regex
        .captures(entry)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let mut guards: HashMap<i32, Guard> = HashMap::new();
    let mut calendar: Vec<&str> = INPUT.lines().collect();
    calendar.sort();

    // ugly globals due to the way the data is organized :(
    // ugly defaults to make compiler happier uninitialized stuff :(
    let mut guard_id: i32 = 0;
    let mut sleep_start: i32 = 0;

    // Go through the calendar, and collect stats for each guard
    for entry in calendar {
        // Figure out entry type
        if entry.contains("Guard") {
            let regex = Regex::new(r"#(\d*)").unwrap();

            // oh dear what horror. there must be a better way
            let id: i32 = regex
                .captures(entry)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            guard_id = id;
        } else if entry.contains("falls asleep") {
            sleep_start = get_minute(entry);
        } else if entry.contains("wakes up") {
            let sleep_end: i32 = get_minute(entry);
            let mut guard = guards.entry(guard_id).or_insert(Guard::new(guard_id));
            guard.sleeps.push(Sleep::new(sleep_start, sleep_end));
        }
    }

    // Part 1: Sleepiest guard by total time
    // wait, does this actually mean i have an array of sorted pointers :D cool!
    let mut guard_list: Vec<&Guard> = guards.values().collect();
    guard_list.sort_by(|a, b| a.sleep_total().cmp(&b.sleep_total()));
    // TODO: fix ugly mutable circumvent of immutable borrow issue
    let mut sleepiest = &Guard::new(0);
    sleepiest = guard_list.last().unwrap();

    println!("{}", sleepiest.id * sleepiest.most_slept_minute());

    // Part 2: Sleepiest guard on specific minute
    guard_list.sort_by(|a, b| a.strategy2().cmp(&b.strategy2()));
    let most_consistent = guard_list.last().unwrap();
    println!(
        "{}",
        most_consistent.id * most_consistent.most_slept_minute()
    )
}
