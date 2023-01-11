#[cfg(test)]
mod test;

pub fn quickselect_and_iterate<T: PartialOrd + Clone>(
    values: &mut [T],
    index: usize,
    action: impl FnMut(&T),
) -> Option<&T> {
    let len = values.len();
    if len == 0 || index > len {
        return None;
    }
    Some(quickselect_and_iterate_inner(values, index, action))
}

fn quickselect_and_iterate_inner<T: PartialOrd + Clone>(
    values: &mut [T],
    index: usize,
    mut action: impl FnMut(&T),
) -> &T {
    let len = values.len();
    debug_assert_ne!(len, 0);
    if len == 1 {
        debug_assert_eq!(index, 0);
        let value_ref = &values[0];
        action(value_ref);
        return value_ref;
    }
    let pivot_index = lomuto_partition(values);
    let pivot_value_ref = &values[pivot_index];
    action(pivot_value_ref);
    if pivot_index == index {
        for i in 0..pivot_index {
            action(&values[i]);
        }
        for i in pivot_index + 1..len {
            action(&values[i]);
        }
        &values[pivot_index]
    } else if index < pivot_index {
        for i in pivot_index + 1..len {
            action(&values[i]);
        }
        quickselect_and_iterate_inner(&mut values[0..pivot_index], index, action)
    } else {
        for i in 0..pivot_index {
            action(&values[i]);
        }
        quickselect_and_iterate_inner(
            &mut values[pivot_index + 1..len],
            index - (pivot_index + 1),
            action,
        )
    }
}

fn lomuto_partition<T: PartialOrd + Clone>(values: &mut [T]) -> usize {
    let len = values.len();
    debug_assert!(len > 1);
    let pivot_value = values[len / 2].clone();
    let mut pivot_index = 0;
    values.swap(len / 2, len - 1);
    for current_index in 0..len - 1 {
        if values[current_index] < pivot_value {
            values.swap(pivot_index, current_index);
            pivot_index += 1;
        }
    }
    values.swap(len - 1, pivot_index);
    pivot_index
}
