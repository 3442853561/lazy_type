use std::fmt::{Debug, Formatter, Result};
use std::ops::{Index, IndexMut};

pub struct Array<T: Clone>(Box<[T]>);

impl<T: Clone> Array<T> {
    pub fn new(value: T, size: usize) -> Self {
        Array(Box::from(vec![value; size]))
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

impl<T: Clone + Debug> Debug for Array<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.0)
    }
}
