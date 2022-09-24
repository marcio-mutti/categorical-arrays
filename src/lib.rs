use std::{collections::HashMap, hash::Hash};

pub struct CategoricalArray<T: Hash> {
    values_map: HashMap<T, usize>,
    index_map: HashMap<usize, T>,
    values: Vec<usize>,
    last_unused_index: usize,
}

impl<T> Default for CategoricalArray<T>
where
    T: Hash,
{
    fn default() -> Self {
        Self {
            values_map: HashMap::new(),
            index_map: HashMap::new(),
            values: Vec::new(),
            last_unused_index: 0usize,
        }
    }
}
impl<T> CategoricalArray<T>
where
    T: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn number_of_categories(&self) -> usize {
        self.values_map.len()
    }
    pub fn push(&mut self, v: T) {
        if let Some(&idx) = self.values_map.get(&v) {
            self.values.push(idx);
        } else {
            self.values_map.insert(v.clone(), self.last_unused_index);
            self.values.push(self.last_unused_index);
            self.index_map.insert(self.last_unused_index, v);
            self.last_unused_index += 1;
        }
    }
}
impl<'a, T> CategoricalArray<T>
where
    T: Eq + Hash + Clone,
{
    pub fn iter(&'a self) -> CategoricalArrayIterator<'a, T> {
        CategoricalArrayIterator::new(self)
    }
    pub fn get(&'a self, pos: usize) -> Option<&'a T> {
        self.values.get(pos).map(|v| self.index_map.get(v).unwrap())
    }
}

pub struct CategoricalArrayIterator<'a, T: Hash> {
    array: &'a CategoricalArray<T>,
    pos: usize,
}

impl<'a, T> CategoricalArrayIterator<'a, T>
where
    T: Hash + Eq + Clone,
{
    pub fn new(array: &'a CategoricalArray<T>) -> Self {
        Self { array, pos: 0usize }
    }
}

impl<'a, T> Iterator for CategoricalArrayIterator<'a, T>
where
    T: Eq + Clone + Hash,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        self.array.get(self.pos - 1)
    }
}

#[cfg(test)]
mod tests;
