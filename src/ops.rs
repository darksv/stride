use core::cmp::{self, Ordering};
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

impl<T, U, const S: usize, const R: usize> PartialOrd<Stride<U, R>> for Stride<T, S>
where
    T: PartialOrd<U>,
{
    fn partial_cmp(&self, other: &Stride<U, R>) -> Option<Ordering> {
        let len = cmp::min(self.len(), other.len());
        for i in 0..len {
            match self[i].partial_cmp(&other[i]) {
                Some(Ordering::Equal) => continue,
                non_eq => return non_eq,
            }
        }
        self.len().partial_cmp(&other.len())
    }
}

impl<T, const S: usize> Ord for Stride<T, S>
where
    T: Ord,
{
    fn cmp(&self, other: &Stride<T, S>) -> Ordering {
        let len = cmp::min(self.len(), other.len());
        for i in 0..len {
            match self[i].cmp(&other[i]) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            }
        }
        self.len().cmp(&other.len())
    }
}

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
