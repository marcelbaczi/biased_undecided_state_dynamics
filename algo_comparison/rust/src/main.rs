#![allow(unused)]

use std::time::Instant;
use incr_stats::incr::Stats;
mod usd;
mod usd_fast;

fn main() {
    let mut s = Stats::new();
    for _ in 0..3 {
        let now = Instant::now();
        let res = usd::run(even_distributed(10000000, 1000),200_000_000);
        let time = now.elapsed().as_millis();
        //println!("{}", res.0);
        s.update(time as f64);
        println!("Finished in {} msec", time);
    }
    println!("Mean: {}", s.mean().unwrap());
}

fn even_distributed(ammount: usize,opinions: usize) -> Vec<usize>{
    let num_per_opinion = ammount / opinions;
    let mut v = Vec::new();
    v.push(0);
    for _ in 0..opinions {
        v.push(num_per_opinion);
    }
    v
}