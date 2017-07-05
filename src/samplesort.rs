use std::mem;

use super::quicksort::par_quicksort;

pub fn par_samplesort<T, F>(v: &mut [T], is_less: F)
where
    T: Send + Sync,
    F: Fn(&T, &T) -> bool + Sync,
{
    // Sorting has no meaningful behavior on zero-sized types.
    if mem::size_of::<T>() == 0 {
        return;
    }

    par_quicksort(v, is_less);
    //unimplemented!()
}
