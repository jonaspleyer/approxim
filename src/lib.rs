// Copyright 2015 Brendan Zabarauskas
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A crate that provides facilities for testing the approximimate equality of floating-point
//! based types, using either relative difference, or units in the last place (ULPs)
//! comparisons.
//!
//! You can also use the `*_{eq, ne}!` and `assert_*_{eq, ne}!` macros to test for equality using a
//! more positional style:
//!
//! ```rust
//! #[macro_use]
//! extern crate approxim;
//!
//! use std::f64;
//!
//! # fn main() {
//! static ε: f64 = f64::EPSILON;
//! assert_abs_diff_eq!(1.0, 1.0);                        // ✅
//! assert_abs_diff_eq!(1.0, 1.0 + ε);                    // ✅ default: epsilon = f64::EPSILON
//! assert_abs_diff_ne!(1.0, 1.0 + ε+ε);                  // ❌ diff (2ε) exceeds default (ε); assert "ne" instead of "eq"
//! assert_abs_diff_eq!(1.0, 1.0 + ε+ε, epsilon = ε+ε);   // ✅ diff (2ε) ≤ "epsilon" param (2ε)
//!
//! assert_relative_eq!(1.0, 1.0);                        // ✅ compare abs(a - b) / max(a, b) to default (f64::EPSILON)
//! assert_relative_ne!(1.0, 1.1);                        // ❌ 0.1/1.1 ≥ ε (assert "ne" instead of "eq")
//! assert_relative_eq!(1.0, 1.1, max_relative = 0.1);    // ✅ 0.1/1.1 < 0.1
//! assert_relative_eq!(1.1, 1.0, max_relative = 0.1);    // ✅ order doesn't matter, cmp is commutative
//! assert_relative_ne!(1.0, 1.2, max_relative = 0.1);    // ❌ 0.2/1.2 > 0.1
//! assert_relative_ne!(0.0, 1e-6, max_relative = 1e-5);  // ❌ maximum possible relative diff is 1.0 (when one side is 0)
//! assert_relative_eq!(0.0, 1e-6, epsilon = 1e-5, max_relative = 1e-5);  // ✅ passing `epsilon` allows short-circuiting based on small abs diff
//!
//! assert_ulps_eq!(1., 1. + 1e-17);                // ✅ default: max_ulps = 4
//! assert_ulps_eq!(1., 1. + 1e-16);                // ✅ ""
//! assert_ulps_ne!(1., 1. + 1e-15);                // ❌ assert "ne" instead of "eq"
//! assert_ulps_eq!(1., 1. + 1e-15, max_ulps = 5);  // ✅ relaxed max_ulps
//! # }
//! ```
//!
//! See also the [`abs_diff_eq!`](AbsDiffEq::abs_diff_eq), [`relative_eq!`](RelativeEq::relative_eq) and [`ulps_eq!`](UlpsEq::ulps_eq) macros, which return [`bool`] instead of [`assert`]ing.
//!
//! # Implementing approximate equality for custom types
//!
//! The `*Eq` traits allow approximimate equalities to be implemented on types, based on the
//! fundamental floating point implementations.
//!
//! For example, we might want to be able to do approximimate assertions on a complex number type:
//!
//! ```rust
//! #[macro_use]
//! extern crate approxim;
//! # use approxim::{AbsDiffEq, RelativeEq, UlpsEq};
//!
//! #[derive(Debug, PartialEq)]
//! struct Complex<T> {
//!     x: T,
//!     i: T,
//! }
//! # impl<T: AbsDiffEq> AbsDiffEq for Complex<T> where T::Epsilon: Copy {
//! #     type Epsilon = T::Epsilon;
//! #     fn default_epsilon() -> T::Epsilon { T::default_epsilon() }
//! #     fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
//! #         T::abs_diff_eq(&self.x, &other.x, epsilon) &&
//! #         T::abs_diff_eq(&self.i, &other.i, epsilon)
//! #     }
//! # }
//! # impl<T: RelativeEq> RelativeEq for Complex<T> where T::Epsilon: Copy {
//! #     fn default_max_relative() -> T::Epsilon { T::default_max_relative() }
//! #     fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon)
//! #                   -> bool {
//! #         T::relative_eq(&self.x, &other.x, epsilon, max_relative) &&
//! #         T::relative_eq(&self.i, &other.i, epsilon, max_relative)
//! #     }
//! # }
//! # impl<T: UlpsEq> UlpsEq for Complex<T> where T::Epsilon: Copy {
//! #     fn default_max_ulps() -> u32 { T::default_max_ulps() }
//! #     fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
//! #         T::ulps_eq(&self.x, &other.x, epsilon, max_ulps) &&
//! #         T::ulps_eq(&self.i, &other.i, epsilon, max_ulps)
//! #     }
//! # }
//!
//! # fn main() {
//! let x = Complex { x: 1.2, i: 2.3 };
//!
//! assert_relative_eq!(x, x);
//! assert_ulps_eq!(x, x, max_ulps = 4);
//! # }
//! ```
//!
//! To do this we can implement [`AbsDiffEq`], [`RelativeEq`] and [`UlpsEq`] generically in terms
//! of a type parameter that also implements `AbsDiffEq`, `RelativeEq` and `UlpsEq` respectively.
//! This means that we can make comparisons for either `Complex<f32>` or `Complex<f64>`:
//!
//! ```rust
//! # use approxim::{AbsDiffEq, RelativeEq, UlpsEq};
//! # #[derive(Debug, PartialEq)]
//! # struct Complex<T> { x: T, i: T, }
//! #
//! impl<T: AbsDiffEq> AbsDiffEq for Complex<T> where
//!     T::Epsilon: Copy,
//! {
//!     type Epsilon = T::Epsilon;
//!
//!     fn default_epsilon() -> T::Epsilon {
//!         T::default_epsilon()
//!     }
//!
//!     fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
//!         T::abs_diff_eq(&self.x, &other.x, epsilon) &&
//!         T::abs_diff_eq(&self.i, &other.i, epsilon)
//!     }
//! }
//!
//! impl<T: RelativeEq> RelativeEq for Complex<T> where
//!     T::Epsilon: Copy,
//! {
//!     fn default_max_relative() -> T::Epsilon {
//!         T::default_max_relative()
//!     }
//!
//!     fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
//!         T::relative_eq(&self.x, &other.x, epsilon, max_relative) &&
//!         T::relative_eq(&self.i, &other.i, epsilon, max_relative)
//!     }
//! }
//!
//! impl<T: UlpsEq> UlpsEq for Complex<T> where
//!     T::Epsilon: Copy,
//! {
//!     fn default_max_ulps() -> u32 {
//!         T::default_max_ulps()
//!     }
//!
//!     fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
//!         T::ulps_eq(&self.x, &other.x, epsilon, max_ulps) &&
//!         T::ulps_eq(&self.i, &other.i, epsilon, max_ulps)
//!     }
//! }
//! ```
//!
//! # References
//!
//! Floating point is hard! Thanks goes to these links for helping to make things a _little_
//! easier to understand:
//!
//! - [Comparing Floating Point Numbers, 2012 Edition](
//!   https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
//! - [The Floating Point Guide - Comparison](http://floating-point-gui.de/errors/comparison/)
//! - [What Every Computer Scientist Should Know About Floating-Point Arithmetic](
//!   https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html)

#![no_std]
#![allow(clippy::transmute_float_to_int)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "num-complex")]
#[cfg_attr(docsrs, doc(cfg(feature = "num-complex")))]
extern crate num_complex;
extern crate num_traits;
#[cfg(feature = "ordered-float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered-float")))]
extern crate ordered_float;

mod abs_diff_eq;
mod relative_eq;
mod ulps_eq;

mod macros;

pub use abs_diff_eq::AbsDiffEq;
pub use relative_eq::RelativeEq;
pub use ulps_eq::UlpsEq;

/// The requisite parameters for testing for approximimate equality using a
/// absolute difference based comparison.
///
/// This is not normally used directly, rather via the
/// `assert_abs_diff_{eq|ne}!` and `abs_diff_{eq|ne}!` macros.
///
/// # Example
///
/// ```rust
/// use std::f64;
/// use approxim::AbsDiff;
///
/// AbsDiff::default().eq(&1.0, &1.0);
/// AbsDiff::default().epsilon(f64::EPSILON).eq(&1.0, &1.0);
/// ```
pub struct AbsDiff<A, B = A>
where
    A: AbsDiffEq<B> + ?Sized,
    B: ?Sized,
{
    /// The tolerance to use when testing values that are close together.
    pub epsilon: A::Epsilon,
}

impl<A, B> Default for AbsDiff<A, B>
where
    A: AbsDiffEq<B> + ?Sized,
    B: ?Sized,
{
    #[inline]
    fn default() -> AbsDiff<A, B> {
        AbsDiff {
            epsilon: A::default_epsilon(),
        }
    }
}

impl<A, B> AbsDiff<A, B>
where
    A: AbsDiffEq<B> + ?Sized,
    B: ?Sized,
{
    /// Replace the epsilon value with the one specified.
    #[inline]
    pub fn epsilon(self, epsilon: A::Epsilon) -> AbsDiff<A, B> {
        AbsDiff { epsilon }
    }

    /// Perform the equality comparison
    #[inline]
    #[must_use]
    pub fn eq(self, lhs: &A, rhs: &B) -> bool {
        A::abs_diff_eq(lhs, rhs, self.epsilon)
    }

    /// Perform the inequality comparison
    #[inline]
    #[must_use]
    pub fn ne(self, lhs: &A, rhs: &B) -> bool {
        A::abs_diff_ne(lhs, rhs, self.epsilon)
    }
}

/// The requisite parameters for testing for approximimate equality using a
/// relative based comparison.
///
/// This is not normally used directly, rather via the
/// `assert_relative_{eq|ne}!` and `relative_{eq|ne}!` macros.
///
/// # Example
///
/// ```rust
/// use std::f64;
/// use approxim::Relative;
///
/// Relative::default().eq(&1.0, &1.0);
/// Relative::default().epsilon(f64::EPSILON).eq(&1.0, &1.0);
/// Relative::default().max_relative(1.0).eq(&1.0, &1.0);
/// Relative::default().epsilon(f64::EPSILON).max_relative(1.0).eq(&1.0, &1.0);
/// Relative::default().max_relative(1.0).epsilon(f64::EPSILON).eq(&1.0, &1.0);
/// ```
pub struct Relative<A, B = A>
where
    A: RelativeEq<B> + ?Sized,
    B: ?Sized,
{
    /// The tolerance to use when testing values that are close together.
    pub epsilon: A::Epsilon,
    /// The relative tolerance for testing values that are far-apart.
    pub max_relative: A::Epsilon,
}

impl<A, B> Default for Relative<A, B>
where
    A: RelativeEq<B> + ?Sized,
    B: ?Sized,
{
    #[inline]
    fn default() -> Relative<A, B> {
        Relative {
            epsilon: A::default_epsilon(),
            max_relative: A::default_max_relative(),
        }
    }
}

impl<A, B> Relative<A, B>
where
    A: RelativeEq<B> + ?Sized,
    B: ?Sized,
{
    /// Replace the epsilon value with the one specified.
    #[inline]
    pub fn epsilon(self, epsilon: A::Epsilon) -> Relative<A, B> {
        Relative { epsilon, ..self }
    }

    /// Replace the maximum relative value with the one specified.
    #[inline]
    pub fn max_relative(self, max_relative: A::Epsilon) -> Relative<A, B> {
        Relative {
            max_relative,
            ..self
        }
    }

    /// Perform the equality comparison
    #[inline]
    #[must_use]
    pub fn eq(self, lhs: &A, rhs: &B) -> bool {
        A::relative_eq(lhs, rhs, self.epsilon, self.max_relative)
    }

    /// Perform the inequality comparison
    #[inline]
    #[must_use]
    pub fn ne(self, lhs: &A, rhs: &B) -> bool {
        A::relative_ne(lhs, rhs, self.epsilon, self.max_relative)
    }
}

/// The requisite parameters for testing for approximimate equality using an ULPs
/// based comparison.
///
/// This is not normally used directly, rather via the `assert_ulps_{eq|ne}!`
/// and `ulps_{eq|ne}!` macros.
///
/// # Example
///
/// ```rust
/// use std::f64;
/// use approxim::Ulps;
///
/// Ulps::default().eq(&1.0, &1.0);
/// Ulps::default().epsilon(f64::EPSILON).eq(&1.0, &1.0);
/// Ulps::default().max_ulps(4).eq(&1.0, &1.0);
/// Ulps::default().epsilon(f64::EPSILON).max_ulps(4).eq(&1.0, &1.0);
/// Ulps::default().max_ulps(4).epsilon(f64::EPSILON).eq(&1.0, &1.0);
/// ```
pub struct Ulps<A, B = A>
where
    A: UlpsEq<B> + ?Sized,
    B: ?Sized,
{
    /// The tolerance to use when testing values that are close together.
    pub epsilon: A::Epsilon,
    /// The ULPs to tolerate when testing values that are far-apart.
    pub max_ulps: u32,
}

impl<A, B> Default for Ulps<A, B>
where
    A: UlpsEq<B> + ?Sized,
    B: ?Sized,
{
    #[inline]
    fn default() -> Ulps<A, B> {
        Ulps {
            epsilon: A::default_epsilon(),
            max_ulps: A::default_max_ulps(),
        }
    }
}

impl<A, B> Ulps<A, B>
where
    A: UlpsEq<B> + ?Sized,
    B: ?Sized,
{
    /// Replace the epsilon value with the one specified.
    #[inline]
    pub fn epsilon(self, epsilon: A::Epsilon) -> Ulps<A, B> {
        Ulps { epsilon, ..self }
    }

    /// Replace the max ulps value with the one specified.
    #[inline]
    pub fn max_ulps(self, max_ulps: u32) -> Ulps<A, B> {
        Ulps { max_ulps, ..self }
    }

    /// Perform the equality comparison
    #[inline]
    #[must_use]
    pub fn eq(self, lhs: &A, rhs: &B) -> bool {
        A::ulps_eq(lhs, rhs, self.epsilon, self.max_ulps)
    }

    /// Perform the inequality comparison
    #[inline]
    #[must_use]
    pub fn ne(self, lhs: &A, rhs: &B) -> bool {
        A::ulps_ne(lhs, rhs, self.epsilon, self.max_ulps)
    }
}

#[doc(inline)]
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use approx_derive;

/// See [approx_derive]
///
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use approx_derive::AbsDiffEq;

/// See [approx_derive]
///
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use approx_derive::RelativeEq;
