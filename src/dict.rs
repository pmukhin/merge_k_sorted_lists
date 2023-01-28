use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};
use std::fmt::Debug;

const INITIAL_SIZE: usize = 64;
const INITIAL_SEARCH_CLOSEST_SLOTS: usize = 6;

#[derive(Debug, Clone, Default)]
enum Pair<K: Debug, V: Clone + Debug> {
    #[default]
    Empty,
    Some(K, V)
}

pub struct Dict<K: Clone + Eq + Hash + Debug, V: Clone + Debug> {
    curr_size: usize,
    search_closest_slots: usize,
    data: Vec<Pair<K, V>>
}

#[warn(dead_code)]
impl <K: Clone + Eq + Hash + Debug, V: Clone + Debug> Dict<K, V> {
    fn new() -> Dict<K, V> {
        let mut data = Vec::with_capacity(INITIAL_SIZE);
        for _ in 0..INITIAL_SIZE {
            data.push(Pair::default());
        }
        Dict { curr_size: INITIAL_SIZE, data: data, search_closest_slots: INITIAL_SEARCH_CLOSEST_SLOTS }
    }

    fn get_slot(&mut self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let hash: u64 = hasher.finish() / 31;
        let idx = (hash % (self.curr_size as u64)) as usize;

        println!("return idx={}", idx);
        idx
    }

    fn get_with_index(&mut self, key: &K) -> (Option<&V>, usize) {
        let idx = self.get_slot(key);
        
        for i in 0..self.search_closest_slots {
            match &self.data[idx+i] {
                Pair::Empty => break,
                Pair::Some(k, v) => {
                    if key == k {
                        return (Option::Some(v), idx);
                    }
                }
            }
        }

        (Option::None, idx)
    }

    fn resize(&mut self) {
        let new_size = self.curr_size * 2;
        let mut new_data: Vec<Pair<K, V>> = Vec::with_capacity(new_size);
        (0..new_size).for_each(|_| new_data.push(Pair::Empty));
        
        let old_data = self.data.to_vec();

        self.curr_size = new_size;
        self.data = new_data;
        self.search_closest_slots = self.search_closest_slots * 1.6 as usize;

        old_data.iter().for_each(|pair| {
            match pair {
                Pair::Some(k, v) => self.put(k.clone(), v.clone()),
                _ => (),
            }
        });
        println!("resized to {}", self.curr_size)
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        match self.get_with_index(key) {
            (Option::Some(v), _) => Option::Some(v.clone()),
            _ => Option::None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        let idx = self.get_slot(&key);
        
        for i in 0..self.search_closest_slots {
            if idx+i >= self.curr_size { break; } // GOTO resize :) }

            match &self.data[idx+i] {
                Pair::Empty => {
                    self.data[idx+i] = Pair::Some(key, value);
                    return;
                },
                Pair::Some(__key, __value) if *__key == key => {
                    self.data[idx+i] = Pair::Some(key, value);
                    return;
                },
                _ => () // do nothing
            }
        }

        self.resize();
        self.put(key, value);
    }

    pub fn diagnosis(&mut self) {
        println!("DIAGNOSIS:");

        for i in 0..self.curr_size {
            let node = &self.data[i];
            let i_s = if i < 10 { format!("0{}", i) } else { format!("{}", i) };
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

        let keys: Vec<String> = (0..size).map(|i| format!("key-of-{}", i)).collect();
        let values: Vec<String> = keys.iter().map(|i| format!("value-of-key-of-{}", i)).collect();
        let new_values: Vec<String> = 
            keys.iter().enumerate().map(|(i, v)| 
                if i % 2 == 0 { format!("newvalue-of-{}", v) } else { v.to_string() }
            ).collect();

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
    fn test_with_strings() {
        with_strings(2400)
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| with_strings(400))
    }
}