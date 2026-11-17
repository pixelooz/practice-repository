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
