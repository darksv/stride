use core::ops::*;

use crate::Stride;

impl<T, U, const S: usize, const R: usize> PartialEq<Stride<U, R>> for Stride<T, S>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Stride<U, R>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T, const S: usize> Eq for Stride<T, S> where T: Eq {}

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
