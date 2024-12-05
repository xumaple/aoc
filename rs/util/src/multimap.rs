use std::collections::HashMap;
use std::{
    fmt::{self, Debug},
    hash::Hash,
};

#[derive(Default)]
pub struct MultiMap<K, V> {
    inner: HashMap<K, Vec<V>>,
}

impl<K: Clone, V: Clone> Clone for MultiMap<K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<K, V> PartialEq for MultiMap<K, V>
where
    K: Eq + Hash,
    V: PartialEq,
{
    fn eq(&self, other: &MultiMap<K, V>) -> bool {
        self.inner == other.inner
    }
}

impl<K, V> Eq for MultiMap<K, V>
where
    K: Eq + Hash,
    V: Eq,
{
}

impl<K: Debug, V: Debug> Debug for MultiMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MultiMap {{ {:?} }}", self.inner)
    }
}

impl<K, V> MultiMap<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}
impl<K, V> MultiMap<K, V>
where
    K: Eq + Hash,
{
    pub fn insert(&mut self, k: K, v: V) -> &Self {
        self.inner.entry(k).or_default().push(v);
        self
    }

    pub fn get_vals(&mut self, k: K) -> impl Iterator<Item = &V> {
        self.inner.entry(k).or_default().iter()
    }
}

impl<K, V> MultiMap<K, V>
where
    K: Eq + Hash,
    V: PartialEq,
{
    pub fn contains(&self, k: K, v: V) -> bool {
        match self.inner.get(&k) {
            Some(vals) => vals.contains(&v),
            None => false,
        }
    }
}
