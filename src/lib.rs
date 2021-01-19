//! This library provides a slice like [`Stride<T, S>`] type where elements are
//! spaced a constant `S` "stride" in memory.
//!
//! Where you want a strided slice use:
//! - [`&Stride<T, S>`][`Stride`] instead of `&[T]`
//! - [`&mut Stride<T, S>`][`Stride`] instead of `&mut [T]`

#![no_std]

mod iter;
mod ops;

pub use crate::iter::{Iter, IterMut};

/// A constant strided slice.
#[derive(Debug)]
#[repr(transparent)]
pub struct Stride<T, const S: usize> {
    data: [T],
}

impl<T, const S: usize> Default for &Stride<T, S> {
    fn default() -> Self {
        Stride::new(&[])
    }
}

impl<T, const S: usize> Default for &mut Stride<T, S> {
    fn default() -> Self {
        Stride::new_mut(&mut [])
    }
}

impl<T, const S: usize> Stride<T, S> {
    /// Construct a new strided slice.
    ///
    /// # Panics
    ///
    /// If the len of `data` is not a multiple of `S`.
    ///
    /// # Examples
    ///
    /// ```
    /// use stride::Stride;
    ///
    /// let data = &[1, 2, 3, 4, 5, 6];
    /// let strided = Stride::<_, 3>::new(data);
    /// ```
    pub fn new(data: &[T]) -> &Self {
        assert_eq!(data.len() % S, 0);
        unsafe { Self::new_unchecked(data) }
    }

    /// Construct a new mutable strided slice.
    ///
    /// # Panics
    ///
    /// If the len of `data` is not a multiple of `S`.
    ///
    /// # Examples
    ///
    /// ```
    /// use stride::Stride;
    ///
    /// let data = &mut [1, 2, 3, 4, 5, 6];
    /// let strided = Stride::<_, 3>::new_mut(data);
    /// ```
    pub fn new_mut(data: &mut [T]) -> &mut Self {
        assert_eq!(data.len() % S, 0);
        unsafe { Self::new_mut_unchecked(data) }
    }

    pub unsafe fn new_unchecked(data: &[T]) -> &Self {
        &*(data as *const [T] as *const Self)
    }

    pub unsafe fn new_mut_unchecked(data: &[T]) -> &mut Self {
        &mut *(data as *const [T] as *mut Self)
    }

    pub fn len(&self) -> usize {
        self.data.len() / S
    }

    /// Returns an iterator over the stride.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stride::Stride;
    /// #
    /// let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    /// let mut iterator = stride.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), Some(&3));
    /// assert_eq!(iterator.next(), Some(&5));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<T, S> {
        Iter::new(self)
    }

    /// Returns an iterator over the stride that allows modifying each value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stride::Stride;
    /// #
    /// let slice = &mut [1, 1, 2, 2, 3, 3];
    /// let stride = Stride::<_, 2>::new_mut(slice);
    /// for elem in stride.iter_mut() {
    ///     *elem *= 2;
    /// }
    /// assert_eq!(slice, &[2, 1, 4, 2, 6, 3]);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T, S> {
        IterMut::new(self)
    }
}
