//! This library provides a slice like [`Stride<T, S>`] type where elements are
//! spaced a constant `S` "stride" in memory.
//!
//! Where you want a strided slice use:
//! - [`&Stride<T, S>`][`Stride`] instead of `&[T]`
//! - [`&mut Stride<T, S>`][`Stride`] instead of `&mut [T]`

#![no_std]

mod ops;

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
}
