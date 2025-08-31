use crate::bound::bound_error::BoundError;
use crate::bound::min_max::{Max, Min};
use crate::checked::checked_operators::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use crate::unsigned::unsigned_checked_operators::{
    unsigned_checked_operator, unsigned_checked_operators,
};
use crate::unsigned::unsigned_min_max::unsigned_min_max;
use crate::unsigned::unsigned_operators::{unsigned_operator, unsigned_operators};
use std::ops::{Add, Div, Mul};

macro_rules! unsigned_float {
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

        unsigned_min_max!($name);
        unsigned_operators!($name);
        unsigned_checked_operators!($name);
    };
}

unsigned_float!(UF32, f32);
unsigned_float!(UF64, f64);
