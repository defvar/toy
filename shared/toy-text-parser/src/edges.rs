use std::cmp;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edges {
    inner: Vec<usize>,
    column_count: usize,
}

impl Edges {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: vec![0; capacity],
            column_count: 0,
        }
    }

    /// return `Range` of column `i`.
    ///
    #[inline]
    pub fn get(&self, i: usize) -> Option<Range<usize>> {
        if i >= self.column_count {
            return None;
        }

        let end = match self.inner.get(i) {
            Some(&e) => e,
            None => return None,
        };

        let start = if i != 0 {
            match self.inner.get(i - 1) {
                Some(&s) => s,
                None => 0,
            }
        } else {
            0
        };

        Some(Range { start, end })
    }

    /// return inner buffer.
    ///
    pub fn get_edges_buf_mut(&mut self) -> &mut Vec<usize> {
        &mut self.inner
    }

    /// return edge of column `i`.
    ///
    #[inline]
    pub fn get_edge(&self, i: usize) -> usize {
        self.inner[i]
    }

    /// return edge of last column.
    ///
    #[inline]
    pub fn get_last_edge(&self) -> usize {
        self.inner[..self.column_count]
            .last()
            .map(|&i| i)
            .unwrap_or(0usize)
    }

    /// return the number of column count.
    ///
    #[inline]
    pub fn len(&self) -> usize {
        self.column_count
    }

    /// set column count.
    ///
    #[inline]
    pub(crate) fn set_len(&mut self, len: usize) {
        self.column_count = len;
    }

    /// reset edges.
    /// clear column count, keep inner buffer.
    ///
    #[inline]
    pub fn clear(&mut self) {
        self.column_count = 0
    }

    /// add last column.
    ///
    #[inline]
    pub fn add(&mut self, edge: usize) {
        self.ensure_capacity();
        self.inner[self.column_count] = edge;
        self.column_count += 1;
    }

    #[inline]
    fn ensure_capacity(&mut self) {
        if self.column_count >= self.inner.len() {
            self.expand_force();
        }
    }

    #[inline]
    pub fn expand_force(&mut self) {
        let n = self.inner.len() * 2;
        self.inner.resize(cmp::max(4, n), 0);
    }
}
