use proptest::{collection::vec, prop_assert_eq, proptest};

use super::select_and_iterate;
use crate::common::noop;

#[test]
fn test_select_empty_vec() {
    let mut values: Vec<i128> = vec![];
    let None = select_and_iterate(&mut values, 0, noop)
        else { panic!() };
}

proptest! {
    #[test]
    fn test_select(mut values in vec(i128::MIN..i128::MAX, 0..100), index in 0..usize::MAX) {
        let len = values.len();
        if len == 0 {
            let None = select_and_iterate(&mut values, index, noop)
                else { panic!() };
        } else {
            let index = index % len;
            let Some(value_index) = select_and_iterate(&mut values, index, noop)
                else { panic!() };
            let value = values[value_index];
            values.sort();
            prop_assert_eq!(value, values[index]);
        }
    }

    #[test]
    fn test_select_singleton_vec(value in i128::MIN..i128::MAX) {
        let mut values = vec![value];
        let Some(value_index) = select_and_iterate(&mut values, 0, noop)
            else { panic!() };
        let value = values[value_index];
        prop_assert_eq!(value, values[0])
    }
}
