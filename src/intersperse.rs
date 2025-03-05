impl<T: Clone, I: Iterator<Item = T> + Sized> IntersperseIterator for I {}

pub trait IntersperseIterator: Iterator + Sized
where
    <Self as Iterator>::Item: Clone,
{
    /// Iterator adapter for positions
    fn intersperse(self, elem: <Self as Iterator>::Item) -> Intersperse<std::iter::Fuse<Self>> {
        Intersperse {
            iter: self.fuse(),
            iter_next: None,
            intersperse_element: elem,
        }
    }
}

pub struct Intersperse<I>
where
    I: Iterator + Sized,
    I::Item: Clone,
{
    iter: I,
    iter_next: Option<Option<I::Item>>,
    intersperse_element: I::Item,
}

impl<I> Iterator for Intersperse<I>
where
    I: Iterator + Sized,
    I::Item: Clone,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter_next {
            Some(elem @ Some(_)) => elem.take(),
            Some(None) => match self.iter.next() {
                new @ Some(_) => {
                    self.iter_next = Some(new);
                    Some(self.intersperse_element.clone())
                }
                None => None,
            },
            None => {
                self.iter_next = Some(None);
                self.iter.next()
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.iter.size_hint();
        (hint.0 * 2, hint.1.map(|s| s * 2))
    }

    fn fold<B, G>(mut self, init: B, mut func: G) -> B
    where
        G: FnMut(B, Self::Item) -> B,
    {
        let mut acc = init;

        // take first element
        if let Some(x) = self.iter_next.unwrap_or_else(|| self.iter.next()) {
            acc = func(acc, x);
        }

        self.iter.fold(acc, |acc, elem| {
            let inner_acc = func(acc, self.intersperse_element.clone());
            func(inner_acc, elem)
        })
    }
}
