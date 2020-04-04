//! A map of String to data::Value.
//!
//! By default the map is backed by a [`IndexMap`].

use crate::data::Value;
use failure::_core::iter::FromIterator;
use indexmap::IndexMap;
use std::borrow::Borrow;
use std::hash::Hash;
use std::{fmt, fmt::Debug, ops};

pub struct Map<K, V> {
    map: IndexMap<K, V>,
}

impl Map<String, Value> {
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
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&Value>
    where
        String: Borrow<Q>,
        Q: Ord + Eq + Hash,
    {
        self.map.get(key)
    }

    #[inline]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.get_mut(key)
    }

    #[inline]
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        String: Borrow<Q>,
        Q: Ord + Eq + Hash,
    {
        self.map.contains_key(key)
    }

    #[inline]
    pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
        self.map.insert(k, v)
    }

    #[inline]
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<Value>
    where
        String: Borrow<Q>,
        Q: Ord + Eq + Hash,
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
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.map.iter()
    }

    #[inline]
    pub fn keys(&self) -> impl ExactSizeIterator<Item = &String> {
        self.map.keys()
    }

    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &Value> {
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

pub struct Iter<'a> {
    iter: indexmap::map::Iter<'a, String, Value>,
}

delegate_iterator!((Iter<'a>) => (&'a String, &'a Value));

impl<'a> IntoIterator for &'a Map<String, Value> {
    type Item = (&'a String, &'a Value);
    type IntoIter = Iter<'a>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.map.iter(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////

pub struct IntoIter {
    iter: indexmap::map::IntoIter<String, Value>,
}

delegate_iterator!((IntoIter) => (String, Value));

impl IntoIterator for Map<String, Value> {
    type Item = (String, Value);
    type IntoIter = IntoIter;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.map.into_iter(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////

impl Default for Map<String, Value> {
    #[inline]
    fn default() -> Self {
        Map {
            map: IndexMap::new(),
        }
    }
}

impl Clone for Map<String, Value> {
    #[inline]
    fn clone(&self) -> Self {
        Map {
            map: self.map.clone(),
        }
    }
}

impl PartialEq for Map<String, Value> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<'a, Q: ?Sized> ops::Index<&'a Q> for Map<String, Value>
where
    String: Borrow<Q>,
    Q: Ord + Eq + Hash,
{
    type Output = Value;

    fn index(&self, index: &Q) -> &Value {
        self.map.index(index)
    }
}

impl<'a, Q: ?Sized> ops::IndexMut<&'a Q> for Map<String, Value>
where
    String: Borrow<Q>,
    Q: Ord + Eq + Hash,
{
    fn index_mut(&mut self, index: &Q) -> &mut Value {
        self.map.get_mut(index).expect("no entry found for key")
    }
}

impl Debug for Map<String, Value> {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.map.fmt(formatter)
    }
}

impl FromIterator<(String, Value)> for Map<String, Value> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (String, Value)>,
    {
        Map {
            map: FromIterator::from_iter(iter),
        }
    }
}

impl Extend<(String, Value)> for Map<String, Value> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (String, Value)>,
    {
        self.map.extend(iter);
    }
}
