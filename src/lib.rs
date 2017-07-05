#![cfg_attr(test, feature(test))]
#![feature(specialization)]

extern crate rand;
extern crate rayon;
#[cfg(test)]
extern crate test;

use std::cmp::Ordering;

mod insertionsort;
mod mergesort;
mod quicksort;
mod samplesort;
#[cfg(test)]
mod tests;

use mergesort::par_mergesort;
use quicksort::par_quicksort;
use samplesort::par_samplesort;

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
        par_mergesort(self, |a, b| a.lt(b));
    }

    fn par_sort_by<F>(&mut self, compare: F)
    where
        F: Sync + Fn(&T, &T) -> Ordering
    {
        par_mergesort(self, |a, b| compare(a, b) == Ordering::Less);
    }

    fn par_sort_by_key<B, F>(&mut self, f: F)
    where
        B: Ord,
        F: Sync + Fn(&T) -> B
    {
        par_mergesort(self, |a, b| f(a).lt(&f(b)));
    }

    default fn par_sort_unstable(&mut self)
    where
        T: Ord,
    {
        par_quicksort(self, |a, b| a.lt(b));
    }

    default fn par_sort_unstable_by<F>(&mut self, compare: F)
    where
        F: Sync + Fn(&T, &T) -> Ordering,
    {
        par_quicksort(self, |a, b| compare(a, b) == Ordering::Less);
    }

    default fn par_sort_unstable_by_key<B, F>(&mut self, f: F)
    where
        B: Ord,
        F: Sync + Fn(&T) -> B,
    {
        par_quicksort(self, |a, b| f(a).lt(&f(b)));
    }
}

impl<T: Send + Sync> ParallelSliceSort<T> for [T] {
    fn par_sort_unstable(&mut self)
    where
        T: Ord,
    {
        par_samplesort(self, |a, b| a.lt(b));
    }

    fn par_sort_unstable_by<F>(&mut self, compare: F)
    where
        F: Sync + Fn(&T, &T) -> Ordering,
    {
        par_samplesort(self, |a, b| compare(a, b) == Ordering::Less);
    }

    fn par_sort_unstable_by_key<B, F>(&mut self, f: F)
    where
        B: Ord,
        F: Sync + Fn(&T) -> B,
    {
        par_samplesort(self, |a, b| f(a).lt(&f(b)));
    }
}
