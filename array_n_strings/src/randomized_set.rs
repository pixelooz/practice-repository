use rand::seq::IndexedRandom;

struct RandomizedSet {
    store: Vec<i32>,
    map: std::collections::HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            store: Vec::new(),
            map: std::collections::HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let curr_len = self.store.len();
        match self.map.entry(val) {
            std::collections::hash_map::Entry::Occupied(_) => false,
            std::collections::hash_map::Entry::Vacant(vacant) => {
                vacant.insert(curr_len);
                self.store.push(val);
                true
            }
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(index) = self.map.remove(&val) {
            self.store.swap_remove(index);
            if index < self.store.len() {
                let swapped_num = self.store[index];
                self.map.insert(swapped_num, index);
            }
            return true;
        }
        false
    }

    fn get_random(&self) -> i32 {
        *self.store.choose(&mut rand::rng()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randomized_set() {
        let mut randomized_set = RandomizedSet::new();

        // 1. Inserts 1 to the set. Returns true as 1 was inserted successfully.
        assert_eq!(randomized_set.insert(1), true);

        // 2. Returns false as 2 does not exist in the set.
        assert_eq!(randomized_set.remove(2), false);

        // 3. Inserts 2 to the set, returns true. Set now contains [1, 2].
        assert_eq!(randomized_set.insert(2), true);

        // 4. getRandom() should return either 1 or 2 randomly.
        let val = randomized_set.get_random();
        assert!(val == 1 || val == 2);

        // 5. Removes 1 from the set, returns true. Set now contains [2].
        assert_eq!(randomized_set.remove(1), true);

        // 6. 2 was already in the set, so return false.
        assert_eq!(randomized_set.insert(2), false);

        // 7. Since 2 is the only number in the set, getRandom() must return 2.
        assert_eq!(randomized_set.get_random(), 2);
    }
}
