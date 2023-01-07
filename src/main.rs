use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

macro_rules! dbg {
    () => {
        eprintln!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}

macro_rules! deprintln {
    [$fmt:literal] => {{
        #[cfg(debug_assertions)]
        { eprintln!($fmt); }
    }};
    [$fmt:literal, $($args:tt),+ $(,)?] => {{
    #[cfg(debug_assertions)]
    { eprintln!($fmt, $($args,)+); }
    }};
}

fn main() {
    let values = vec![
        30050, 17767, 12534, -24364, 20538, -17, 690, -7966, -40, -1172, -25598, 34,
    ];
    let MedianAndMode { median, mode } = median_and_mode(&values);
    println!("Median: {median:?}");
    println!("Mode: {mode:?}");
}

#[derive(Debug)]
pub enum Median<N> {
    At(N),
    Between(N, N),
}

#[derive(Debug)]
pub struct Mode<N>(HashSet<N>);

pub struct MedianAndMode<N> {
    pub median: Median<N>,
    pub mode: Mode<N>,
}

pub fn median_and_mode<N: Eq + Hash + PartialOrd + Copy + Debug>(values: &[N]) -> MedianAndMode<N> {
    let len = values.len();
    assert!(len != 0);
    let mut frequencies = HashMap::new();
    let mut working_values: Vec<&N> = values.iter().collect();
    if len % 2 == 1 {
        let middle = len / 2;
        let median = Median::At(index_and_count(
            &mut working_values,
            &mut frequencies,
            middle,
        ));
        dbg!(&frequencies);
        let mode = mode(&frequencies);
        MedianAndMode { median, mode }
    } else {
        let middle_1 = len / 2;
        let middle_2 = len / 2 + 1;
        let median_candidate_1 = index_and_count(&mut working_values, &mut frequencies, middle_1);
        let median_candidate_2 = do_index(&mut working_values, middle_2);
        let median;
        if median_candidate_1 == median_candidate_2 {
            median = Median::At(median_candidate_1);
        } else {
            median = Median::Between(median_candidate_1, median_candidate_2);
        }
        dbg!(&frequencies);
        let mode = mode(&frequencies);
        MedianAndMode { median, mode }
    }
}

fn index_and_count<N: PartialOrd + Eq + Hash + Copy + Debug>(
    values: &mut [&N],
    frequencies: &mut HashMap<N, usize>,
    index: usize,
) -> N {
    deprintln!("index_and_count({values:?}, {frequencies:?}, {index:?}");
    let len = values.len();
    debug_assert!(len != 0);
    if len == 1 {
        let value = *values[0];
        let frequency = frequencies.entry(value).or_insert(0);
        *frequency += 1;
        return value;
    }
    let pivot_index = partition(values);
    let pivot_value = *values[pivot_index];
    let pivot_frequency = frequencies.entry(pivot_value).or_insert(0);
    *pivot_frequency += 1;
    if index == pivot_index {
        count(&values[0..pivot_index], frequencies);
        count(&values[pivot_index + 1..len], frequencies);
        pivot_value
    } else if index < pivot_index {
        count(&values[pivot_index + 1..len], frequencies);
        index_and_count(&mut values[0..pivot_index], frequencies, index)
    } else {
        count(&values[0..pivot_index], frequencies);
        index_and_count(&mut values[pivot_index + 1..len], frequencies, index)
    }
}

fn count<N: Eq + Hash + Copy>(values: &[&N], frequencies: &mut HashMap<N, usize>) {
    for value in values {
        let frequency = frequencies.entry(**value).or_insert(0);
        *frequency += 1;
    }
}

fn partition<N: PartialOrd + Debug>(values: &mut [&N]) -> usize {
    deprintln!("partition({values:?})");
    let len = values.len();
    debug_assert!(len != 0);
    let pivot = values[(len - 1) / 2];
    let mut pivot_index = 0;
    for index in 0..len - 2 {
        if values[index] < pivot {
            pivot_index += 1;
            values.swap(pivot_index - 1, index);
            dbg!(pivot_index);
            dbg!(index);
            dbg!(&values);
        }
    }
    values.swap(pivot_index, len / 2);
    dbg!(&values);
    pivot_index
}

fn mode<N: Eq + Hash + Copy>(frequencies: &HashMap<N, usize>) -> Mode<N> {
    let mut most_frequent = HashSet::new();
    let mut highest_frequency: usize = 0;
    for (value, frequency) in frequencies {
        if *frequency == highest_frequency {
            most_frequent.insert(*value);
        }
        if *frequency > highest_frequency {
            most_frequent.clear();
            most_frequent.insert(*value);
            highest_frequency = *frequency;
        }
    }
    return Mode(most_frequent);
}

fn do_index<N: PartialOrd + Copy + Debug>(values: &mut [&N], index: usize) -> N {
    deprintln!("do_index({values:?}, {index:?})");
    let len = values.len();
    debug_assert!(len != 0);
    if len == 1 {
        return *values[0];
    }
    let pivot_index = partition(values);
    dbg!(pivot_index);
    if index == pivot_index {
        *values[index]
    } else if index < pivot_index {
        do_index(&mut values[0..pivot_index], index)
    } else {
        do_index(&mut values[pivot_index + 1..len], index)
    }
}
