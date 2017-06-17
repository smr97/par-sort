extern crate par_sort;

use par_sort::ParallelSliceSort;

fn main() {
    let mut v = (0u64..100_000_000)
        .map(|x| x * x * x * 18913515181)
        .collect::<Vec<_>>();

    v.par_sort_unstable();
}
