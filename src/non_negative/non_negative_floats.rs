use crate::non_negative::non_negative_operator::non_negative_operator;
use std::ops::{Add, Div, Mul};

macro_rules! non_negative_float {
    ($name: ident, $inner_type: ty) => {
        #[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
        pub struct $name($inner_type);

        impl $name {
            /// # Safety
            ///
            /// The caller must guarantee that `value` is not `< 0`.
            pub unsafe fn new_unchecked(value: $inner_type) -> Self {
                Self(value)
            }

            pub fn new(value: $inner_type) -> Option<Self> {
                match value {
                    0.0.. => Some(unsafe { Self::new_unchecked(value) }),
                    _ => None,
                }
            }

            pub fn zero() -> Self {
                unsafe { Self::new_unchecked(0.0) }
            }

            pub fn one() -> Self {
                unsafe { Self::new_unchecked(1.0) }
            }

            pub fn get(self) -> $inner_type {
                self.0
            }
        }

        non_negative_operator!($name, Add, add);
        non_negative_operator!($name, Mul, mul);
        non_negative_operator!($name, Div, div);
    };
}

non_negative_float!(NonNegativeF32, f32);
non_negative_float!(NonNegativeF64, f64);
