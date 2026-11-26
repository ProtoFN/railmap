pub struct RailMap<K: Ord, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K: Ord, V> Default for RailMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> RailMap<K, V> {
    pub const fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.keys.binary_search(key);

        match index {
            Ok(index) => self.values.get(index),
            Err(_) => None,
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let index = self.keys.binary_search(key);

        match index {
            Ok(index) => self.values.get_mut(index),
            Err(_) => None,
        }
    }

    pub fn entry<'a>(&'a mut self, key: K) -> Entry<'a, K, V> {
        let (index, exists) = match self.keys.binary_search(&key) {
            Ok(index) => (index, true),
            Err(index) => (index, false),
        };

        Entry {
            map: self,
            index,
            key,
            exists,
        }
    }
}

pub struct Entry<'a, K: Ord, V> {
    map: &'a mut RailMap<K, V>,
    index: usize,
    key: K,
    exists: bool,
}

impl<'a, K: Ord, V> Entry<'a, K, V> {
    pub fn or_insert_with<F>(self, func: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        #[cold]
        fn insert<K: Ord, V, F: FnOnce() -> V>(
            map: &mut RailMap<K, V>,
            index: usize,
            key: K,
            func: F,
        ) -> &mut V {
            map.keys.insert(index, key);

            map.values.insert_mut(index, func())
        }

        match self.exists {
            true => &mut self.map.values[self.index],
            false => insert(self.map, self.index, self.key, func),
        }
    }
}
