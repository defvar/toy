use std::ascii;
use std::cmp;
use std::fmt;
use std::iter::FromIterator;
use std::ops;

use super::edges::Edges;

#[derive(Clone)]
pub struct Line(Box<Inner>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Inner {
    raw: Vec<u8>,
    edges: Edges,
}

impl Line {
    pub fn new() -> Line {
        Line::with_capacity(0, 0)
    }

    pub fn with_capacity(buf: usize, column: usize) -> Line {
        Line(Box::new(Inner {
            raw: vec![0; buf],
            edges: Edges::with_capacity(column),
        }))
    }

    #[inline]
    pub fn parts(&mut self) -> (&mut Vec<u8>, &mut Vec<usize>) {
        (&mut self.0.raw, self.0.edges.get_edges_buf_mut())
    }

    /// clear all column, but keep inner buffer cause reuse.
    ///
    #[inline]
    pub fn clear(&mut self) {
        self.0.edges.clear()
    }

    #[inline]
    pub fn push(&mut self, column: &[u8]) {
        let s = self.0.edges.get_last_edge();
        let e = s + column.len();
        self.ensure_capacity(column.len());
        self.0.raw[s..e].copy_from_slice(column);
        self.0.edges.add(e);
    }

    #[inline]
    pub fn get(&self, idx: usize) -> Option<&[u8]> {
        self.0.edges.get(idx).map(|range| &self.0.raw[range])
    }

    #[inline]
    fn ensure_capacity(&mut self, size: usize) {
        let e = self.0.edges.get_last_edge() + size;
        while e > self.0.raw.len() {
            self.expand_force_columns();
        }
    }

    #[inline]
    pub fn expand_force_columns(&mut self) {
        let n = self.0.raw.len() * 2;
        self.0.raw.resize(cmp::max(4, n), 0);
    }

    #[inline]
    pub fn expand_force_edges(&mut self) {
        self.0.edges.expand_force();
    }

    /// return the number of column count in this row.
    ///
    #[inline]
    pub fn len(&self) -> usize {
        self.0.edges.len()
    }

    /// set column count.
    ///
    #[inline]
    pub fn set_len(&mut self, len: usize) {
        self.0.edges.set_len(len)
    }

    /// return row size.
    /// not column count, byte size.
    ///
    #[inline]
    pub fn len_bytes(&self) -> usize {
        self.0.raw[..self.0.edges.get_last_edge()].len()
    }

    #[inline]
    pub fn iter(&'_ self) -> ColumnIterator<'_> {
        self.into_iter()
    }
}

fn to_strings(line: &Line) -> Vec<String> {
    let mut cols = vec![];
    for col in line {
        let escaped: Vec<u8> = col.iter().flat_map(|&b| ascii::escape_default(b)).collect();
        cols.push(String::from_utf8(escaped).unwrap());
    }
    cols
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cols = to_strings(&self);
        write!(f, "Line({:?})", cols)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cols = to_strings(&self);
        write!(f, "{}", cols.join(","))
    }
}

impl ops::Index<usize> for Line {
    type Output = [u8];

    #[inline]
    fn index(&self, i: usize) -> &[u8] {
        self.get(i).unwrap()
    }
}

impl<A: AsRef<[u8]>> FromIterator<A> for Line {
    #[inline]
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut r = Line::new();
        r.extend(iter);
        r
    }
}

impl<A: AsRef<[u8]>> Extend<A> for Line {
    #[inline]
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        for x in iter {
            self.push(x.as_ref());
        }
    }
}

pub struct ColumnIterator<'a> {
    row: &'a Line,

    current_col_idx: usize,
    row_length: usize,

    // edge of prev item for forward iteration
    prev_edge: usize,

    // start of prev item for reverse iteration
    prev_start: usize,
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_col_idx == self.row_length {
            None
        } else {
            let s = self.prev_edge;
            let e = self.row.0.edges.get_edge(self.current_col_idx);
            self.current_col_idx += 1;
            self.prev_edge = e;
            Some(&self.row.0.raw[s..e])
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.row_length - self.current_col_idx;
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.row.len()
    }
}

impl<'a> DoubleEndedIterator for ColumnIterator<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [u8]> {
        if self.current_col_idx == self.row_length {
            None
        } else {
            self.row_length -= 1;
            let start = if self.row_length > 0 {
                self.row
                    .0
                    .edges
                    .get(self.row_length)
                    .map(|x| x.start)
                    .unwrap_or(0)
            } else {
                0
            };
            let end = self.prev_start;
            self.prev_start = start;
            Some(&self.row.0.raw[start..end])
        }
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = &'a [u8];
    type IntoIter = ColumnIterator<'a>;

    #[inline]
    fn into_iter(self) -> ColumnIterator<'a> {
        ColumnIterator {
            row: self,
            current_col_idx: 0,
            row_length: self.len(),
            prev_edge: 0,
            prev_start: self.len_bytes(),
        }
    }
}
