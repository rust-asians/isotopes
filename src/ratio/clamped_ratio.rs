use crate::bound::bound_error::BoundError;
use crate::bound::min_max::{Max, Min};
use crate::checked::checked_operators::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use std::ops::{Add, Div, Mul, Sub};

macro_rules! clamped_ratio_checked_operator {
    ($type_name: ident, $trait_name: ident, $function_name: ident, $operator_name: ident) => {
        impl $trait_name for $type_name {
            type Error = BoundError;

            fn $function_name(self, rhs: $type_name) -> Result<Self, BoundError> {
                Self::new(self.get().$operator_name(rhs.get()))
            }
        }
    };
}

macro_rules! clamped_ratio {
    ($type_name: ident, $inner_type: ty) => {
        /// A floating-point value inside [0, 1].
        #[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Default)]
        pub struct $type_name($inner_type);

        impl $type_name {
            /// # Safety
            ///
            /// The caller must guarantee that `value` is inside `[0, 1]`.
            pub unsafe fn new_unchecked(value: $inner_type) -> Self {
                Self(value)
            }

            pub fn new(value: $inner_type) -> Result<Self, BoundError> {
                match value {
                    0.0..=1.0 => Ok(unsafe { Self::new_unchecked(value) }),
                    ..0.0 => Err(BoundError::Underflow),
                    _ => Err(BoundError::Overflow),
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

        impl Min for $type_name {
            fn min() -> Self {
                Self::zero()
            }
        }

        impl Max for $type_name {
            fn max() -> Self {
                Self::one()
            }
        }

        clamped_ratio_checked_operator!($type_name, CheckedAdd, checked_add, add);
        clamped_ratio_checked_operator!($type_name, CheckedSub, checked_sub, sub);
        clamped_ratio_checked_operator!($type_name, CheckedMul, checked_mul, mul);
        clamped_ratio_checked_operator!($type_name, CheckedDiv, checked_div, div);
    };
}

clamped_ratio!(ClampedRatio32, f32);
clamped_ratio!(ClampedRatio64, f64);
