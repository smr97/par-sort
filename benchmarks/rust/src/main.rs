#![feature(sort_unstable)]

extern crate par_sort;

use std::time::Instant;
use par_sort::ParallelSliceSort;

fn main() {
    {
        let mut v = (0u64..100_000_000)
            .map(|x| x.wrapping_mul(x).wrapping_mul(x).wrapping_mul(18913515181))
            .collect::<Vec<_>>();
        let start = Instant::now();
        v.sort();
        let elapsed = start.elapsed();
        println!("sort              {} ms", elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000);
    }
    {
        let mut v = (0u64..100_000_000)
            .map(|x| x.wrapping_mul(x).wrapping_mul(x).wrapping_mul(18913515181))
            .collect::<Vec<_>>();
        let start = Instant::now();
        v.sort_unstable();
        let elapsed = start.elapsed();
        println!("sort_unstable     {} ms", elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000);
    }
    {
        let mut v = (0u64..100_000_000)
            .map(|x| x.wrapping_mul(x).wrapping_mul(x).wrapping_mul(18913515181))
            .collect::<Vec<_>>();
        let start = Instant::now();
        v.par_sort();
        let elapsed = start.elapsed();
        println!("par_sort          {} ms", elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000);
    }
    {
        let mut v = (0u64..100_000_000)
            .map(|x| x.wrapping_mul(x).wrapping_mul(x).wrapping_mul(18913515181))
            .collect::<Vec<_>>();
        let start = Instant::now();
        v.par_sort_unstable();
        let elapsed = start.elapsed();
        println!("par_sort_unstable {} ms", elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000);
    }
}
