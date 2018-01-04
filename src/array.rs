use alloc::raw_vec::RawVec;

use std::fmt;
use std::ptr;
use std::ops::{Index, IndexMut};

struct _Array<T: Clone> {
    buf: RawVec<T>,
    len: usize,
}

impl<T: Clone> _Array<T> {
    fn new(value: T, size: usize) -> Self {
        let mut result: _Array<T> = _Array {
            buf: RawVec::new(),
            len: 0,
        };
        result.buf.reserve(0, size);
        for x in vec![value; size] {
            unsafe {
                ptr::write(result.buf.ptr().offset(result.len as isize), x.clone());
            }
            result.len += 1;
        }
        result
    }

    fn get(self) -> (Box<[T]>, Self) {
        let slice = unsafe { self.buf.into_box() };
        let mut reduzate: _Array<T> = _Array {
            buf: RawVec::new(),
            len: 0,
        };
        reduzate.set(&*slice.clone());
        (slice, reduzate)
    }

    fn set(&mut self, elems: &[T]) {
        self.buf.reserve(0, elems.len());
        for x in elems {
            unsafe {
                ptr::write(self.buf.ptr().offset(self.len as isize), x.clone());
            }
            self.len += 1;
        }
    }

    fn boxed(self) -> Array<T> {
        let (slice, _) = self.get();
        Array(slice)
    }
}

pub struct Array<T: Clone>(Box<[T]>);

impl<T: Clone> Array<T> {
    pub fn new(value: T, size: usize) -> Self {
        _Array::new(value, size).boxed()
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T: Clone> Index<usize> for Array<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.0[index]
    }
}

impl<T: Clone> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.0[index]
    }
}

impl<T: Clone + fmt::Debug> fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
