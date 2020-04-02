#![feature(sort_unstable)]

extern crate par_sort;
extern crate rand;
extern crate rayon;
extern crate rayon_adaptive;

use par_sort::ParallelSliceSort;
use rand::{thread_rng, Rng};
use rayon::current_num_threads;
use rayon_adaptive::merge_sort_adaptive;
use std::time::Instant;

const PROBLEM_SIZE: u64 = 100_000_000;
fn main() {
    {
        let mut v: Vec<u64> = (0..PROBLEM_SIZE).map(|_| rand::random()).collect();
        let start = Instant::now();
        v.sort();
        let elapsed = start.elapsed();
        println!(
            "sort              {} ms",
            elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000
        );
    }
    {
        let mut v: Vec<u64> = (0..PROBLEM_SIZE).map(|_| rand::random()).collect();
        let start = Instant::now();
        v.sort_unstable();
        let elapsed = start.elapsed();
        println!(
            "sort_unstable     {} ms",
            elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000
        );
    }
    {
        let mut v: Vec<u64> = (0..PROBLEM_SIZE).map(|_| rand::random()).collect();
        let start = Instant::now();
        v.par_sort();
        let elapsed = start.elapsed();
        println!(
            "par_sort          {} ms",
            elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000
        );
    }
    {
        let mut v: Vec<u64> = (0..PROBLEM_SIZE).map(|_| rand::random()).collect();
        let start = Instant::now();
        merge_sort_adaptive(&mut v);
        let elapsed = start.elapsed();
        println!(
            "adaptive sort {} ms",
            elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000
        );
    }
    {
        let mut v: Vec<u64> = (0..PROBLEM_SIZE).map(|_| rand::random()).collect();
        let start = Instant::now();
        v.par_sort_unstable();
        let elapsed = start.elapsed();
        println!(
            "par_sort_unstable {} ms",
            elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000
        );
    }
}
