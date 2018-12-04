extern crate utils;
#[allow(unused_imports)]
use utils::{HashSet, HashMap, read_file, split_ws};

#[derive(Debug, Copy, Clone)]
enum Kind {
    Asleep,
    Wake,
    Begin(u32),
}

#[derive(Debug, Copy, Clone)]
struct Entry {
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    kind: Kind,
}

fn main() {
    let mut entries = Vec::new();
    for line in read_file("advent.txt") {
        let entry = process_line(&line);
        entries.push(entry);
    }
    entries.sort_by(|a, b| a.month.cmp(&b.month)
                    .then(a.day.cmp(&b.day))
                    .then(a.hour.cmp(&b.hour))
                    .then(a.minute.cmp(&b.minute))
                    );
    /*
    let id = find_most_asleep(&entries);
    let min = find_time_asleep(&entries, id);
    println!("{}", id);
    println!("{}", min);
    */
    let (id, min) = find_sleep_freq(&entries);
    println!("{}", id*min);
}

fn find_sleep_freq(entries: &Vec<Entry>) -> (u32, u32) {
    let mut sleep_time = HashMap::new();
    let mut current_id = 0u32;
    let mut current_sleep_start = 0;
    for e in entries {
        match e.kind {
            Kind::Asleep => {
                current_sleep_start = e.minute;
            },
            Kind::Wake => {
                let mut st = sleep_time.entry(current_id).or_insert([0; 60]);
                for i in current_sleep_start..e.minute {
                    st[i as usize] += 1;
                }
                current_sleep_start = 0;
            },
            Kind::Begin(id) => {
                current_id = id;
            }
        }
    }
    let mut most_sleep = 0;
    let mut result_id = 0;
    let mut result_idx = 0;
    for (k, v) in sleep_time.iter() {
        let mut best = 0;
        let mut idx = 0;
        for i in 0..60 {
            if v[i] > best {
                best = v[i];
                idx = i;
            }
        }
        if best > most_sleep {
            most_sleep = best;
            result_idx = idx;
            result_id = *k;
        }
    }
    (result_id, result_idx as u32)
}

fn find_time_asleep(entries: &Vec<Entry>, id: u32) -> u32 {
    let mut sleep_time = HashMap::new();
    let mut current_sleep_start = 0;
    let mut current_id = 0;

    for e in entries {
        match e.kind {
            Kind::Asleep => {
                current_sleep_start = e.minute;
            },
            Kind::Wake => {
                if current_id == id {
                    for i in current_sleep_start..e.minute {
                        let mut st = sleep_time.entry(i).or_insert(0);
                        *st += 1;
                    }
                }
                current_sleep_start = 0;
            },
            Kind::Begin(id) => {
                current_id = id;
            }
        }
    }
    let mut most_sleep = 0;
    let mut result = 0;
    for (k, v) in sleep_time.iter() {
        if *v > most_sleep {
            most_sleep = *v;
            result = *k;
        }
    }
    result
}

fn find_most_asleep(entries: &Vec<Entry>) -> u32 {
    let mut sleep_time = HashMap::new();
    let mut current_id = 0u32;
    let mut current_sleep_start = 0;
    for e in entries {
        match e.kind {
            Kind::Asleep => {
                current_sleep_start = e.minute;
            },
            Kind::Wake => {
                let current_sleep = e.minute - current_sleep_start;
                if current_id > 0 {
                    let mut st = sleep_time.entry(current_id).or_insert(0);
                    *st += current_sleep;
                }
                current_sleep_start = 0;
            },
            Kind::Begin(id) => {
                current_id = id;
            }
        }
    }
    let mut most_id = 0;
    let mut most_sleep = 0;
    for (k, v) in sleep_time.iter() {
        if *v > most_sleep {
            most_sleep = *v;
            most_id = *k;
        }
    }
    most_id
}

fn process_line(line: &str) -> Entry {
    let data = split_ws(line);

    let month = data[0][6..8].parse::<u32>().unwrap();
    let day = data[0][9..11].parse::<u32>().unwrap();
    let hour = data[1][0..2].parse::<u32>().unwrap();
    let minute = data[1][3..5].parse::<u32>().unwrap();

    let id = data[3][1..].parse::<u32>();
    let kind = match data[2] {
        "Guard" => Kind::Begin(id.unwrap()),
        "wakes" => Kind::Wake,
        "falls" => Kind::Asleep,
        _ => panic!("bad data: {}", data[2]),
    };

    Entry {
        month,
        day,
        hour,
        minute,
        kind,
    }
}
