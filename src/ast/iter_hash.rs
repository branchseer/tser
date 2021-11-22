use std::borrow::{Borrow, BorrowMut};
use std::hash::{Hash, Hasher};
use indexmap::{IndexMap, IndexSet};

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct IterHash<T>(T);

impl<T> Hash for IterHash<T> where
        for<'a> &'a T: IntoIterator,
        for <'a> <&'a T as IntoIterator>::Item: Hash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in &self.0 {
            item.hash(state)
        }
    }
}

impl<T> AsRef<T> for IterHash<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for IterHash<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> From<T> for IterHash<T> {
    fn from(val: T) -> Self {
        Self(val)
    }
}

pub type HashableIndexMap<K, V> = IterHash<IndexMap<K, V>>;
pub type HashableIndexSet<T> = IterHash<IndexSet<T>>;
