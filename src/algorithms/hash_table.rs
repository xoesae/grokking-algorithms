const INITIAL_TABLE_SIZE: usize = 10;

pub struct HashTable {
    buckets: Vec<Option<u32>>,
    size: usize,
}

impl HashTable {
    pub fn new() -> Self {
        Self {
            buckets: vec![None; INITIAL_TABLE_SIZE],
            size: INITIAL_TABLE_SIZE,
        }
    }

    pub fn hash(&self, hashable: u32) -> usize {
        return (hashable as usize) % self.size;
    }

    pub fn insert(&mut self, value: u32) {
        let index = self.hash(value.clone());

        match self.buckets[index] {
            Some(_) => {
                let mut i = self.hash((index + 1) as u32);

                while i != index && self.buckets[i].is_some() {
                    i = self.hash((index + 1) as u32);
                }

                if i != index {
                    self.buckets[i] = Some(value.clone());
                } else {
                    panic!("Table is full.");
                }
            },
            None => {
                self.buckets[index] = Some(value.clone());
            }
        }
    }

    pub fn find(&self, value: u32) -> Option<usize> {
        let mut index = self.hash(value.clone());

        while let Some(v) = self.buckets[index] {
            if v == value {
                return Some(index);
            }

            index = self.hash((index + 1) as u32);
        }

        None
    }

    #[allow(unused_assignments)]
    pub fn remove(&mut self, value: u32) -> Option<u32> {
        match self.find(value) {
            Some(mut index) => {
                let mut removed_value = None;

                loop {
                    let new_index = self.hash((index + 1) as u32);
                    
                    match self.buckets[new_index] {
                        Some(v) if (self.hash(v) == index) => {
                            self.buckets[index] = Some(v);
                            index = new_index;
                        },
                        _ => {
                            removed_value = self.buckets[index];
                            self.buckets[index] = None;
                            break;
                        }
                    }
                }

                removed_value
            },
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_function() {
        let table = HashTable::new();
        let hash = table.hash(15);

        assert_eq!(hash, 5);
    }

    #[test]
    fn test_insert_element() {
        let mut table = HashTable::new();
        table.insert(15);

        let any_not_none = table.buckets.iter().any(|&x| x.is_some());

        assert_eq!(any_not_none, true);
    }

    #[test]
    fn test_insert_element_with_collision() {
        let mut table = HashTable::new();
        table.insert(5);
        table.insert(15);

        let count_some = table.buckets.iter().filter(|&x| x.is_some()).count();

        assert_eq!(count_some, 2);
    }

    #[test]
    fn test_find_existing_element() {
        let mut table = HashTable::new();
        table.insert(15);
        let index = table.find(15).unwrap();

        assert_eq!(table.hash(15), index);
    }

    #[test]
    fn test_find_non_existing_element() {
        let table = HashTable::new();
        let index = table.find(15);

        assert_eq!(index, None);
    }

    #[test]
    fn test_find_element_with_collision() {
        let mut table = HashTable::new();
        table.insert(5);
        table.insert(15);

        let index = table.find(15).unwrap();
        
        assert_eq!(table.hash(15 + 1), index);
    }

    #[test]
    fn test_remove_existing_element() {
        let mut table = HashTable::new();
        table.insert(15);
        table.remove(15);

        let any_not_none = table.buckets.iter().any(|&x| x.is_some());

        assert_eq!(any_not_none, false);
    }
}