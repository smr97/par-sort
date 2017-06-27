#![feature(sort_unstable)]

extern crate rand;
extern crate par_sort;

use std::time::Instant;
use par_sort::ParallelSliceSort;
use rand::{Rng, thread_rng};

fn main() {
    {
        let mut v: Vec<u64> = thread_rng().gen_iter::<u64>().take(100_000_000).collect();
        let start = Instant::now();
        v.sort();
        let elapsed = start.elapsed();
        println!("sort              {} ms", elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000);
    }
    {
        let mut v: Vec<u64> = thread_rng().gen_iter::<u64>().take(100_000_000).collect();
        let start = Instant::now();
        v.sort_unstable();
        let elapsed = start.elapsed();
        println!("sort_unstable     {} ms", elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000);
    }
    {
        let mut v: Vec<u64> = thread_rng().gen_iter::<u64>().take(100_000_000).collect();
        let start = Instant::now();
        v.par_sort();
        let elapsed = start.elapsed();
        println!("par_sort          {} ms", elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000);
    }
    {
        let mut v: Vec<u64> = thread_rng().gen_iter::<u64>().take(100_000_000).collect();
        let start = Instant::now();
        v.par_sort_unstable();
        let elapsed = start.elapsed();
        println!("par_sort_unstable {} ms", elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000);
    }
}
