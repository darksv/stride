use core::ops::*;

use crate::Stride;

impl<T, const S: usize> Index<usize> for Stride<T, S> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx * S]
    }
}

impl<T, const S: usize> IndexMut<usize> for Stride<T, S> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx * S]
    }
}
