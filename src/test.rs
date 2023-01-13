use super::{median_and_mode, Median, MedianAndMode, Mode};
use std::collections::HashSet;

#[test]
fn test_median_and_mode_empty_array() {
    let values: [i128; 0] = [];
    let None = median_and_mode(&values)
        else { panic!() };
}
#[test]
fn test_median_and_mode_1() {
    let values: [i128; 12] = [
        30050, 17767, 12534, -24364, 20538, -17, 690, -7966, -40, -1172, -25598, 34,
    ];
    let Some(MedianAndMode { median, mode }) = median_and_mode(&values)
        else { panic!() };
    assert_eq!(median, Median::Between(-17, 34));
    assert_eq!(mode, Mode(HashSet::from_iter(values.iter().copied())))
}
#[test]
fn test_median_and_mode_2() {
    let values: [i128; 9] = [
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
    let Some(MedianAndMode { median, mode }) = median_and_mode(&values)
        else { panic!() };
    assert_eq!(median, Median::At(7952));
    assert_eq!(mode, Mode(HashSet::from_iter(values.iter().copied())));
}
