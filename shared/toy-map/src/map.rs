//! A map of preserve order.
//!
//! By the map is backed by a [`IndexMap`].

use core::fmt::Formatter;
use core::iter::FromIterator;
use indexmap::IndexMap;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;

/// A map which preserves insertion order.
/// By the map is backed by a [`IndexMap`].
#[derive(Clone)]
pub struct Map<K, V> {
    map: IndexMap<K, V>,
}

impl<K, V> Map<K, V>
where
    K: Eq + Hash,
{
    #[inline]
    pub fn new() -> Self {
        Map {
            map: IndexMap::new(),
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Map {
            map: IndexMap::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.map.clear()
    }

    #[inline]
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + Eq + Hash,
    {
        self.map.get(key)
    }

    #[inline]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.get_mut(key)
    }

    #[inline]
    pub fn get_or_insert(&mut self, key: K, default: V) -> &mut V {
        self.map.entry(key).or_insert(default)
    }

    #[inline]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.contains_key(key)
    }

    #[inline]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.map.insert(k, v)
    }

    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        return self.map.swap_remove(key);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter()
    }

    #[inline]
    pub fn into_iter(self) -> impl IntoIterator<Item = (K, V)> {
        self.map.into_iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.map.iter_mut()
    }

    #[inline]
    pub fn keys(&self) -> impl ExactSizeIterator<Item = &K> {
        self.map.keys()
    }

    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.map.values()
    }
}

///////////////////////////////////////////////////////////////////////////////////

macro_rules! delegate_iterator {
    (($name:ident $($generics:tt)*) => $item:ty) => {
        impl $($generics)* Iterator for $name $($generics)* {
            type Item = $item;
            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next()
            }
            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.iter.size_hint()
            }
        }

        impl $($generics)* DoubleEndedIterator for $name $($generics)* {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.iter.next_back()
            }
        }

        impl $($generics)* ExactSizeIterator for $name $($generics)* {
            #[inline]
            fn len(&self) -> usize {
                self.iter.len()
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////

pub struct Iter<'a, K, V> {
    iter: indexmap::map::Iter<'a, K, V>,
}

delegate_iterator!((Iter<'a, K, V>) => (&'a K, &'a V));

impl<'a, K, V> IntoIterator for &'a Map<K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.map.iter(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////

pub struct IntoIter<K, V> {
    iter: indexmap::map::IntoIter<K, V>,
}

delegate_iterator!((IntoIter<K, V>) => (K, V));

impl<K, V> IntoIterator for Map<K, V>
where
    K: Eq + Hash,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.map.into_iter(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////

pub struct IterMut<'a, K, V> {
    iter: indexmap::map::IterMut<'a, K, V>,
}

delegate_iterator!((IterMut<'a, K, V>) => (&'a K, &'a mut V));

impl<'a, K, V> IntoIterator for &'a mut Map<K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            iter: self.map.iter_mut(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////

impl<K, V> Default for Map<K, V>
where
    K: Eq + Hash,
{
    #[inline]
    fn default() -> Self {
        Map {
            map: IndexMap::new(),
        }
    }
}

impl<K, V> fmt::Debug for Map<K, V>
where
    K: fmt::Debug + Hash + Eq,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.map.fmt(f)
    }
}

impl<K, V> fmt::Display for Map<K, V>
where
    K: fmt::Display + Hash + Eq,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut first = true;
        f.write_str("Map(")?;
        for (k, v) in self.iter() {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }
            f.write_fmt(format_args!("{}: {}", k, v))?;
        }
        f.write_str(")")?;
        Ok(())
    }
}

impl<K, V> PartialEq for Map<K, V>
where
    K: Eq + Hash + std::cmp::Ord,
    V: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<K, V> PartialOrd for Map<K, V>
where
    K: Eq + Hash + std::cmp::Ord + PartialEq,
    V: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.keys().partial_cmp(other.keys())
    }
}

impl<K, V> FromIterator<(K, V)> for Map<K, V>
where
    K: Eq + Hash + std::cmp::Ord + PartialEq,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        Map {
            map: FromIterator::from_iter(iter),
        }
    }
}

impl<K, V> Extend<(K, V)> for Map<K, V>
where
    K: Eq + Hash + std::cmp::Ord + PartialEq,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (K, V)>,
    {
        self.map.extend(iter);
    }
}
