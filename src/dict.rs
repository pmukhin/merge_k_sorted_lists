use std::fmt::Debug;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const INITIAL_CARDINALITY: usize          = 64;
const INITIAL_SEARCH_CLOSEST_SLOTS: usize = 6;

#[derive(Debug, Clone, Default)]
enum Pair<K: Debug, V: Clone + Debug> {
    #[default]
    Empty,
    Some(K, V),
}

pub struct Dict<K: Clone + Eq + Hash + Debug, V: Clone + Debug> {
    curr_size: usize,
    search_closest_slots: usize,
    data: Vec<Pair<K, V>>,
}

#[warn(dead_code)]
impl<K: Clone + Eq + Hash + Debug, V: Clone + Debug> Dict<K, V> {
    fn new() -> Dict<K, V> {
        let total_slots = INITIAL_CARDINALITY + (INITIAL_SEARCH_CLOSEST_SLOTS - 1);
        let mut data = Vec::with_capacity(total_slots);
        (0..total_slots).for_each(|_| data.push(Pair::default()));

        Dict {
            curr_size: INITIAL_CARDINALITY,
            data: data,
            search_closest_slots: INITIAL_SEARCH_CLOSEST_SLOTS,
        }
    }

    fn get_slot(&mut self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let hash: u64 = hasher.finish() / 31;
        let idx = (hash % (self.curr_size as u64)) as usize;

        println!("return idx={}", idx);
        idx
    }

    fn resize(&mut self) {
        let new_size = self.curr_size * 2;
        let new_search_closest_slots = self.search_closest_slots * 1.6 as usize;
        let total_new_size = new_size + new_search_closest_slots;
        let mut new_data: Vec<Pair<K, V>> = Vec::with_capacity(total_new_size);

        (0..total_new_size).for_each(|_| new_data.push(Pair::Empty));

        let old_data = self.data.to_vec();

        self.curr_size = new_size;
        self.data = new_data;
        self.search_closest_slots = new_search_closest_slots;

        old_data.iter().for_each(|pair| match pair {
            Pair::Some(k, v) => self.put(k.clone(), v.clone()),
            _ => (),
        });

        println!("resized to {}", self.curr_size)
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        let idx = self.get_slot(key);

        for i in 0..self.search_closest_slots {
            match &self.data[idx + i] {
                Pair::Empty => break,
                Pair::Some(k, v) => {
                    if *key == *k {
                        return Option::Some(v.clone());
                    }
                }
            }
        }
        Option::None
    }

    fn insert(&mut self, idx: usize, key: K, value: V) {
        self.data[idx] = Pair::Some(key, value);
    }

    pub fn put(&mut self, key: K, value: V) {
        let idx = self.get_slot(&key);

        for i in 0..self.search_closest_slots {
            match &self.data[idx + i] {
                Pair::Empty => {
                    return self.insert(idx + i, key, value);
                }
                Pair::Some(__key, __value) if *__key == key => {
                    return self.insert(idx + i, key, value);
                }
                _ => (), // do nothing
            }
        }

        self.resize();
        self.put(key, value);
    }

    pub fn diagnosis(&mut self) {
        println!("DIAGNOSIS:");

        for i in 0..self.curr_size {
            let node = &self.data[i];
            let i_s = if i < 10 {
                format!("0{}", i)
            } else {
                format!("{}", i)
            };
            println!("{i_s}: {:?}", node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    fn with_strings(size: usize) {
        let mut dict: Dict<String, String> = Dict::new();

        let keys = (0..size).map(|i| format!("key-of-{}", i)).collect::<Vec<_>>();
        let values = keys
            .iter()
            .map(|i| format!("value-of-key-of-{}", i))
            .collect::<Vec<_>>();
        let new_values = keys
            .iter()
            .enumerate()
            .map(|(i, v)| {
                if i % 2 == 0 {
                    format!("newvalue-of-{}", v)
                } else {
                    v.to_string()
                }
            })
            .collect::<Vec<_>>();
        for (k, v) in keys.iter().zip(&values) {
            dict.put(k.to_string(), v.to_string());
        }
        for (k, v) in keys.iter().zip(&values) {
            assert_eq!(dict.get(k), Option::Some(v.to_string()));
        }
        for (k, v) in keys.iter().zip(&new_values) {
            dict.put(k.to_string(), v.to_string());
        }
        for (k, v) in keys.iter().zip(&new_values) {
            assert_eq!(dict.get(k), Option::Some(v.to_string()));
        }
        dict.diagnosis();
    }

    #[test]
    fn test_with_chars() {
        let mut dict: Dict<char, usize> = Dict::new();
        let keys = (b'A'..=b'z')
            .map(|c| c as char)
            .filter(|c| c.is_alphabetic())
            .collect::<Vec<_>>();
        let values = (0..keys.len()).collect::<Vec<_>>();
        for (k, v) in keys.iter().zip(values) {
            dict.put(*k, v);
            assert_eq!(dict.get(k), Option::Some(v));
        }
    }

    #[test]
    fn test_with_strings() {
        with_strings(2400)
    }

    #[bench]
    fn bench_with_strings(b: &mut Bencher) {
        b.iter(|| with_strings(40))
    }
}
