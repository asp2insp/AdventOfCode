use itertools::*;
use chrono::{NaiveDateTime,Timelike};
use std::collections::HashMap;

#[derive(Debug)]
enum What {
    Start(usize),
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
struct Event {
    dt: NaiveDateTime,
    what: What,
}

use self::What::*;

fn parse_line(s: &str) -> Event {
    Event {
        dt: NaiveDateTime::parse_from_str(&s[..18], "[%Y-%m-%d %H:%M]").unwrap(),
        what: match &s[19..24] {
            "falls" => FallAsleep,
            "wakes" => WakeUp,
            "Guard" => Start(s[25..].split_whitespace().next().unwrap()[1..].parse().unwrap()),
            _ => panic!("Unknown"),
        },
    }
}

pub fn part1(input: String) -> String {
    let mut events = input.lines().map(|l| {parse_line(l)}).collect::<Vec<_>>();
    events[..].sort_by_key(|e| e.dt);
    let mut sleep_times: HashMap<usize, usize> = HashMap::new();
    let mut current = 0;
    let mut sleep_start = NaiveDateTime::from_timestamp(0, 0);
    for e in &events {
        match e.what {
            Start(guard) => current = guard,
            FallAsleep => sleep_start = e.dt.clone(),
            WakeUp => *(sleep_times.entry(current).or_insert(0)) += (e.dt - sleep_start).num_minutes() as usize,
        }
    }
    let sleepiest_guard = sleep_times.iter().fold((0usize, 0usize), |max, n| if *n.1 > max.1 {(*n.0, *n.1)} else {max}).0;

    let mut sleep_minutes = [0usize; 60];
    for e in events {
        match e.what {
            Start(guard) => current = guard,
            FallAsleep => sleep_start = e.dt.clone(),
            WakeUp => {
                if current == sleepiest_guard {
                    let start = sleep_start.minute() as usize;
                    let end = e.dt.minute() as usize;
                    for i in start..end {
                        sleep_minutes[i] += 1;
                    }
                }
            },
        }
    }
    let sleepiest_minute = sleep_minutes.iter().enumerate().max_by_key(|a| a.1).unwrap().0;
    format!("{}", sleepiest_guard * sleepiest_minute)
}

pub fn part2(input: String) -> String {
    let mut events = input.lines().map(|l| {parse_line(l)}).collect::<Vec<_>>();
    events[..].sort_by_key(|e| e.dt);
    let mut current = 0;
    let mut sleep_start = NaiveDateTime::from_timestamp(0, 0);
    let mut sleep_minutes_by_guard: HashMap<usize, [usize; 60]> = HashMap::new();
    for e in events {
        match e.what {
            Start(guard) => current = guard,
            FallAsleep => sleep_start = e.dt.clone(),
            WakeUp => {
                let entry = sleep_minutes_by_guard.entry(current).or_insert([0usize; 60]);
                let start = sleep_start.minute() as usize;
                let end = e.dt.minute() as usize;
                for i in start..end {
                    entry[i] += 1;
                }
            },
        }
    }
    let (guard, minute) = sleep_minutes_by_guard.iter()
        .map(|(g, mins)| (g, mins.iter().enumerate().max_by_key(|a| a.1).unwrap()))
        .max_by_key(|(_, (_, n))| *n)
        .map(|(g, (m, _))| (g, m))
        .unwrap();
    format!("{}", guard * minute)
}