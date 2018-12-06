use chrono::{NaiveDateTime, Timelike, Utc};
use regex::Regex;
use std::collections::HashMap;
use std::cmp::Ordering;

pub fn execute(_lines: String) -> (u32, u32) {
    let mut lines = _lines.lines().map(|e| read_line(e).unwrap()).collect::<Vec<InputLine>>();
    lines.sort();

    let mut guards_sleeps = HashMap::new();
    let mut tmp = (0, Utc::now().naive_utc());

    for line in lines {
        match line.action {
            Action::TakeShift(id) => { tmp.0 = id; }
            Action::FallsAsleep => tmp.1 = line.timestamp,
            Action::WakeUp => {
                update_guards_sleeps(&mut guards_sleeps, tmp.0, &tmp.1, &line.timestamp)
            }
        }
    }


    (most_slept_guard_x_his_most_slept_minute(&guards_sleeps), most_slept_minute_x_her_most_slept_guard(&guards_sleeps))
}

fn getGuardId(action: &str) -> u32 {
    action.split("#").collect::<Vec<&str>>()
        .get(1).unwrap().split(" ").collect::<Vec<&str>>()
        .get(0).unwrap().parse().unwrap()
}

fn update_guards_sleeps(_guards_sleeps: &mut HashMap<u32, HashMap<u32, u32>>, _guard: u32, _start_sleep: &NaiveDateTime, _endSleep: &NaiveDateTime) {
    let currentGuardSleeps = _guards_sleeps.entry(_guard).or_insert(HashMap::new());
    for minute in _start_sleep.minute().._endSleep.minute() {
        *currentGuardSleeps.entry(minute).or_insert(0) += 1;
    }
}

fn most_slept_guard_x_his_most_slept_minute(_guards_sleeps: &HashMap<u32, HashMap<u32, u32>>) -> u32 {
    let sum = _guards_sleeps.iter().map(
        |e|
            (*e.0, e.1.iter().map(|i| *i.1).collect::<Vec<u32>>().iter().sum())
    ).collect::<Vec<(u32, u32)>>();
    let most_slept_guard = *sum.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap();
    let most_slept_minute = _guards_sleeps.get(&most_slept_guard.0).unwrap().iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap();

    most_slept_guard.0 * most_slept_minute.0
}

fn most_slept_minute_x_her_most_slept_guard(_guards_sleeps: &HashMap<u32, HashMap<u32, u32>>) -> u32 {
    let mut max: (u32, u32, u32) = (0, 0, 0);
    for line in _guards_sleeps.iter() {
        for lineM in line.1.iter() {
            if max.2 < *lineM.1 {
                max = (*line.0, *lineM.0, *lineM.1);
            }
        }
    }

    max.0  * max.1
}

fn read_line(_line: &str) -> Option<InputLine> {
    let pattern: Regex = Regex::new(r"(\[)(.*)(\])(.*)").unwrap();
    if !pattern.is_match(_line) {
        return None;
    }
    let groups = pattern.captures_iter(_line).next().unwrap();
    Some(
        InputLine {
            timestamp: NaiveDateTime::parse_from_str(&groups[2], "%Y-%m-%d %H:%M").unwrap(),
            action: {
                match (&groups[4]).trim() {
                    "wakes up" => Action::WakeUp,
                    "falls asleep" => Action::FallsAsleep,
                    _ => Action::TakeShift(getGuardId(&groups[4]))
                }
            },
        }
    )
}

#[derive(Eq)]
pub struct InputLine {
    timestamp: NaiveDateTime,
    action: Action,
}

impl Ord for InputLine {
    fn cmp(&self, other: &InputLine) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for InputLine {
    fn partial_cmp(&self, other: &InputLine) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for InputLine {
    fn eq(&self, other: &InputLine) -> bool {
        self.timestamp == other.timestamp && self.action == other.action
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Action {
    WakeUp,
    FallsAsleep,
    TakeShift(u32),
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_line_1() {
        let readLine = read_line("[1518-11-01 00:30] falls asleep").unwrap();
        assert_eq!(NaiveDateTime::parse_from_str("1518-11-01 00:30", "%Y-%m-%d %H:%M").unwrap(), readLine.timestamp);
        assert_eq!(Action::FallsAsleep, readLine.action);
    }

    #[test]
    fn getGuardId_test() {
        assert_eq!(3, getGuardId("Guard #3 begins shift"));
        assert_eq!(83745, getGuardId("Guard #83745 begins shift"));
        assert_eq!(238, getGuardId("Guard #238 begins shift"));
        assert_eq!(0, getGuardId("Guard #0 begins shift"));
    }
}
