use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[cfg(test)]
mod test {
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
}

use crate::common::noop;
use crate::quickselect_and_iterate::quickselect_and_iterate;

#[derive(Debug, PartialEq)]
pub enum Median<T> {
    At(T),
    Between(T, T),
}

#[derive(Debug, PartialEq)]
pub struct Mode<T: Eq + Hash>(HashSet<T>);

#[derive(Debug, PartialEq)]
pub struct MedianAndMode<T: Eq + Hash> {
    pub median: Median<T>,
    pub mode: Mode<T>,
}

pub fn median_and_mode<T: PartialOrd + Eq + Hash + Clone>(
    values: &[T],
) -> Option<MedianAndMode<T>> {
    let len = values.len();
    if len == 0 {
        return None;
    }
    let mut working_values: Vec<&T> = values.iter().collect();
    let mut frequencies = HashMap::new();
    let action = |x: &&T| {
        let frequency = frequencies.entry((*x).clone()).or_insert(0);
        *frequency += 1;
    };
    let median;
    if len % 2 == 1 {
        let middle = len / 2;
        let Some(median_ref) = quickselect_and_iterate(&mut working_values, middle, action)
            else { return None; };
        median = Median::At((*median_ref).clone());
    } else {
        let middle_1 = len / 2 - 1;
        let middle_2 = len / 2;
        let Some(median_1_ref) = quickselect_and_iterate(&mut working_values, middle_1, action)
            else { return None; };
        let median_1 = (*median_1_ref).clone();
        let Some(median_2_ref) = quickselect_and_iterate(&mut working_values, middle_2, noop)
            else { panic!() };
        let median_2 = (*median_2_ref).clone();
        if median_1 == median_2 {
            median = Median::At(median_1);
        } else {
            median = Median::Between(median_1, median_2);
        }
    }
    let mode = Mode(get_mode(frequencies));
    return Some(MedianAndMode { median, mode });
}

fn get_mode<T: Eq + Hash>(frequencies: HashMap<T, usize>) -> HashSet<T> {
    let mut modes = HashSet::new();
    let mut highest_frequency = 0;
    for (value, frequency) in frequencies {
        if frequency > highest_frequency {
            highest_frequency = frequency;
            modes.clear();
            modes.insert(value);
        } else if frequency == highest_frequency {
            modes.insert(value);
        }
    }
    modes
}
