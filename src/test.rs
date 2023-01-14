use super::{median_and_mode, Median, MedianAndMode, Mode};
use proptest::{collection::vec, prop_assert, prop_assert_eq, prop_assert_ne, proptest};
// ^ See https://github.com/proptest-rs/proptest/issues/256
use std::collections::HashSet;

#[test]
fn test_median_and_mode_empty_array() {
    let mut values: [i128; 0] = [];
    let None = median_and_mode(&mut values)
        else { panic!() };
}
#[test]
fn test_median_and_mode_1() {
    let mut values: [i128; 12] = [
        30050, 17767, 12534, -24364, 20538, -17, 690, -7966, -40, -1172, -25598, 34,
    ];
    let Some(MedianAndMode { median, mode }) = median_and_mode(&mut values)
        else { panic!() };
    assert_eq!(median, Median::Between(-17, 34));
    assert_eq!(mode, Mode(HashSet::from_iter(values.iter().copied())))
}
#[test]
fn test_median_and_mode_2() {
    let mut values: [i128; 9] = [
        7952,
        19412,
        -1450,
        6978825196251534519,
        11125,
        5270098434161345047,
        -13739,
        -27060,
        -467,
    ];
    let Some(MedianAndMode { median, mode }) = median_and_mode(&mut values)
        else { panic!() };
    assert_eq!(median, Median::At(7952));
    assert_eq!(mode, Mode(HashSet::from_iter(values.iter().copied())));
}

proptest! {
    #[test]
    fn proptest_median_and_mode(mut values in vec(i8::MIN..i8::MAX, 1..32768)) {
        let len = values.len();
        if len == 0 {
            let None = median_and_mode(&mut values)
                else { panic!() };
        } else {
            let Some(MedianAndMode { median, mode }) = median_and_mode(&mut values)
                else { panic!() };
            values.sort();
            match median {
                Median::At(x) => {
                    prop_assert_eq!(x, values[len / 2]);
                },
                Median::Between(x, y) => {
                    prop_assert_eq!(x, values[len / 2 - 1]);
                    prop_assert_eq!(y, values[len / 2]);
                },
            }
            let Mode(mode) = mode;
            prop_assert_ne!(mode.len(), 0);
            let mut frequencies = Vec::new();
            for value in mode {
                frequencies.push(values.iter().filter(|n| **n == value).count())
            };
            let first_frequency = frequencies[0];
            prop_assert!(frequencies.iter().all(|n| *n == first_frequency));
        }
    }

    #[test]
    fn proptest_median_and_mode_singleton_vec(value in i128::MIN..i128::MAX) {
        let mut values = vec![value];
        let Some(MedianAndMode { median, mode }) = median_and_mode(&mut values)
            else { panic!() };
        prop_assert_eq!(median, Median::At(value));
        prop_assert_eq!(mode, Mode(HashSet::from([value])))
    }
}
