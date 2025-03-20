/// A structure that holds a reference to a slice and provides index-based comparison.
#[derive(Debug, Clone, Copy)]
pub struct SliceIndexOrdering<'a, T> {
    pub slice: &'a [T],
}

impl<'a, T: Ord> SliceIndexOrdering<'a, T> {
    /// Creates a new `SliceIndexOrdering` with the given slice.
    pub fn new(slice: &'a [T]) -> Self {
        Self { slice }
    }

    /// Creates an index wrapper that can be used for comparison.
    #[must_use]
    pub fn index(&'a self, index: usize) -> IndexWrapper<'a, T> {
        IndexWrapper {
            ordering: self,
            index,
        }
    }
}

/// A wrapper around an index that compares based on slice values.
#[derive(Debug, Clone, Copy)]
pub struct IndexWrapper<'a, T> {
    pub ordering: &'a SliceIndexOrdering<'a, T>,
    pub index: usize,
}

impl<T: Ord> PartialEq for IndexWrapper<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.ordering.slice[self.index] == other.ordering.slice[other.index]
    }
}

impl<T: Ord> Eq for IndexWrapper<'_, T> {}

impl<T: Ord> PartialOrd for IndexWrapper<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for IndexWrapper<'_, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ordering.slice[self.index].cmp(&other.ordering.slice[other.index])
    }
}
