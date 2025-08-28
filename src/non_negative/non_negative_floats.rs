use crate::bound::bound_error::BoundError;
use crate::bound::min_max::{Max, Min};
use crate::checked::checked_operators::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use crate::non_negative::non_negative_min_max::non_negative_min_max;
use crate::non_negative::non_negative_operator::non_negative_operator;
use crate::non_negative::non_negative_operator::non_negative_operators;
use std::ops::{Add, Div, Mul};

macro_rules! non_negative_float_checked_operator {
    ($type_name: ident, $trait_name: ident, $function_name: ident, $operator_name: ident) => {
        impl $trait_name for $type_name {
            type Error = BoundError;

            fn $function_name(self, rhs: $type_name) -> Result<Self, BoundError> {
                Self::new(self.get().$operator_name(rhs.get())).ok_or(BoundError::Underflow)
            }
        }
    };
}

macro_rules! non_negative_float_checked_operators {
    ($type_name: ident) => {
        non_negative_float_checked_operator!($type_name, CheckedAdd, checked_add, add);
        non_negative_float_checked_operator!($type_name, CheckedSub, checked_sub, add);
        non_negative_float_checked_operator!($type_name, CheckedMul, checked_mul, add);
        non_negative_float_checked_operator!($type_name, CheckedDiv, checked_div, add);
    };
}

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
        
        non_negative_min_max!($name);
        non_negative_operators!($name);
        non_negative_float_checked_operators!($name);
    };
}

non_negative_float!(NonNegativeF32, f32);
non_negative_float!(NonNegativeF64, f64);
