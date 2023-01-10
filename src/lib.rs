pub mod median_and_mode {
    use std::{
        collections::{HashMap, HashSet},
        hash::Hash,
    };

    #[cfg(test)]
    mod tests {
        use core::panic;

        use super::*;
        #[test]
        #[should_panic]
        fn test_13646096770106105413() {
            let values: [isize; 0] = [];
            let Some(MedianAndMode { median: _, mode: _ }) = get_median_and_mode(&values)
            else { panic!() };
        }
        #[test]
        fn test_796468841128670673() {
            let mut values: [isize; 12] = [
                30050, 17767, 12534, -24364, 20538, -17, 690, -7966, -40, -1172, -25598, 34,
            ];
            values.sort();
            let Some(MedianAndMode { median, mode }) = get_median_and_mode(&values)
            else { panic!() };
            assert_eq!(median, Median::Between(-17, 34));
            assert_eq!(mode, Mode(HashSet::from_iter(values.iter().copied())))
        }
    }

    pub struct MedianAndMode<N: Hash + Eq> {
        pub median: Median<N>,
        pub mode: Mode<N>,
    }

    #[derive(Debug, PartialEq)]
    pub enum Median<N> {
        At(N),
        Between(N, N),
    }

    #[derive(Debug, PartialEq)]
    pub struct Mode<N: Hash + Eq>(HashSet<N>);

    pub fn get_median_and_mode<N>(values: &[N]) -> Option<MedianAndMode<N>>
    where
        N: Hash + Eq + Clone + Ord,
    {
        let len = values.len();
        if len == 0 {
            return None;
        }
        let mut frequencies = HashMap::new();
        let mut value_refs: Vec<&N> = values.iter().collect();
        let result;
        if len % 2 == 1 {
            let middle = len / 2;
            let median = index_and_count(&mut value_refs, &mut frequencies, middle).clone();
            let mode = get_mode(&frequencies);
            result = MedianAndMode {
                median: Median::At(median),
                mode: Mode(mode),
            };
        } else {
            let middle_1 = len / 2;
            let middle_2 = len / 2 - 1;
            let median_candidate_1 =
                index_and_count(&mut value_refs, &mut frequencies, middle_1).clone();
            let median_candidate_2 = do_index(&mut value_refs, middle_2).clone();
            let median;
            if median_candidate_1 == median_candidate_2 {
                median = Median::At(median_candidate_1);
            } else {
                median = Median::Between(median_candidate_1, median_candidate_2);
            }
            let mode = get_mode(&frequencies);
            result = MedianAndMode {
                median,
                mode: Mode(mode),
            }
        }
        Some(result)
    }

    fn index_and_count<'a, N>(
        value_refs: &mut [&'a N],
        frequencies: &mut HashMap<N, usize>,
        index: usize,
    ) -> &'a N
    where
        N: Hash + Eq + Clone + Ord,
    {
        let len = value_refs.len();
        debug_assert_ne!(len, 0);
        if len == 1 {
            debug_assert_eq!(index, 0);
            let value_ref = value_refs[0];
            let frequency = frequencies.entry(value_ref.clone()).or_insert(0);
            *frequency += 1;
            return value_ref;
        }
        let pivot_index = do_partition(value_refs);
        let pivot_value_ref = value_refs[pivot_index];
        let pivot_frequency = frequencies.entry(pivot_value_ref.clone()).or_insert(0);
        *pivot_frequency += 1;
        if index == pivot_index {
            do_count(&value_refs[0..pivot_index], frequencies);
            do_count(&value_refs[pivot_index + 1..len], frequencies);
            pivot_value_ref
        } else if index < pivot_index {
            do_count(&value_refs[pivot_index + 1..len], frequencies);
            index_and_count(&mut value_refs[0..pivot_index], frequencies, index)
        } else {
            do_count(&value_refs[0..pivot_index], frequencies);
            index_and_count(
                &mut value_refs[pivot_index + 1..len],
                frequencies,
                index - (pivot_index + 1),
            )
        }
    }

    fn do_index<T: Clone + PartialOrd>(values: &mut [T], index: usize) -> T {
        let len = values.len();
        debug_assert_ne!(len, 0);
        if len == 1 {
            debug_assert_eq!(index, 0);
            return values[0].clone();
        }
        let pivot_index = do_partition(values);
        if index == pivot_index {
            values[pivot_index].clone()
        } else if index < pivot_index {
            do_index(&mut values[0..pivot_index], index)
        } else {
            do_index(&mut values[pivot_index + 1..len], index - (pivot_index + 1))
        }
    }

    fn do_partition<T: PartialOrd + Clone>(values: &mut [T]) -> usize {
        let len = values.len();
        debug_assert_ne!(len, 0);
        if len == 1 {
            return 0;
        }
        let pivot_value_index = len / 2;
        let pivot_value = values[pivot_value_index].clone();
        values.swap(pivot_value_index, len - 1);
        let mut pivot_index = 0;
        for current_index in 0..len - 2 {
            if values[current_index] < pivot_value {
                values.swap(current_index, pivot_index);
                pivot_index += 1;
            }
        }
        pivot_index
    }

    fn do_count<N: Hash + Eq + Clone>(value_refs: &[&N], frequencies: &mut HashMap<N, usize>) {
        for value_ref in value_refs {
            let frequency = frequencies.entry((*value_ref).clone()).or_insert(0);
            *frequency += 1;
        }
    }

    fn get_mode<T: Hash + Eq + Clone>(frequencies: &HashMap<T, usize>) -> HashSet<T> {
        let mut most_frequent = HashSet::new();
        let mut highest_frequency: usize = 0;
        for (value, frequency) in frequencies {
            if *frequency == highest_frequency {
                most_frequent.insert(value.clone());
            }
            if *frequency > highest_frequency {
                most_frequent.clear();
                most_frequent.insert(value.clone());
                highest_frequency = *frequency;
            }
        }
        return most_frequent;
    }
}
