pub trait CollectVec: Iterator + Sized {
    fn collect_vec(self) -> Vec<<Self as Iterator>::Item> {
        self.collect()
    }
}

impl<I: Iterator + Sized> CollectVec for I {}
