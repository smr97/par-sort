use std::cmp::Ordering::{Equal, Greater, Less};
use std::mem;

use rand::{thread_rng, Rng};
use test::Bencher;

use ParallelSliceSort;

macro_rules! test_sort {
    ($f:ident, $name:ident) => {
        #[test]
        fn $name() {
            let mut rng = thread_rng();

            for len in (0..25).chain(500..501) {
                for &modulus in &[5, 10, 100] {
                    for _ in 0..100 {
                        let v: Vec<_> = rng.gen_iter::<i32>()
                            .map(|x| x % modulus)
                            .take(len)
                            .collect();

                        // Test sort using `<` operator.
                        let mut tmp = v.clone();
                        tmp.$f(|a, b| a.cmp(b));
                        assert!(tmp.windows(2).all(|w| w[0] <= w[1]));

                        // Test sort using `>` operator.
                        let mut tmp = v.clone();
                        tmp.$f(|a, b| b.cmp(a));
                        assert!(tmp.windows(2).all(|w| w[0] >= w[1]));
                    }
                }
            }

            // Test sort with many duplicates.
            for &len in &[1000, 10_000, 100_000] {
                for &modulus in &[5, 10, 100, 10_000] {
                    let mut v: Vec<_> = rng.gen_iter::<i32>()
                        .map(|x| x % modulus)
                        .take(len)
                        .collect();

                    v.$f(|a, b| a.cmp(b));
                    assert!(v.windows(2).all(|w| w[0] <= w[1]));
                }
            }

            // Test sort with many pre-sorted runs.
            for &len in &[1000, 10_000, 100_000] {
                for &modulus in &[5, 10, 1000, 50_000] {
                    let mut v: Vec<_> = rng.gen_iter::<i32>()
                        .map(|x| x % modulus)
                        .take(len)
                        .collect();

                    v.sort();
                    v.reverse();

                    for _ in 0..5 {
                        let a = rng.gen::<usize>() % len;
                        let b = rng.gen::<usize>() % len;
                        if a < b {
                            v[a..b].reverse();
                        } else {
                            v.swap(a, b);
                        }
                    }

                    v.$f(|a, b| a.cmp(b));
                    assert!(v.windows(2).all(|w| w[0] <= w[1]));
                }
            }

            // Sort using a completely random comparison function.
            // This will reorder the elements *somehow*, but won't panic.
            let mut v: Vec<_> = (0..100).collect();
            v.$f(|_, _| *thread_rng().choose(&[Less, Equal, Greater]).unwrap());
            v.$f(|a, b| a.cmp(b));
            for i in 0..v.len() {
                assert_eq!(v[i], i);
            }

            // Should not panic.
            [0i32; 0].$f(|a, b| a.cmp(b));
            [(); 10].$f(|a, b| a.cmp(b));
            [(); 100].$f(|a, b| a.cmp(b));

            let mut v = [0xDEADBEEFu64];
            v.$f(|a, b| a.cmp(b));
            assert!(v == [0xDEADBEEF]);
        }
    }
}

test_sort!(par_sort_by, test_par_sort);
test_sort!(par_sort_unstable_by, test_par_sort_unstable);

#[test]
fn test_par_sort_stability() {
    for len in (2..25).chain(500..510).chain(50_000..50_010) {
        for _ in 0..10 {
            let mut counts = [0; 10];

            // Create a vector like [(6, 1), (5, 1), (6, 2), ...],
            // where the first item of each tuple is random, but
            // the second item represents which occurrence of that
            // number this element is, i.e. the second elements
            // will occur in sorted order.
            let mut v: Vec<_> = (0..len)
                .map(|_| {
                    let n = thread_rng().gen::<usize>() % 10;
                    counts[n] += 1;
                    (n, counts[n])
                })
                .collect();

            // Only sort on the first element, so an unstable sort
            // may mix up the counts.
            v.par_sort_by(|&(a, _), &(b, _)| a.cmp(&b));

            // This comparison includes the count (the second item
            // of the tuple), so elements with equal first items
            // will need to be ordered with increasing
            // counts... i.e. exactly asserting that this sort is
            // stable.
            assert!(v.windows(2).all(|w| w[0] <= w[1]));
        }
    }
}

fn gen_ascending(len: usize) -> Vec<u64> {
    (0..len as u64).collect()
}

fn gen_descending(len: usize) -> Vec<u64> {
    (0..len as u64).rev().collect()
}

fn gen_random(len: usize) -> Vec<u64> {
    let mut rng = thread_rng();
    rng.gen_iter::<u64>().take(len).collect()
}

fn gen_mostly_ascending(len: usize) -> Vec<u64> {
    let mut rng = thread_rng();
    let mut v = gen_ascending(len);
    for _ in (0usize..).take_while(|x| x * x <= len) {
        let x = rng.gen::<usize>() % len;
        let y = rng.gen::<usize>() % len;
        v.swap(x, y);
    }
    v
}

fn gen_mostly_descending(len: usize) -> Vec<u64> {
    let mut rng = thread_rng();
    let mut v = gen_descending(len);
    for _ in (0usize..).take_while(|x| x * x <= len) {
        let x = rng.gen::<usize>() % len;
        let y = rng.gen::<usize>() % len;
        v.swap(x, y);
    }
    v
}

fn gen_strings(len: usize) -> Vec<String> {
    let mut rng = thread_rng();
    let mut v = vec![];
    for _ in 0..len {
        let n = rng.gen::<usize>() % 20 + 1;
        v.push(rng.gen_ascii_chars().take(n).collect());
    }
    v
}

fn gen_big_random(len: usize) -> Vec<[u64; 16]> {
    let mut rng = thread_rng();
    rng.gen_iter().map(|x| [x; 16]).take(len).collect()
}

macro_rules! bench_sort {
    ($f:ident, $name:ident, $gen:expr, $len:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| $gen($len).$f());
            b.bytes = $len * mem::size_of_val(&$gen(1)[0]) as u64;
        }
    }
}

bench_sort!(par_sort, par_sort_large_ascending, gen_ascending, 50_000);
bench_sort!(par_sort, par_sort_large_descending, gen_descending, 50_000);
bench_sort!(par_sort, par_sort_large_mostly_ascending, gen_mostly_ascending, 50_000);
bench_sort!(par_sort, par_sort_large_mostly_descending, gen_mostly_descending, 50_000);
bench_sort!(par_sort, par_sort_large_random, gen_random, 50_000);
bench_sort!(par_sort, par_sort_large_big_random, gen_big_random, 50_000);
bench_sort!(par_sort, par_sort_large_strings, gen_strings, 50_000);

bench_sort!(par_sort_unstable, par_sort_unstable_large_ascending, gen_ascending, 50_000);
bench_sort!(par_sort_unstable, par_sort_unstable_large_descending, gen_descending, 50_000);
bench_sort!(par_sort_unstable, par_sort_unstable_large_mostly_ascending, gen_mostly_ascending, 50_000);
bench_sort!(par_sort_unstable, par_sort_unstable_large_mostly_descending, gen_mostly_descending, 50_000);
bench_sort!(par_sort_unstable, par_sort_unstable_large_random, gen_random, 50_000);
bench_sort!(par_sort_unstable, par_sort_unstable_large_big_random, gen_big_random, 50_000);
bench_sort!(par_sort_unstable, par_sort_unstable_large_strings, gen_strings, 50_000);
