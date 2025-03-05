type VecIntoIter<T> = std::vec::IntoIter<T>;

impl<T: Ord, I: Iterator<Item = T>> SortedIterator for I {}
impl<T: Ord, I: Iterator<Item = T>> SortedUnstableIterator for I {}
impl<I: Iterator> SortedByKeyIterator for I {}
impl<I: Iterator> SortedUnstableByKeyIterator for I {}

pub trait SortedIterator: Iterator + Sized {
    /// Iterator adapter for positions
    fn sorted(self) -> VecIntoIter<<Self as Iterator>::Item>
    where
        <Self as Iterator>::Item: Ord,
    {
        let mut tmp: Vec<_> = self.collect();
        tmp.sort();
        tmp.into_iter()
    }
}

pub trait SortedUnstableIterator: Iterator + Sized {
    /// Iterator adapter for positions
    fn sorted(self) -> VecIntoIter<<Self as Iterator>::Item>
    where
        <Self as Iterator>::Item: Ord,
    {
        let mut tmp: Vec<_> = self.collect();
        tmp.sort_unstable();
        tmp.into_iter()
    }
}

pub trait SortedByKeyIterator: Iterator + Sized {
    /// Iterator adapter for positions
    fn sorted_by_key<T, F>(self, map: F) -> VecIntoIter<<Self as Iterator>::Item>
    where
        T: Ord,
        F: FnMut(&<Self as Iterator>::Item) -> T,
    {
        let mut tmp: Vec<_> = self.collect();
        tmp.sort_by_key(map);
        tmp.into_iter()
    }
}

pub trait SortedUnstableByKeyIterator: Iterator + Sized {
    /// Iterator adapter for positions
    fn sorted_unstable_by_key<T, F>(self, map: F) -> VecIntoIter<<Self as Iterator>::Item>
    where
        T: Ord,
        F: FnMut(&<Self as Iterator>::Item) -> T,
    {
        let mut tmp: Vec<_> = self.collect();
        tmp.sort_unstable_by_key(map);
        tmp.into_iter()
    }
}
