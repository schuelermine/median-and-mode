#[cfg(test)]
mod test;

use std::cmp::{min, Ordering};

use crate::common::noop;

// Algorithm stolen wholesale from Wikipedia: https://en.wikipedia.org/wiki/Median_of_medians

pub fn select_and_iterate<T: Ord + Clone>(
    values: &mut [T],
    index: usize,
    action: impl FnMut(&T),
) -> Option<usize> {
    let len = values.len();
    if len == 0 || index > len {
        return None;
    }
    Some(select_and_iterate_inner(values, index, action))
}

fn select_and_iterate_inner<T: Ord + Clone>(
    values: &mut [T],
    index: usize,
    mut action: impl FnMut(&T),
) -> usize {
    let len = values.len();
    debug_assert_ne!(len, 0);
    if len == 1 {
        debug_assert_eq!(index, 0);
        action(&values[0]);
        return 0;
    }
    let pivot_index = pivot(values);
    let pivot_index = partition(values, pivot_index, index, action);
    match index.cmp(&pivot_index) {
        Ordering::Less => select_and_iterate_inner(&mut values[0..pivot_index], index, noop),
        Ordering::Equal => pivot_index,
        Ordering::Greater => {
            select_and_iterate_inner(
                &mut values[pivot_index + 1..len],
                index - (pivot_index + 1),
                noop,
            ) + (pivot_index + 1)
        }
    }
}

fn pivot<T: Ord + Clone>(values: &mut [T]) -> usize {
    let len = values.len();
    if len <= 5 {
        return median_of_5(values);
    }
    for i in (0..len - 1).step_by(5) {
        let right_index = min(i + 4, len - 1);
        let median = median_of_5(&mut values[i..right_index]);
        values.swap(median, i / 5);
    }
    select_and_iterate_inner(&mut values[0..len / 5], (len - 1) / 10, noop)
}

fn median_of_5<T: Ord>(values: &mut [T]) -> usize {
    values.sort();
    (values.len() - 1) / 2
}

fn partition<T: Ord + Clone>(
    values: &mut [T],
    pivot_index: usize,
    target_index: usize,
    mut action: impl FnMut(&T),
) -> usize {
    let len = values.len();
    let pivot_value_ref = &values[pivot_index];
    action(pivot_value_ref);
    let pivot_value = pivot_value_ref.clone();
    values.swap(pivot_index, len - 1);
    let mut store_index = 0;
    for i in 0..len - 1 {
        action(&values[i]);
        if values[i] < pivot_value {
            values.swap(store_index, i);
            store_index += 1;
        }
    }
    let mut store_index_eq = store_index;
    for i in store_index..len - 1 {
        if values[i] == pivot_value {
            values.swap(store_index_eq, i);
            store_index_eq += 1;
        }
    }
    values.swap(len - 1, store_index_eq);
    if target_index < store_index {
        store_index
    } else if target_index <= store_index_eq {
        target_index
    } else {
        store_index_eq
    }
}
