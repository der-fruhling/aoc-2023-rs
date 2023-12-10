use std::{env, cmp::Ordering};

#[derive(Debug, Eq)]
struct RaceResult {
    time: u64,
    distance: u64,
    charging_time: Option<u64>,
}

impl PartialOrd<u64> for RaceResult {
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        Some(self.distance.cmp(&other))
    }
}

impl PartialEq<u64> for RaceResult {
    fn eq(&self, other: &u64) -> bool {
        self.distance == *other
    }
}

impl PartialEq for RaceResult {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

fn simulate_once(charge_time: u64, travel_time: u64) -> RaceResult {
    let speed = charge_time;
    let distance = speed * travel_time;
    
    RaceResult {
        time: charge_time + travel_time,
        distance,
        charging_time: Some(charge_time)
    }
}

fn simulate(total_time: u64) -> Vec<RaceResult> {
    (1..total_time).into_iter()
        .map(|charge_time| simulate_once(charge_time, total_time - charge_time))
        .collect()
}

fn find_winning_records(total_time: u64, record: u64) -> Vec<RaceResult> {
    simulate(total_time).into_iter()
        .filter(|v| v > &record)
        .collect()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_name = &args[1];

    let bytes = std::fs::read(file_name)
        .expect("Failed to read file!");

    let string = String::from_utf8(bytes).expect("Invalid file contents! Not UTF-8?");
    let mut lines = string.lines();
    let time_line = &lines.next().unwrap()[10..];
    let distance_line = &lines.next().unwrap()[10..];
    
    let times = time_line.split(' ')
        .filter(|v| !v.is_empty())
        .map(|v| u64::from_str_radix(v, 10).unwrap());
    let distances = distance_line.split(' ')
        .filter(|v| !v.is_empty())
        .map(|v| u64::from_str_radix(v, 10).unwrap());
    
    let records = times.zip(distances)
        .map(|(time, dist)| find_winning_records(time, dist));
    
    for record in records.clone() {
        println!("{:?}", record.len());
    }
    
    println!("Product: {:?}", records.map(|v| v.len()).product::<usize>())
}
