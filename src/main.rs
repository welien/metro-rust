use std::time::{Duration, Instant};

use metro::Station;
use metro::Exhaustive;

fn main() {
    let n_stations = 10;
    let symbols: Vec<i32> = vec![0, 1];
    let mut stations: Vec<Station> = Vec::new();

    // create random stations
    for _ in 0..n_stations {
        stations.push(Station::randomize(&symbols, 0, 11));
    }

    // must use iter or it doesn't get borrowed (referenced)
    for station in stations.iter() {
        println!("{}", station)
    }

    let time_start = Instant::now();
    let mut search_strategy = Exhaustive::new(stations);
    let best = search_strategy.search();
    let time_end = time_start.elapsed().as_millis();
    println!("Best path is {} units long. Found in {} mseconds", best.unwrap().length, time_end);
}
