//! TODO: A note on where this code is taken from

use std::cmp;
use std::mem;
use std::ptr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;

/// Holds a value, but never drops it.
struct NoDrop<T> {
    value: Option<T>,
}

impl<T> Drop for NoDrop<T> {
    fn drop(&mut self) {
        mem::forget(self.value.take());
    }
}

/// When dropped, copies from `src` into `dest`.
struct CopyOnDrop<T> {
    src: *mut T,
    dest: *mut T,
}

impl<T> Drop for CopyOnDrop<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::copy_nonoverlapping(self.src, self.dest, 1);
        }
    }
}

/// Shifts the first element to the right until it encounters a greater or equal element.
pub fn shift_head<T, F>(v: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    unsafe {
        // If the first two elements are out-of-order...
        if len >= 2 && is_less(v.get_unchecked(1), v.get_unchecked(0)) {
            // Read the first element into a stack-allocated variable. If a following comparison
            // operation panics, `hole` will get dropped and automatically write the element back
            // into the slice.
            let mut tmp = NoDrop { value: Some(ptr::read(v.get_unchecked(0))) };
            let mut hole = CopyOnDrop {
                src: tmp.value.as_mut().unwrap(),
                dest: v.get_unchecked_mut(1),
            };
            ptr::copy_nonoverlapping(v.get_unchecked(1), v.get_unchecked_mut(0), 1);

            for i in 2..len {
                if !is_less(v.get_unchecked(i), tmp.value.as_ref().unwrap()) {
                    break;
                }

                // Move `i`-th element one place to the left, thus shifting the hole to the right.
                ptr::copy_nonoverlapping(v.get_unchecked(i), v.get_unchecked_mut(i - 1), 1);
                hole.dest = v.get_unchecked_mut(i);
            }
            // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
        }
    }
}

/// Shifts the last element to the left until it encounters a smaller or equal element.
pub fn shift_tail<T, F>(v: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    unsafe {
        // If the last two elements are out-of-order...
        if len >= 2 && is_less(v.get_unchecked(len - 1), v.get_unchecked(len - 2)) {
            // Read the last element into a stack-allocated variable. If a following comparison
            // operation panics, `hole` will get dropped and automatically write the element back
            // into the slice.
            let mut tmp = NoDrop { value: Some(ptr::read(v.get_unchecked(len - 1))) };
            let mut hole = CopyOnDrop {
                src: tmp.value.as_mut().unwrap(),
                dest: v.get_unchecked_mut(len - 2),
            };
            ptr::copy_nonoverlapping(v.get_unchecked(len - 2), v.get_unchecked_mut(len - 1), 1);

            for i in (0..len - 2).rev() {
                if !is_less(&tmp.value.as_ref().unwrap(), v.get_unchecked(i)) {
                    break;
                }

                // Move `i`-th element one place to the right, thus shifting the hole to the left.
                ptr::copy_nonoverlapping(v.get_unchecked(i), v.get_unchecked_mut(i + 1), 1);
                hole.dest = v.get_unchecked_mut(i);
            }
            // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
        }
    }
}

/// Partially sorts a slice by shifting several out-of-order elements around.
///
/// Returns `true` if the slice is sorted at the end. This function is `O(n)` worst-case.
#[cold]
pub fn partial_insertionsort<T, F>(v: &mut [T], is_less: &F) -> bool
where
    F: Fn(&T, &T) -> bool,
{
    // Maximum number of adjacent out-of-order pairs that will get shifted.
    const MAX_STEPS: usize = 5;
    // If the slice is shorter than this, don't shift any elements.
    const SHORTEST_SHIFTING: usize = 50;

    let len = v.len();
    let mut i = 1;

    for _ in 0..MAX_STEPS {
        unsafe {
            // Find the next pair of adjacent out-of-order elements.
            while i < len && !is_less(v.get_unchecked(i), v.get_unchecked(i - 1)) {
                i += 1;
            }
        }

        // Are we done?
        if i == len {
            return true;
        }

        // Don't shift elements on short arrays, that has a performance cost.
        if len < SHORTEST_SHIFTING {
            return false;
        }

        // Swap the found pair of elements. This puts them in correct order.
        v.swap(i - 1, i);

        // Shift the smaller element to the left.
        shift_tail(&mut v[..i], is_less);
        // Shift the greater element to the right.
        shift_head(&mut v[i..], is_less);
    }

    // Didn't manage to sort the slice in the limited number of steps.
    false
}

/// Sorts a slice using insertion sort, which is `O(n^2)` worst-case.
pub fn insertionsort<T, F>(v: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool,
{
    for i in 1..v.len() {
        shift_tail(&mut v[..i + 1], is_less);
    }
}
