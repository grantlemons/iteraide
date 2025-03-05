#[derive(Debug)]
pub struct Positions<I, F> {
    iter: std::iter::Enumerate<I>,
    predicate: F,
}

pub trait PositionsIterator: Iterator + Sized {
    /// Iterator adapter for positions
    fn positions<F>(self, predicate: F) -> Positions<Self, F>
    where
        F: FnMut(Self::Item) -> bool,
    {
        Positions {
            iter: self.enumerate(),
            predicate,
        }
    }
}
impl<I: Iterator> PositionsIterator for I {}

impl<I, F> Iterator for Positions<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> bool,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let f = &mut self.predicate;
        self.iter.find_map(|(count, val)| f(val).then_some(count))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }

    fn fold<B, G>(self, init: B, mut func: G) -> B
    where
        G: FnMut(B, Self::Item) -> B,
    {
        let mut predicate = self.predicate;
        self.iter.fold(init, |mut acc, (count, val)| {
            if predicate(val) {
                acc = func(acc, count);
            }
            acc
        })
    }
}
