use super::{rawvaliter::RawValIter, rawvec::RawVec};

pub struct IntoIter<T> {
    pub _buf: RawVec<T>, // we do not care, we just need it to live
    pub iter: RawValIter<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // Drop any remaining elements
        for _x in &mut *self {}
    }
}
