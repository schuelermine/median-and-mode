use super::select_and_iterate;
use crate::common::noop;
use proptest::{collection::vec, prop_assert_eq, proptest};
use std::collections::HashMap;

#[test]
fn test_select_empty_vec() {
    let mut values: Vec<i128> = vec![];
    let None = select_and_iterate(&mut values, 0, noop)
        else { panic!() };
}

proptest! {
    #[test]
    fn test_select(mut values in vec(i8::MIN..i8::MAX, 1..32768), index in 0..usize::MAX) {
        let len = values.len();
        if len == 0 {
            let None = select_and_iterate(&mut values, index, noop)
                else { panic!() };
        } else {
            let index = index % len;
            let mut frequencies = HashMap::new();
            let action = |x: &i8| {
                let frequency = frequencies.entry(*x).or_insert(0);
                *frequency += 1;
            };
            let Some(value_index) = select_and_iterate(&mut values, index, action)
                else { panic!() };
            let value = values[value_index];
            values.sort();
            prop_assert_eq!(value, values[index]);
            for (value, frequency) in frequencies {
                assert!(values.contains(&value));
                assert!(frequency <= len);
            }
        }
    }

    #[test]
    fn test_select_singleton_vec(value in i128::MIN..i128::MAX) {
        let mut values = vec![value];
        let Some(0) = select_and_iterate(&mut values, 0, noop)
            else { panic!() };
    }
}
