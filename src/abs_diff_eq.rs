use core::cell;
#[cfg(feature = "num-complex")]
use num_complex::Complex;
#[cfg(feature = "ordered-float")]
use num_traits::Float;
#[cfg(feature = "ordered-float")]
use ordered_float::{NotNan, OrderedFloat};

/// Equality that is defined using the absolute difference of two numbers.
///
/// For two numbers `a` and `b`, if `|a - b| < epsilon`, then the two numbers are considered
/// to be equal under the absolute difference equality.
///
/// `abs_diff_eq`, `abs_diff_ne`, `assert_abs_diff_eq`, and `assert_abs_diff_ne` macros
/// are all wrappers of the `abs_diff_eq` function in this trait.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate approxim;
/// # fn main() {
/// assert_abs_diff_eq!(1.0f32, 1.00000001f32, epsilon = 1e-8);
/// assert_abs_diff_ne!(1.0f32, 1.0000001f32, epsilon = 1e-8);
/// # }
/// ```
pub trait AbsDiffEq<Rhs = Self>: PartialEq<Rhs>
where
    Rhs: ?Sized,
{
    /// Used for specifying relative comparisons.
    type Epsilon;

    /// The default tolerance to use when testing values that are close together.
    ///
    /// This is used when no `epsilon` value is supplied to the
    /// [`abs_diff_eq!`](crate::abs_diff_eq), [`relative_eq!`](crate::relative_eq),
    /// or [`ulps_eq!`](crate::ulps_eq) macros.
    fn default_epsilon() -> Self::Epsilon;

    /// A test for equality that uses the absolute difference to compute the approximimate
    /// equality of two numbers.
    fn abs_diff_eq(&self, other: &Rhs, epsilon: Self::Epsilon) -> bool;

    /// The inverse of [`AbsDiffEq::abs_diff_eq`].
    fn abs_diff_ne(&self, other: &Rhs, epsilon: Self::Epsilon) -> bool {
        !Self::abs_diff_eq(self, other, epsilon)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Base implementations
///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_unsigned_abs_diff_eq {
    ($T:ident, $default_epsilon:expr) => {
        impl AbsDiffEq for $T {
            type Epsilon = $T;

            #[inline]
            fn default_epsilon() -> $T {
                $default_epsilon
            }

            #[inline]
            fn abs_diff_eq(&self, other: &$T, epsilon: $T) -> bool {
                (if self > other {
                    self - other
                } else {
                    other - self
                }) <= epsilon
            }
        }
    };
}

impl_unsigned_abs_diff_eq!(u8, 0);
impl_unsigned_abs_diff_eq!(u16, 0);
impl_unsigned_abs_diff_eq!(u32, 0);
impl_unsigned_abs_diff_eq!(u64, 0);
impl_unsigned_abs_diff_eq!(u128, 0);
impl_unsigned_abs_diff_eq!(usize, 0);

macro_rules! impl_signed_abs_diff_eq {
    ($T:ident, $default_epsilon:expr) => {
        impl AbsDiffEq for $T {
            type Epsilon = $T;

            #[inline]
            fn default_epsilon() -> $T {
                $default_epsilon
            }

            #[inline]
            #[allow(unused_imports)]
            fn abs_diff_eq(&self, other: &$T, epsilon: $T) -> bool {
                use num_traits::float::FloatCore;
                $T::abs(self - other) <= epsilon
            }
        }
    };
}

impl_signed_abs_diff_eq!(i8, 0);
impl_signed_abs_diff_eq!(i16, 0);
impl_signed_abs_diff_eq!(i32, 0);
impl_signed_abs_diff_eq!(i64, 0);
impl_signed_abs_diff_eq!(isize, 0);
impl_signed_abs_diff_eq!(f32, core::f32::EPSILON);
impl_signed_abs_diff_eq!(f64, core::f64::EPSILON);

///////////////////////////////////////////////////////////////////////////////////////////////////
// Derived implementations
///////////////////////////////////////////////////////////////////////////////////////////////////

impl<T: AbsDiffEq> AbsDiffEq for Option<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Option<T>, epsilon: T::Epsilon) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => T::abs_diff_eq(a, b, epsilon),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T: AbsDiffEq, E: AbsDiffEq> AbsDiffEq for Result<T, E> {
    type Epsilon = (T::Epsilon, E::Epsilon);

    #[inline]
    fn default_epsilon() -> (T::Epsilon, E::Epsilon) {
        (T::default_epsilon(), E::default_epsilon())
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Result<T, E>, epsilon: (T::Epsilon, E::Epsilon)) -> bool {
        match (self, other) {
            (Ok(a), Ok(b)) => T::abs_diff_eq(a, b, epsilon.0),
            (Err(a), Err(b)) => E::abs_diff_eq(a, b, epsilon.1),
            _ => false,
        }
    }
}

impl<'a, T: AbsDiffEq + ?Sized> AbsDiffEq for &'a T {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &&'a T, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(*self, *other, epsilon)
    }
}

impl<'a, T: AbsDiffEq + ?Sized> AbsDiffEq for &'a mut T {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &&'a mut T, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(*self, *other, epsilon)
    }
}

impl<T: AbsDiffEq + Copy> AbsDiffEq for cell::Cell<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &cell::Cell<T>, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.get(), &other.get(), epsilon)
    }
}

impl<T: AbsDiffEq + ?Sized> AbsDiffEq for cell::RefCell<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &cell::RefCell<T>, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.borrow(), &other.borrow(), epsilon)
    }
}

impl<A, B> AbsDiffEq<[B]> for [A]
where
    A: AbsDiffEq<B>,
    A::Epsilon: Clone,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn default_epsilon() -> A::Epsilon {
        A::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &[B], epsilon: A::Epsilon) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other).all(|(x, y)| A::abs_diff_eq(x, y, epsilon.clone()))
    }
}

#[cfg(feature = "array_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "array_impl")))]
impl<A, B, const N: usize> AbsDiffEq<[B; N]> for [A; N]
where
    A: AbsDiffEq<B>,
    A::Epsilon: Clone,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn default_epsilon() -> A::Epsilon {
        A::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &[B; N], epsilon: A::Epsilon) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other).all(|(x, y)| A::abs_diff_eq(x, y, epsilon.clone()))
    }
}

#[cfg(feature = "tuple_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "tuple_impl")))]
macro_rules! impl_abs_diff_eq {
    () => {
        impl AbsDiffEq for () {
            type Epsilon = ();

            fn default_epsilon() -> Self::Epsilon {}

            fn abs_diff_eq(&self, _other: &Self, _epsilon: Self::Epsilon) -> bool {
                true
            }
        }
    };

    ($($idx:tt),+) => {
        paste::paste! {
            impl<$( [<T $idx>], )+> AbsDiffEq for ($( [<T $idx>], )+)
            where
                $( [<T $idx>]: AbsDiffEq, )+
            {
                type Epsilon = ($( [<T $idx>]::Epsilon, )+);

                fn default_epsilon() -> Self::Epsilon {
                    ($( [<T $idx>]::default_epsilon(), )+)
                }

                fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                    true $( && self.$idx.abs_diff_eq(&other.$idx, epsilon.$idx) )+
                }
            }
        }
    };
}

#[cfg(feature = "tuple_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "tuple_impl")))]
mod abs_diff_eq_tuple_impls {
    use super::*;

    impl_abs_diff_eq!();
    impl_abs_diff_eq!(0);
    impl_abs_diff_eq!(0, 1);
    impl_abs_diff_eq!(0, 1, 2);
    impl_abs_diff_eq!(0, 1, 2, 3);
    impl_abs_diff_eq!(0, 1, 2, 3, 4);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5, 6);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5, 6, 7);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
}

#[cfg(feature = "num-complex")]
#[cfg_attr(docsrs, doc(cfg(feature = "num-complex")))]
impl<T: AbsDiffEq> AbsDiffEq for Complex<T>
where
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Complex<T>, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.re, &other.re, epsilon.clone())
            && T::abs_diff_eq(&self.im, &other.im, epsilon)
    }
}

#[cfg(feature = "ordered-float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered-float")))]
impl<T: AbsDiffEq + Copy> AbsDiffEq for NotNan<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        T::abs_diff_eq(&self.into_inner(), &other.into_inner(), epsilon)
    }
}

#[cfg(feature = "ordered-float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered-float")))]
impl<T: AbsDiffEq + Float + ordered_float::FloatCore> AbsDiffEq<T> for NotNan<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &T, epsilon: Self::Epsilon) -> bool {
        T::abs_diff_eq(&self.into_inner(), other, epsilon)
    }
}

#[cfg(feature = "ordered-float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered-float")))]
impl<T: AbsDiffEq + Float + ordered_float::FloatCore> AbsDiffEq for OrderedFloat<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        T::abs_diff_eq(&self.into_inner(), &other.into_inner(), epsilon)
    }
}

#[cfg(feature = "ordered-float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered-float")))]
impl<T: AbsDiffEq + Float + ordered_float::FloatCore> AbsDiffEq<T> for OrderedFloat<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &T, epsilon: Self::Epsilon) -> bool {
        T::abs_diff_eq(&self.into_inner(), other, epsilon)
    }
}
