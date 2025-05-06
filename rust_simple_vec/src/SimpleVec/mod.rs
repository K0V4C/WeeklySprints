mod drain;
mod intoiter;
mod rawvaliter;
mod rawvec;

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use drain::Drain;
use intoiter::IntoIter;
use rawvaliter::RawValIter;
use rawvec::RawVec;

struct SimpleVec<T> {
    buf: RawVec<T>,
    len: usize,
}

unsafe impl<T: Send> Send for SimpleVec<T> {}
unsafe impl<T: Sync> Sync for SimpleVec<T> {}

impl<T> SimpleVec<T> {
    /*

        PUBLIC API

    */

    pub fn new() -> Self {
        assert!(
            std::mem::size_of::<T>() != 0,
            "We are not ready to handle ZSTs"
        );

        SimpleVec {
            buf: RawVec::new(),
            len: 0,
        }
    }

    pub fn drain(&mut self) -> Drain<T> {
        let iter = unsafe { RawValIter::new(&self) };

        // this is a mem::forget safety thing. If Drain is forgotten, we just
        // leak the whole Vec's contents. Also we need to do this *eventually*
        // anyway, so why not do it now?
        self.len = 0;

        Drain {
            iter,
            vec: PhantomData,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.len() == self.cap() {
            self.buf.grow();
        }

        unsafe {
            std::ptr::write(self.ptr().add(self.len), item);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            // Notice how we use new_len to index the vector
            self.len -= 1;
            unsafe { Some(std::ptr::read(self.ptr().add(self.len))) }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        // <= is valid because its same as push

        assert!(index <= self.len, "index out of bounds");

        if self.len == self.cap() {
            self.buf.grow();
        }

        unsafe {
            std::ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index.saturating_add(1)),
                self.len.saturating_sub(index),
            );
            std::ptr::write(self.ptr().add(index), elem);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len(), "index out of bounds");

        unsafe {
            self.len -= 1;
            let result = std::ptr::read(self.ptr().add(index));
            std::ptr::copy(
                self.ptr().add(index.saturating_add(1)),
                self.ptr().add(index),
                self.len.saturating_sub(index),
            );
            result
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /*

        GUTS AND INTERNALS

    */

    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }
}

impl<T> IntoIterator for SimpleVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        unsafe {
            let iter = RawValIter::new(&self);

            let buf = std::ptr::read(&self.buf);
            std::mem::forget(self);

            IntoIter { iter, _buf: buf }
        }
    }
}

impl<T> Deref for SimpleVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<T> DerefMut for SimpleVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_vec_creation() {
        let new_vec: SimpleVec<i32> = SimpleVec::new();
        assert!(
            new_vec.len() == 0,
            "Creating new vector failed, size is not 0"
        )
    }
}
