extern crate rayon;

use std::cmp::Ordering;

mod mergesort;
mod quicksort;

pub trait ParallelSliceSort<T: Send> {
    fn par_sort(&mut self)
    where
        T: Ord;

    fn par_sort_by<F>(&mut self, compare: F)
    where
        F: Sync + Fn(&T, &T) -> Ordering;

    fn par_sort_by_key<B, F>(&mut self, f: F)
    where
        B: Ord,
        F: Sync + Fn(&T) -> B;

    fn par_sort_unstable(&mut self)
    where
        T: Ord;

    fn par_sort_unstable_by<F>(&mut self, compare: F)
    where
        F: Sync + Fn(&T, &T) -> Ordering;

    fn par_sort_unstable_by_key<B, F>(&mut self, f: F)
    where
        B: Ord,
        F: Sync + Fn(&T) -> B;
}

impl<T: Send> ParallelSliceSort<T> for [T] {
    fn par_sort(&mut self)
    where
        T: Ord
    {
        mergesort::sort(self, |a, b| a.lt(b));
    }

    fn par_sort_by<F>(&mut self, compare: F)
    where
        F: Sync + Fn(&T, &T) -> Ordering
    {
        mergesort::sort(self, |a, b| compare(a, b) == Ordering::Less);
    }

    fn par_sort_by_key<B, F>(&mut self, f: F)
    where
        B: Ord,
        F: Sync + Fn(&T) -> B
    {
        mergesort::sort(self, |a, b| f(a).lt(&f(b)));
    }

    fn par_sort_unstable(&mut self)
    where
        T: Ord,
    {
        quicksort::sort(self, |a, b| a.lt(b));
    }

    fn par_sort_unstable_by<F>(&mut self, compare: F)
    where
        F: Sync + Fn(&T, &T) -> Ordering,
    {
        quicksort::sort(self, |a, b| compare(a, b) == Ordering::Less);
    }

    fn par_sort_unstable_by_key<B, F>(&mut self, f: F)
    where
        B: Ord,
        F: Sync + Fn(&T) -> B,
    {
        quicksort::sort(self, |a, b| f(a).lt(&f(b)));
    }
}
