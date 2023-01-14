mod common;
mod select_and_iterate;
#[cfg(test)]
mod test;

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::{common::noop, select_and_iterate::select_and_iterate};

#[derive(Debug, PartialEq, Eq)]
pub enum Median<T> {
    At(T),
    Between(T, T),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Mode<T: Eq + Hash>(HashSet<T>);

#[derive(Debug, PartialEq, Eq)]
pub struct MedianAndMode<T: Eq + Hash> {
    pub median: Median<T>,
    pub mode: Mode<T>,
}

pub fn median_and_mode<T: Ord + Eq + Hash + Clone>(values: &mut [T]) -> Option<MedianAndMode<T>> {
    let len = values.len();
    if len == 0 {
        return None;
    }
    let mut frequencies = HashMap::new();
    let action = |x: &T| {
        let frequency = frequencies.entry(x.clone()).or_insert(0);
        *frequency += 1;
    };
    let median;
    if len % 2 == 1 {
        let middle = len / 2;
        let Some(median_index) = select_and_iterate(values, middle, action)
            else { return None; };
        median = Median::At(values[median_index].clone());
    } else {
        let middle_1 = len / 2 - 1;
        let middle_2 = len / 2;
        let Some(median_1_index) = select_and_iterate(values, middle_1, action)
            else { return None; };
        let median_1 = values[median_1_index].clone();
        let Some(median_2_index) = select_and_iterate(values, middle_2, noop)
            else { panic!() };
        let median_2 = values[median_2_index].clone();
        if median_1 == median_2 {
            median = Median::At(median_1);
        } else {
            median = Median::Between(median_1, median_2);
        }
    }
    let mode = Mode(get_mode(frequencies));
    Some(MedianAndMode { median, mode })
}

fn get_mode<T: Eq + Hash>(frequencies: HashMap<T, usize>) -> HashSet<T> {
    let mut modes = HashSet::new();
    let mut highest_frequency = 0;
    for (value, frequency) in frequencies {
        match frequency.cmp(&highest_frequency) {
            std::cmp::Ordering::Less => {}
            std::cmp::Ordering::Equal => {
                modes.insert(value);
            }
            std::cmp::Ordering::Greater => {
                highest_frequency = frequency;
                modes.clear();
                modes.insert(value);
            }
        }
    }
    modes
}
