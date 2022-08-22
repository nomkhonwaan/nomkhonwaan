use std::collections::HashMap;
use std::io::BufRead;
use std::{env, fs, io, path};

use chrono::{Duration, NaiveDateTime};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut records: Vec<Record> = vec![];
    let mut list_of_guards: HashMap<String, HashMap<String, Vec<u32>>> = HashMap::new();

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let record = Record::from_string(line);
                records.push(record);
            }
        }
    }

    records.sort_by(|a, b| a.date_time.cmp(&b.date_time));

    let mut duties: HashMap<String, Vec<u32>> = HashMap::new();
    let mut guard_id = records.get(0).unwrap().get_guard_id();
    let mut begin_sleep_time: Option<NaiveDateTime> = None;
    let mut midnight_hour = vec![0; 60];

    // skip the first record since it has been recorded at the init step
    for record in records.iter().skip(1) {
        if record.begin_record() {
            // store previous record
            if let Some(t) = begin_sleep_time {
                duties.insert(t.format("%Y-%m-%d").to_string(), midnight_hour);
                list_of_guards.insert(guard_id.clone(), duties);
            }

            // init a new record
            guard_id = record.get_guard_id();
            // try to get the previous duties data,
            // will create a new one if not exists
            duties = match list_of_guards.get(&guard_id) {
                Some(duties) => duties.clone(),
                _ => HashMap::new(),
            };
            begin_sleep_time = None;
            midnight_hour = vec![0; 60];
        }

        if record.falls_asleep() {
            begin_sleep_time = Some(record.date_time);
        }

        if record.wakes_up() {
            let end_sleep_time = record.date_time;
            if let Some(mut time) = begin_sleep_time {
                while time < end_sleep_time {
                    let i = time.format("%M").to_string().parse::<usize>().unwrap();
                    midnight_hour[i] = 1;
                    time = time + Duration::minutes(1);
                }
            }
        }
    }

    // store the latest record if not empty
    if let Some(t) = begin_sleep_time {
        duties.insert(t.format("%Y-%m-%d").to_string(), midnight_hour);
        list_of_guards.insert(guard_id.clone(), duties);
    }

    let accumulated_list_of_guards: Vec<(&String, Vec<u32>)> = list_of_guards
        .iter()
        // at this point, the list of duties will aggregated into a single vector
        //
        // from:
        // [
        //   "2022-08-18" => [0, 0, 1, 1, ...],
        //   "2022-08-19" => [1, 1, 1, 0, ...],
        // ]
        //
        // to:
        // [1, 1, 2, 1, ...]
        .map(|(guard_id, duties)| -> (&String, Vec<u32>) {
            (guard_id, accumulate_all_duties(&duties))
        })
        .collect();

    let (guard_id, accumulated, _): (&str, Vec<u32>, _) = accumulated_list_of_guards
        .clone()
        .into_iter()
        // find the most spending time on sleep guard
        .fold(("", vec![], 0u32), |result, (guard_id, accumulated)| {
            let total_sleep_time = accumulated.iter().sum::<u32>();
            if total_sleep_time > result.2 {
                return (guard_id, accumulated, total_sleep_time);
            }
            result
        });

    // find the most fall asleep time of the guard
    let max = accumulated.iter().max().unwrap();
    let i = accumulated.iter().position(|v| v == max).unwrap();

    println!("first part answer is: {}", guard_id.parse::<usize>().unwrap() * i);

    let (guard_id, accumulated, max): (&str, Vec<u32>, u32) = accumulated_list_of_guards
        .clone()
        .into_iter()
        // find the most frequently sleep at time same time guard
        .fold(("", vec![], 0u32), |result, (guard_id, accumulated)| {
            let &most_frequently_sleep = accumulated.iter().max().unwrap();
            if most_frequently_sleep > result.2 {
                return (guard_id, accumulated, most_frequently_sleep);
            }
            result
        });

    let i = accumulated.iter().position(|v| *v == max).unwrap();

    println!("second part answer is: {}", guard_id.parse::<usize>().unwrap() * i);
}

/// Contain each line of the record.
#[derive(Debug, Default)]
struct Record {
    date_time: NaiveDateTime,
    info: String,
}

impl Record {
    fn from_string(s: String) -> Self {
        let re = Regex::new(r"\[(.+)]\s(.+)$").unwrap();
        let captures = re.captures(&s).unwrap();
        let date_time = captures.get(1).map(|m| m.as_str().to_string()).unwrap();

        Record {
            date_time: NaiveDateTime::parse_from_str(&date_time, "%Y-%m-%d %H:%M").unwrap(),
            info: captures.get(2).map(|m| m.as_str().to_string()).unwrap(),
        }
    }

    fn begin_record(&self) -> bool {
        self.info.contains("begins")
    }

    fn falls_asleep(&self) -> bool {
        self.info.contains("falls asleep")
    }

    fn wakes_up(&self) -> bool {
        self.info.contains("wakes up")
    }

    fn get_guard_id(&self) -> String {
        let words: Vec<&str> = self.info.split(" ").collect();
        let id = words.get(1).unwrap();
        id[1..].to_string()
    }
}

fn accumulate_all_duties(duties: &HashMap<String, Vec<u32>>) -> Vec<u32> {
    let mut accumulated: Vec<u32> = vec![0; 60];
    for duty in duties.iter() {
        for (i, &v) in duty.1.iter().enumerate() {
            if v > 0 {
                accumulated[i] += 1;
            }
        }
    }
    accumulated
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
