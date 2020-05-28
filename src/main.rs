use itertools::Itertools;

extern crate ndarray;
use ndarray::prelude::*;

use metro::Solution;
use metro::Station;

fn main() {
    let n_stations = 10;
    let symbols: Vec<i32> = vec![0, 1];
    let mut stations: Vec<Station> = Vec::new();

    for _ in 0..n_stations {
        stations.push(Station::randomize(&symbols, 0, 11));
    }

    // must use iter or it doesn't get borrowed (referenced)
    for station in stations.iter() {
        println!("{}", station)
    }

    let mut distances : Array2<f32> = Array::zeros((n_stations, n_stations));

    for i in 0..n_stations {
        for j in 0..n_stations {
            distances[[i,j]] = Station::distance(&stations[i], &stations[j]);
        }
    }

    // create vector of references to stations
    let mut path : Vec<usize> = vec![];

    for i in 0..n_stations {
        path.push(i);
        
    }

    //println!("{:?}", path);

    let it = path.into_iter().permutations(n_stations);
    //let x = &it.next().unwrap();
    //let y = &it.next().unwrap();

    //println!("{:?} {}",x, full_length(x, &distances));
    //println!("{:?} {}",y, full_length(y, &distances));
    //println!("{}", it[0])
    
    let mut permutations : Vec<Vec<usize>>= vec!();

    for permutation in it {
        println!("{}",full_length(&permutation, &distances));
        //println!("{:?}", permutation);
        //permutations.push(permutation);
    }

    //println!("{}", distances[(0,0)]);
    println!("{}", distances);

    // now to do some evaluations

}

fn full_length(path : &[usize], distances : &Array2<f32>) -> f32{
    let mut total = 0.;
    for i in 0..(path.len()-1) {
        let x = path[i] as usize;
        let y = path[i+1] as usize;
        total += distances[(x, y)];
    }
    total += distances[(0,(path.len()-1))];
    total
}
