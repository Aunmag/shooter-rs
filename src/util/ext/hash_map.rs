use std::hash::Hash;

pub trait HashMapExt<K, V> {
    fn pop(&mut self) -> Option<(K, V)>;
}

#[allow(clippy::unwrap_used)]
impl<K: Copy + Eq + Hash, V> HashMapExt<K, V> for std::collections::HashMap<K, V> {
    fn pop(&mut self) -> Option<(K, V)> {
        return self
            .keys()
            .next()
            .copied()
            .map(|k| (k, self.remove(&k).unwrap()));
    }
}

#[allow(clippy::unwrap_used)]
impl<K: Copy + Eq + Hash, V> HashMapExt<K, V> for bevy::utils::hashbrown::HashMap<K, V> {
    fn pop(&mut self) -> Option<(K, V)> {
        return self
            .keys()
            .next()
            .copied()
            .map(|k| (k, self.remove(&k).unwrap()));
    }
}
