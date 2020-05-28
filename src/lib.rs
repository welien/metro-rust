extern crate ndarray;
use ndarray::prelude::*;
use std::fmt;
use rand::Rng;
use itertools::Itertools;
use std::sync::Arc;

use std::thread;
use crossbeam;
use crossbeam::crossbeam_channel::unbounded;

pub struct Solution {
    path: Vec<usize>,
    cb: Option<usize>,
    length: f32,
    avg_dist : f32,
    distances: Arc<Array2<f32>>
}

pub struct Station {
    pub symbol: i32,
    pub x: i32,
    pub y: i32,
}

impl Station {
    pub fn randomize(symbols : &Vec<i32>, lower : i32, upper : i32) -> Station{
        Station{
            symbol: *rand::thread_rng().choose(symbols).unwrap(),
            x: rand::thread_rng().gen_range(lower,upper),
            y: rand::thread_rng().gen_range(lower,upper),
        }
    }

    pub fn distance(s1 : &Station, s2 : &Station) -> f32 {
        (((s1.x - s2.x).pow(2) + (s1.y - s2.y).pow(2)) as f32).sqrt()
    }
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut  fmt::Formatter) -> fmt::Result{
        write!(f, "Station {} has x {} and y {}", self.symbol, self.x, self.y)
    }
}

impl Solution{
    fn new(path : Vec<usize>, cb : Option<usize>, distances : Arc<Array2<f32>>) -> Solution {
        Solution {
            path,
            cb,
            length: 0.,
            avg_dist: 0.,
            distances: distances
        }
    }

    fn total_length(&mut self) {
        // calculate total length
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id : usize, receiver : crossbeam::Receiver<Message>, sender : crossbeam::Sender<Message>) -> Worker{
        // spawn thread
        let thread = thread::spawn(move || loop {
            let message = receiver.recv().unwrap();

            match message {
                Message::NewJob(solution) => {
                    let mut solution = solution;
                    solution.total_length();
                    sender.send(Message::NewJob(solution));
                }
                Message::Terminate => {
                    break;
                }
                Message::FinishedJob(_) => {
                    println!("A finished job was submitted to a queue processing lengths!");
                    panic!();
                }
            }
        });
        Worker {
            id,
            thread: None
        }
    }
}

struct ThreadPool{
    workers: Vec<Worker>,
    sender: crossbeam::Sender<Message>,
    receiver: crossbeam::Receiver<Message>
}

impl ThreadPool {
    fn new(size : usize) -> ThreadPool{
        let mut workers : Vec<Worker> = Vec::with_capacity(size);

        // create channels for sending jobs to threads
        let (solutions_raw_sender, solutions_raw_receiver) = unbounded::<Message>();

        // create channels for sending finished jobs back
        let (solutions_evaluated_sender, solutions_evaluated_receiver) = unbounded::<Message>();

        for i in 0..size {
            let worker = Worker::new(i, solutions_raw_receiver.clone(), solutions_evaluated_sender.clone());
            workers.push(worker);
        }

        ThreadPool {
            workers,
            sender: solutions_raw_sender,
            receiver: solutions_evaluated_receiver
        }
    }

    fn send(&self, message : Message) {
        self.sender.send(message).unwrap();
    }

    fn finish_sending(&self) {
        // pad the queue so that all workers get the message that they're done
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

    }

    /*fn wait_for_evaluating(self) {
        for worker in self.workers.iter_mut() {
            if let Some(thread) = &worker.thread {
                thread.join().unwrap();
            }
            
        }
    }*/
}

enum Message{
    NewJob(Solution),
    FinishedJob(Solution),
    Terminate
}

pub struct Exhaustive{
    best: Option<Solution>,
    stations : Vec<Station>
}

impl Exhaustive {
    fn new(stations : Vec<Station>) -> Exhaustive{
        Exhaustive {
            best: Option::None,
            stations: stations
        }
    }

    fn search(&mut self) -> Option<Solution> {
        // first create a vector that represents station indices
        let n_stations = self.stations.len();
        let station_indices = 0..n_stations;

        // calculate distances
        let mut distances : Array2<f32> = Array::zeros((n_stations, n_stations));

        for i in 0..n_stations {
            for j in 0..n_stations {
                distances[[i,j]] = Station::distance(&self.stations[i], &self.stations[j]);
            }
        }

        let distances = Arc::new(distances);

        // initialize thread pool for jobs
        let n_threads = 8;
        let pool = ThreadPool::new(8);

        // then create permutations of that vector
        let it = station_indices.into_iter().permutations(n_stations);
        for p in it {
            // create a solution
            let mut solution = Solution {
                path: p,
                cb: None,
                length: 0.,
                avg_dist: 0.,
                distances: distances.clone()
            };
            // then give them as jobs to evaluators
            // send solution to the evaluating queue
            pool.send(Message::NewJob(solution));

        }
        
        // send termination message to all evaluators
        pool.finish_sending();
        //pool.wait_for_evaluating();
        // collect results from evaluators
        // return the best result
        return Option::None
    }
}