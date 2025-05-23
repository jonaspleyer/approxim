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

// Test cases derived from:
// https://github.com/Pybonacci/puntoflotante.org/blob/master/content/errors/NearlyEqualsTest.java
#![no_std]

#[macro_use]
extern crate approxim;

mod test_f32 {
    use core::f32;

    static EPSILON: f32 = f32::EPSILON;

    #[test]
    fn test_basic() {
        assert_abs_diff_eq!(1.0f32, 1.0f32);
        assert_abs_diff_ne!(1.0f32, 2.0f32);
    }

    #[test]
    #[should_panic]
    fn test_basic_panic_eq() {
        assert_abs_diff_eq!(1.0f32, 2.0f32);
    }

    #[test]
    #[should_panic]
    fn test_basic_panic_ne() {
        assert_abs_diff_ne!(1.0f32, 1.0f32);
    }

    #[test]
    fn test_big() {
        assert_abs_diff_eq!(100000000.0f32, 100000001.0f32);
        assert_abs_diff_eq!(100000001.0f32, 100000000.0f32);
        assert_abs_diff_ne!(10000.0f32, 10001.0f32);
        assert_abs_diff_ne!(10001.0f32, 10000.0f32);
    }

    #[test]
    fn test_big_neg() {
        assert_abs_diff_eq!(-100000000.0f32, -100000001.0f32);
        assert_abs_diff_eq!(-100000001.0f32, -100000000.0f32);
        assert_abs_diff_ne!(-10000.0f32, -10001.0f32);
        assert_abs_diff_ne!(-10001.0f32, -10000.0f32);
    }

    #[test]
    fn test_mid() {
        assert_abs_diff_eq!(1.0000001f32, 1.0000002f32);
        assert_abs_diff_eq!(1.0000002f32, 1.0000001f32);
        assert_abs_diff_ne!(1.000001f32, 1.000002f32);
        assert_abs_diff_ne!(1.000002f32, 1.000001f32);
    }

    #[test]
    fn test_mid_neg() {
        assert_abs_diff_eq!(-1.0000001f32, -1.0000002f32);
        assert_abs_diff_eq!(-1.0000002f32, -1.0000001f32);
        assert_abs_diff_ne!(-1.000001f32, -1.000002f32);
        assert_abs_diff_ne!(-1.000002f32, -1.000001f32);
    }

    #[test]
    fn test_small() {
        assert_abs_diff_eq!(0.000010001f32, 0.000010002f32);
        assert_abs_diff_eq!(0.000010002f32, 0.000010001f32);
        assert_abs_diff_ne!(0.000001002f32, 0.0000001001f32);
        assert_abs_diff_ne!(0.000001001f32, 0.0000001002f32);
    }

    #[test]
    fn test_small_neg() {
        assert_abs_diff_eq!(-0.000010001f32, -0.000010002f32);
        assert_abs_diff_eq!(-0.000010002f32, -0.000010001f32);
        assert_abs_diff_ne!(-0.000001002f32, -0.0000001001f32);
        assert_abs_diff_ne!(-0.000001001f32, -0.0000001002f32);
    }

    #[test]
    fn test_zero() {
        assert_abs_diff_eq!(0.0f32, 0.0f32);
        assert_abs_diff_eq!(0.0f32, -0.0f32);
        assert_abs_diff_eq!(-0.0f32, -0.0f32);

        assert_abs_diff_ne!(0.000001f32, 0.0f32);
        assert_abs_diff_ne!(0.0f32, 0.000001f32);
        assert_abs_diff_ne!(-0.000001f32, 0.0f32);
        assert_abs_diff_ne!(0.0f32, -0.000001f32);
    }

    #[test]
    fn test_default_epsilon() {
        assert_abs_diff_eq!(1.0f32, 1.0f32 + EPSILON);
        assert_abs_diff_ne!(1.0f32, 1.0f32 + EPSILON + EPSILON);
        assert_abs_diff_eq!(
            1.0f32,
            1.0f32 + EPSILON + EPSILON,
            epsilon = EPSILON + EPSILON
        );
        assert_abs_diff_eq!(1.0f32, 1.0f32 - EPSILON);
        assert_abs_diff_ne!(1.0f32, 1.0f32 - EPSILON - EPSILON);
        assert_abs_diff_eq!(
            1.0f32,
            1.0f32 - EPSILON - EPSILON,
            epsilon = EPSILON + EPSILON
        );
    }

    #[test]
    fn test_epsilon() {
        assert_abs_diff_eq!(0.0f32, 1e-40f32, epsilon = 1e-40f32);
        assert_abs_diff_eq!(1e-40f32, 0.0f32, epsilon = 1e-40f32);
        assert_abs_diff_eq!(0.0f32, -1e-40f32, epsilon = 1e-40f32);
        assert_abs_diff_eq!(-1e-40f32, 0.0f32, epsilon = 1e-40f32);

        assert_abs_diff_ne!(1e-40f32, 0.0f32, epsilon = 1e-41f32);
        assert_abs_diff_ne!(0.0f32, 1e-40f32, epsilon = 1e-41f32);
        assert_abs_diff_ne!(-1e-40f32, 0.0f32, epsilon = 1e-41f32);
        assert_abs_diff_ne!(0.0f32, -1e-40f32, epsilon = 1e-41f32);
    }

    #[test]
    fn test_max() {
        assert_abs_diff_eq!(f32::MAX, f32::MAX);
        assert_abs_diff_ne!(f32::MAX, -f32::MAX);
        assert_abs_diff_ne!(-f32::MAX, f32::MAX);
        assert_abs_diff_ne!(f32::MAX, f32::MAX / 2.0);
        assert_abs_diff_ne!(f32::MAX, -f32::MAX / 2.0);
        assert_abs_diff_ne!(-f32::MAX, f32::MAX / 2.0);
    }

    // NOTE: abs_diff_eq fails as numbers begin to get very large

    // #[test]
    // fn test_infinity() {
    //     assert_abs_diff_eq!(f32::INFINITY, f32::INFINITY);
    //     assert_abs_diff_eq!(f32::NEG_INFINITY, f32::NEG_INFINITY);
    //     assert_abs_diff_ne!(f32::NEG_INFINITY, f32::INFINITY);
    //     assert_abs_diff_eq!(f32::INFINITY, f32::MAX);
    //     assert_abs_diff_eq!(f32::NEG_INFINITY, -f32::MAX);
    // }

    #[test]
    fn test_nan() {
        assert_abs_diff_ne!(f32::NAN, f32::NAN);

        assert_abs_diff_ne!(f32::NAN, 0.0);
        assert_abs_diff_ne!(-0.0, f32::NAN);
        assert_abs_diff_ne!(f32::NAN, -0.0);
        assert_abs_diff_ne!(0.0, f32::NAN);

        assert_abs_diff_ne!(f32::NAN, f32::INFINITY);
        assert_abs_diff_ne!(f32::INFINITY, f32::NAN);
        assert_abs_diff_ne!(f32::NAN, f32::NEG_INFINITY);
        assert_abs_diff_ne!(f32::NEG_INFINITY, f32::NAN);

        assert_abs_diff_ne!(f32::NAN, f32::MAX);
        assert_abs_diff_ne!(f32::MAX, f32::NAN);
        assert_abs_diff_ne!(f32::NAN, -f32::MAX);
        assert_abs_diff_ne!(-f32::MAX, f32::NAN);

        assert_abs_diff_ne!(f32::NAN, f32::MIN_POSITIVE);
        assert_abs_diff_ne!(f32::MIN_POSITIVE, f32::NAN);
        assert_abs_diff_ne!(f32::NAN, -f32::MIN_POSITIVE);
        assert_abs_diff_ne!(-f32::MIN_POSITIVE, f32::NAN);
    }

    #[test]
    fn test_opposite_signs() {
        assert_abs_diff_ne!(1.000000001f32, -1.0f32);
        assert_abs_diff_ne!(-1.0f32, 1.000000001f32);
        assert_abs_diff_ne!(-1.000000001f32, 1.0f32);
        assert_abs_diff_ne!(1.0f32, -1.000000001f32);

        assert_abs_diff_eq!(10.0 * f32::MIN_POSITIVE, 10.0 * -f32::MIN_POSITIVE);
    }

    #[test]
    fn test_close_to_zero() {
        assert_abs_diff_eq!(f32::MIN_POSITIVE, f32::MIN_POSITIVE);
        assert_abs_diff_eq!(f32::MIN_POSITIVE, -f32::MIN_POSITIVE);
        assert_abs_diff_eq!(-f32::MIN_POSITIVE, f32::MIN_POSITIVE);

        assert_abs_diff_eq!(f32::MIN_POSITIVE, 0.0f32);
        assert_abs_diff_eq!(0.0f32, f32::MIN_POSITIVE);
        assert_abs_diff_eq!(-f32::MIN_POSITIVE, 0.0f32);
        assert_abs_diff_eq!(0.0f32, -f32::MIN_POSITIVE);

        assert_abs_diff_ne!(0.000001f32, -f32::MIN_POSITIVE);
        assert_abs_diff_ne!(0.000001f32, f32::MIN_POSITIVE);
        assert_abs_diff_ne!(f32::MIN_POSITIVE, 0.000001f32);
        assert_abs_diff_ne!(-f32::MIN_POSITIVE, 0.000001f32);
    }
}

#[cfg(test)]
mod test_f64 {
    use core::f64;

    static EPSILON: f64 = f64::EPSILON;

    #[test]
    fn test_basic() {
        assert_abs_diff_eq!(1.0f64, 1.0f64);
        assert_abs_diff_ne!(1.0f64, 2.0f64);
    }

    #[test]
    #[should_panic]
    fn test_basic_panic_eq() {
        assert_abs_diff_eq!(1.0f64, 2.0f64);
    }

    #[test]
    #[should_panic]
    fn test_basic_panic_ne() {
        assert_abs_diff_ne!(1.0f64, 1.0f64);
    }

    #[test]
    fn test_big() {
        assert_abs_diff_eq!(10000000000000000.0f64, 10000000000000001.0f64);
        assert_abs_diff_eq!(10000000000000001.0f64, 10000000000000000.0f64);
        assert_abs_diff_ne!(1000000000000000.0f64, 1000000000000001.0f64);
        assert_abs_diff_ne!(1000000000000001.0f64, 1000000000000000.0f64);
    }

    #[test]
    fn test_big_neg() {
        assert_abs_diff_eq!(-10000000000000000.0f64, -10000000000000001.0f64);
        assert_abs_diff_eq!(-10000000000000001.0f64, -10000000000000000.0f64);
        assert_abs_diff_ne!(-1000000000000000.0f64, -1000000000000001.0f64);
        assert_abs_diff_ne!(-1000000000000001.0f64, -1000000000000000.0f64);
    }

    #[test]
    fn test_mid() {
        assert_abs_diff_eq!(1.0000000000000001f64, 1.0000000000000002f64);
        assert_abs_diff_eq!(1.0000000000000002f64, 1.0000000000000001f64);
        assert_abs_diff_ne!(1.000000000000001f64, 1.000000000000002f64);
        assert_abs_diff_ne!(1.000000000000002f64, 1.000000000000001f64);
    }

    #[test]
    fn test_mid_neg() {
        assert_abs_diff_eq!(-1.0000000000000001f64, -1.0000000000000002f64);
        assert_abs_diff_eq!(-1.0000000000000002f64, -1.0000000000000001f64);
        assert_abs_diff_ne!(-1.000000000000001f64, -1.000000000000002f64);
        assert_abs_diff_ne!(-1.000000000000002f64, -1.000000000000001f64);
    }

    #[test]
    fn test_small() {
        assert_abs_diff_eq!(0.0000000100000001f64, 0.0000000100000002f64);
        assert_abs_diff_eq!(0.0000000100000002f64, 0.0000000100000001f64);
        assert_abs_diff_ne!(0.0000000100000001f64, 0.0000000010000002f64);
        assert_abs_diff_ne!(0.0000000100000002f64, 0.0000000010000001f64);
    }

    #[test]
    fn test_small_neg() {
        assert_abs_diff_eq!(-0.0000000100000001f64, -0.0000000100000002f64);
        assert_abs_diff_eq!(-0.0000000100000002f64, -0.0000000100000001f64);
        assert_abs_diff_ne!(-0.0000000100000001f64, -0.0000000010000002f64);
        assert_abs_diff_ne!(-0.0000000100000002f64, -0.0000000010000001f64);
    }

    #[test]
    fn test_zero() {
        assert_abs_diff_eq!(0.0f64, 0.0f64);
        assert_abs_diff_eq!(0.0f64, -0.0f64);
        assert_abs_diff_eq!(-0.0f64, -0.0f64);

        assert_abs_diff_ne!(0.000000000000001f64, 0.0f64);
        assert_abs_diff_ne!(0.0f64, 0.000000000000001f64);
        assert_abs_diff_ne!(-0.000000000000001f64, 0.0f64);
        assert_abs_diff_ne!(0.0f64, -0.000000000000001f64);
    }

    #[test]
    fn test_epsilon() {
        assert_abs_diff_eq!(0.0f64, 1e-40f64, epsilon = 1e-40f64);
        assert_abs_diff_eq!(1e-40f64, 0.0f64, epsilon = 1e-40f64);
        assert_abs_diff_eq!(0.0f64, -1e-40f64, epsilon = 1e-40f64);
        assert_abs_diff_eq!(-1e-40f64, 0.0f64, epsilon = 1e-40f64);

        assert_abs_diff_ne!(1e-40f64, 0.0f64, epsilon = 1e-41f64);
        assert_abs_diff_ne!(0.0f64, 1e-40f64, epsilon = 1e-41f64);
        assert_abs_diff_ne!(-1e-40f64, 0.0f64, epsilon = 1e-41f64);
        assert_abs_diff_ne!(0.0f64, -1e-40f64, epsilon = 1e-41f64);
    }

    #[test]
    fn test_default_epsilon() {
        assert_abs_diff_eq!(1.0, 1.0 + EPSILON);
        assert_abs_diff_ne!(1.0, 1.0 + EPSILON + EPSILON);
        assert_abs_diff_eq!(1.0, 1.0 + EPSILON + EPSILON, epsilon = EPSILON + EPSILON);
        assert_abs_diff_eq!(1.0, 1.0 - EPSILON);
        assert_abs_diff_ne!(1.0, 1.0 - EPSILON - EPSILON);
        assert_abs_diff_eq!(1.0, 1.0 - EPSILON - EPSILON, epsilon = EPSILON + EPSILON);
    }

    #[test]
    fn test_max() {
        assert_abs_diff_eq!(f64::MAX, f64::MAX);
        assert_abs_diff_ne!(f64::MAX, -f64::MAX);
        assert_abs_diff_ne!(-f64::MAX, f64::MAX);
        assert_abs_diff_ne!(f64::MAX, f64::MAX / 2.0);
        assert_abs_diff_ne!(f64::MAX, -f64::MAX / 2.0);
        assert_abs_diff_ne!(-f64::MAX, f64::MAX / 2.0);
    }

    // NOTE: abs_diff_eq fails as numbers begin to get very large

    // #[test]
    // fn test_infinity() {
    //     assert_abs_diff_eq!(f64::INFINITY, f64::INFINITY);
    //     assert_abs_diff_eq!(f64::NEG_INFINITY, f64::NEG_INFINITY);
    //     assert_abs_diff_ne!(f64::NEG_INFINITY, f64::INFINITY);
    //     assert_abs_diff_eq!(f64::INFINITY, f64::MAX);
    //     assert_abs_diff_eq!(f64::NEG_INFINITY, -f64::MAX);
    // }

    #[test]
    fn test_nan() {
        assert_abs_diff_ne!(f64::NAN, f64::NAN);

        assert_abs_diff_ne!(f64::NAN, 0.0);
        assert_abs_diff_ne!(-0.0, f64::NAN);
        assert_abs_diff_ne!(f64::NAN, -0.0);
        assert_abs_diff_ne!(0.0, f64::NAN);

        assert_abs_diff_ne!(f64::NAN, f64::INFINITY);
        assert_abs_diff_ne!(f64::INFINITY, f64::NAN);
        assert_abs_diff_ne!(f64::NAN, f64::NEG_INFINITY);
        assert_abs_diff_ne!(f64::NEG_INFINITY, f64::NAN);

        assert_abs_diff_ne!(f64::NAN, f64::MAX);
        assert_abs_diff_ne!(f64::MAX, f64::NAN);
        assert_abs_diff_ne!(f64::NAN, -f64::MAX);
        assert_abs_diff_ne!(-f64::MAX, f64::NAN);

        assert_abs_diff_ne!(f64::NAN, f64::MIN_POSITIVE);
        assert_abs_diff_ne!(f64::MIN_POSITIVE, f64::NAN);
        assert_abs_diff_ne!(f64::NAN, -f64::MIN_POSITIVE);
        assert_abs_diff_ne!(-f64::MIN_POSITIVE, f64::NAN);
    }

    #[test]
    fn test_opposite_signs() {
        assert_abs_diff_ne!(1.000000001f64, -1.0f64);
        assert_abs_diff_ne!(-1.0f64, 1.000000001f64);
        assert_abs_diff_ne!(-1.000000001f64, 1.0f64);
        assert_abs_diff_ne!(1.0f64, -1.000000001f64);

        assert_abs_diff_eq!(10.0 * f64::MIN_POSITIVE, 10.0 * -f64::MIN_POSITIVE);
    }

    #[test]
    fn test_close_to_zero() {
        assert_abs_diff_eq!(f64::MIN_POSITIVE, f64::MIN_POSITIVE);
        assert_abs_diff_eq!(f64::MIN_POSITIVE, -f64::MIN_POSITIVE);
        assert_abs_diff_eq!(-f64::MIN_POSITIVE, f64::MIN_POSITIVE);

        assert_abs_diff_eq!(f64::MIN_POSITIVE, 0.0f64);
        assert_abs_diff_eq!(0.0f64, f64::MIN_POSITIVE);
        assert_abs_diff_eq!(-f64::MIN_POSITIVE, 0.0f64);
        assert_abs_diff_eq!(0.0f64, -f64::MIN_POSITIVE);

        assert_abs_diff_ne!(0.000000000000001f64, -f64::MIN_POSITIVE);
        assert_abs_diff_ne!(0.000000000000001f64, f64::MIN_POSITIVE);
        assert_abs_diff_ne!(f64::MIN_POSITIVE, 0.000000000000001f64);
        assert_abs_diff_ne!(-f64::MIN_POSITIVE, 0.000000000000001f64);
    }
}

mod test_option {
    mod test_f32 {
        #[test]
        fn test_basic() {
            assert_abs_diff_eq!(Some(1.0f32), Some(1.0f32));

            assert_abs_diff_ne!(Some(1.0f32), Some(2.0f32));
            assert_abs_diff_ne!(Some(1.0f32), None);
        }
    }

    mod test_f64 {
        #[test]
        fn test_basic() {
            assert_abs_diff_eq!(Some(1.0f64), Some(1.0f64));

            assert_abs_diff_ne!(Some(1.0f64), Some(2.0f64));
            assert_abs_diff_ne!(Some(1.0f64), None);
        }
    }
}

mod test_result {
    mod test_f32 {
        #[test]
        fn test_basic() {
            assert_abs_diff_eq!(Ok::<f32, f32>(1.0f32), Ok(1.0f32));
            assert_abs_diff_eq!(Err::<f32, f32>(1.0f32), Err(1.0f32));

            assert_abs_diff_ne!(Ok::<f32, f32>(1.0f32), Ok(2.0f32));
            assert_abs_diff_ne!(Ok::<f32, f32>(1.0f32), Err(1.0f32));
            assert_abs_diff_ne!(Err::<f32, f32>(1.0f32), Err(2.0f32));
        }
    }

    mod test_f64 {
        #[test]
        fn test_basic() {
            assert_abs_diff_eq!(Ok::<f64, f64>(1.0f64), Ok(1.0f64));
            assert_abs_diff_eq!(Err::<f64, f64>(1.0f64), Err(1.0f64));

            assert_abs_diff_ne!(Ok::<f64, f64>(1.0f64), Ok(2.0f64));
            assert_abs_diff_ne!(Ok::<f64, f64>(1.0f64), Err(1.0f64));
            assert_abs_diff_ne!(Err::<f64, f64>(1.0f64), Err(2.0f64));
        }
    }
}

mod test_ref {
    mod test_f32 {
        #[test]
        fn test_basic() {
            assert_abs_diff_eq!(&1.0f32, &1.0f32);
            assert_abs_diff_ne!(&1.0f32, &2.0f32);
        }
    }

    mod test_f64 {
        #[test]
        fn test_basic() {
            assert_abs_diff_eq!(&1.0f64, &1.0f64);
            assert_abs_diff_ne!(&1.0f64, &2.0f64);
        }
    }
}

mod test_slice {
    mod test_f32 {
        #[test]
        fn test_basic() {
            assert_abs_diff_eq!([1.0f32, 2.0f32][..], [1.0f32, 2.0f32][..]);
            assert_abs_diff_ne!([1.0f32, 2.0f32][..], [2.0f32, 1.0f32][..]);
        }
    }

    mod test_f64 {
        #[test]
        fn test_basic() {
            assert_abs_diff_eq!([1.0f64, 2.0f64][..], [1.0f64, 2.0f64][..]);
            assert_abs_diff_ne!([1.0f64, 2.0f64][..], [2.0f64, 1.0f64][..]);
        }
    }
}

#[cfg(feature = "array_impl")]
mod test_array {
    mod test_f32 {
        #[test]
        fn test_basic() {
            assert_abs_diff_eq!([1.0f32, 2.0f32], [1.0f32, 2.0f32]);
            assert_abs_diff_ne!([1.0f32, 2.0f32], [2.0f32, 1.0f32]);
        }
    }

    mod test_f64 {
        #[test]
        fn test_basic() {
            assert_abs_diff_eq!([1.0f64, 2.0f64], [1.0f64, 2.0f64]);
            assert_abs_diff_ne!([1.0f64, 2.0f64], [2.0f64, 1.0f64]);
        }
    }
}

#[cfg(feature = "tuple_impl")]
mod test_tuple{
    use approxim::AbsDiffEq;

    #[test]
    fn test_basic() {
        ().abs_diff_eq(&(), ());
        ((),).abs_diff_eq(&((),), ((),));
        ((), (),).abs_diff_eq(&((), (),), ((), (),));
        ((), (), (),).abs_diff_eq(&((), (), (),), ((), (), (),));
        ((), (), (), (),).abs_diff_eq(&((), (), (), (),), ((), (), (), (),));
        ((), (), (), (), (),).abs_diff_eq(&((), (), (), (), (),), ((), (), (), (), (),));
        ((), (), (), (), (), (),).abs_diff_eq(&((), (), (), (), (), (),), ((), (), (), (), (), (),));
        ((), (), (), (), (), (), (),).abs_diff_eq(&((), (), (), (), (), (), (),), ((), (), (), (), (), (), (),));
        ((), (), (), (), (), (), (), (),).abs_diff_eq(&((), (), (), (), (), (), (), (),), ((), (), (), (), (), (), (), (),));
        ((), (), (), (), (), (), (), (), (),).abs_diff_eq(&((), (), (), (), (), (), (), (), (),), ((), (), (), (), (), (), (), (), (),));
        ((), (), (), (), (), (), (), (), (), (),).abs_diff_eq(&((), (), (), (), (), (), (), (), (), (),), ((), (), (), (), (), (), (), (), (), (),));
        ((), (), (), (), (), (), (), (), (), (), (),).abs_diff_eq(&((), (), (), (), (), (), (), (), (), (), (),), ((), (), (), (), (), (), (), (), (), (), (),));
        ((), (), (), (), (), (), (), (), (), (), (), (),).abs_diff_eq(&((), (), (), (), (), (), (), (), (), (), (), (),), ((), (), (), (), (), (), (), (), (), (), (), (),));
    }
}

#[cfg(feature = "num-complex")]
mod test_complex {
    extern crate num_complex;
    pub use self::num_complex::Complex;

    mod test_f32 {
        use super::Complex;

        #[test]
        fn test_basic() {
            assert_abs_diff_eq!(Complex::new(1.0f32, 2.0f32), Complex::new(1.0f32, 2.0f32));
            assert_abs_diff_ne!(Complex::new(1.0f32, 2.0f32), Complex::new(2.0f32, 1.0f32));
        }

        #[test]
        #[should_panic]
        fn test_basic_panic_eq() {
            assert_abs_diff_eq!(Complex::new(1.0f32, 2.0f32), Complex::new(2.0f32, 1.0f32));
        }

        #[test]
        #[should_panic]
        fn test_basic_panic_ne() {
            assert_abs_diff_ne!(Complex::new(1.0f32, 2.0f32), Complex::new(1.0f32, 2.0f32));
        }
    }

    mod test_f64 {
        use super::Complex;

        #[test]
        fn test_basic() {
            assert_abs_diff_eq!(Complex::new(1.0f64, 2.0f64), Complex::new(1.0f64, 2.0f64));
            assert_abs_diff_ne!(Complex::new(1.0f64, 2.0f64), Complex::new(2.0f64, 1.0f64));
        }

        #[test]
        #[should_panic]
        fn test_basic_panic_eq() {
            assert_abs_diff_eq!(Complex::new(1.0f64, 2.0f64), Complex::new(2.0f64, 1.0f64));
        }

        #[test]
        #[should_panic]
        fn test_basic_panic_ne() {
            assert_abs_diff_ne!(Complex::new(1.0f64, 2.0f64), Complex::new(1.0f64, 2.0f64));
        }
    }
}

#[cfg(feature = "ordered-float")]
mod test_ordered_float {
    extern crate ordered_float;
    pub use self::ordered_float::OrderedFloat;

    mod test_f32 {
        use super::OrderedFloat;

        #[test]
        fn test_basic() {
            assert_abs_diff_eq!(OrderedFloat(1.0f32), OrderedFloat(1.0f32));
            assert_abs_diff_ne!(OrderedFloat(1.0f32), OrderedFloat(2.0f32));
            assert_abs_diff_eq!(OrderedFloat(1.0f32), 1.0f32);
            assert_abs_diff_ne!(OrderedFloat(1.0f32), 2.0f32);
            assert_abs_diff_eq!(1.0f32, OrderedFloat(1.0f32));
            assert_abs_diff_ne!(1.0f32, OrderedFloat(2.0f32));
        }

        #[test]
        #[should_panic]
        fn test_basic_panic_eq() {
            assert_abs_diff_eq!(OrderedFloat(1.0f32), OrderedFloat(2.0f32));
        }

        #[test]
        #[should_panic]
        fn test_basic_panic_ne() {
            assert_abs_diff_ne!(OrderedFloat(1.0f32), OrderedFloat(1.0f32));
        }
    }

    mod test_f64 {
        use super::OrderedFloat;

        #[test]
        fn test_basic() {
            assert_abs_diff_eq!(OrderedFloat(1.0f64), OrderedFloat(1.0f64));
            assert_abs_diff_ne!(OrderedFloat(1.0f64), OrderedFloat(2.0f64));
            assert_abs_diff_eq!(OrderedFloat(1.0f64), 1.0f64);
            assert_abs_diff_ne!(OrderedFloat(1.0f64), 2.0f64);
            assert_abs_diff_eq!(1.0f64, OrderedFloat(1.0f64));
            assert_abs_diff_ne!(1.0f64, OrderedFloat(2.0f64));
        }

        #[test]
        #[should_panic]
        fn test_basic_panic_eq() {
            assert_abs_diff_eq!(OrderedFloat(1.0f64), OrderedFloat(2.0f64));
        }

        #[test]
        #[should_panic]
        fn test_basic_panic_ne() {
            assert_abs_diff_ne!(OrderedFloat(1.0f64), OrderedFloat(1.0f64));
        }
    }
}
