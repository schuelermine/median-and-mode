use super::select_and_iterate;
use crate::common::noop;
use proptest::{collection::vec, prop_assert, prop_assert_eq, proptest};
use std::collections::HashMap;

#[test]
fn test_select_empty_vec() {
    let mut values: Vec<i128> = vec![];
    let None = select_and_iterate(&mut values, 0, noop)
        else { panic!("Wrong result pattern") };
}

proptest! {
    #[test]
    fn proptest_select(mut values in vec(i8::MIN..i8::MAX, 1..32768), index in 0..usize::MAX) {
        let len = values.len();
        if len == 0 {
            let None = select_and_iterate(&mut values, index, noop)
                else { panic!("Wrong result pattern") };
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
                prop_assert!(values.contains(&value));
                prop_assert!(frequency <= len);
            }
        }
    }

    #[test]
    fn proptest_select_singleton_vec(value in i128::MIN..i128::MAX) {
        let mut counter = 0;
        let action = |_: &i128| {
            counter += 1;
        };
        let mut values = vec![value];
        let Some(0) = select_and_iterate(&mut values, 0, action)
            else { panic!("Wrong result pattern") };
        prop_assert_eq!(counter, 1);
    }
}
