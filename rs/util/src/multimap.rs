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

    pub fn iter(&self) -> impl Iterator<Item = (&K, &Vec<V>)> {
        self.inner.iter()
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

    pub fn get_vals(&self, k: K) -> impl Iterator<Item = &V> {
        // self.inner.entry(k).or_default().iter()
        if let Some(v) = self.inner.get(&k) {
            Iter(Some(v.iter()))
        } else {
            Iter(None)
        }
    }

    pub fn get_vals_unchecked(&self, k: K) -> impl Iterator<Item = &V> {
        self.inner[&k].iter()
    }
}

impl<K, V> FromIterator<(K, V)> for MultiMap<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = Self::new();
        let mut iter = iter.into_iter();
        while let Some((k, v)) = iter.next() {
            map.insert(k, v);
        }
        map
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

struct Iter<'a, T: 'a>(Option<std::slice::Iter<'a, T>>);

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().map(|i| i.next()).flatten()
    }
}
