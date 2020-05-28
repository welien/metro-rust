use itertools::Itertools;

extern crate ndarray;
use ndarray::prelude::*;

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

    let mut search_strategy = Exhaustive::new(stations);
    let best = search_strategy.search();
    println!("Best path is {} units long.", best.unwrap().length);
}
