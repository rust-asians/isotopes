use crate::non_negative::non_negative_min_max::non_negative_min_max;
use crate::bound::bound_error::BoundError;
use crate::bound::min_max::{Max, Min};
use crate::checked::checked_operators::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use crate::non_negative::non_negative_operator::{non_negative_operator, non_negative_operators};
use std::ops::{Add, Div, Mul};

macro_rules! non_negative_integer_checked_operator {
    ($type_name: ident, $trait_name: ident, $function_name: ident) => {
        impl $trait_name for $type_name {
            type Error = BoundError;

            fn $function_name(self, rhs: $type_name) -> Result<Self, BoundError> {
                Self::new($trait_name::$function_name(self.get(), rhs.get())?)
                    .ok_or(BoundError::Underflow)
            }
        }
    };
}

macro_rules! non_negative_integer_checked_operators {
    ($type_name: ident) => {
        non_negative_integer_checked_operator!($type_name, CheckedAdd, checked_add);
        non_negative_integer_checked_operator!($type_name, CheckedSub, checked_sub);
        non_negative_integer_checked_operator!($type_name, CheckedMul, checked_mul);
        non_negative_integer_checked_operator!($type_name, CheckedDiv, checked_div);
    };
}

macro_rules! non_negative_integer {
    ($name: ident, $inner_type: ty) => {
        #[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
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
                    0.. => Some(unsafe { Self::new_unchecked(value) }),
                    _ => None,
                }
            }

            pub fn zero() -> Self {
                unsafe { Self::new_unchecked(0) }
            }

            pub fn one() -> Self {
                unsafe { Self::new_unchecked(1) }
            }

            pub fn get(self) -> $inner_type {
                self.0
            }
        }

        non_negative_min_max!($name);
        non_negative_operators!($name);
        non_negative_integer_checked_operators!($name);
    };
}

non_negative_integer!(NonNegativeI8, i8);
non_negative_integer!(NonNegativeI16, i16);
non_negative_integer!(NonNegativeI32, i32);
non_negative_integer!(NonNegativeI64, i64);
non_negative_integer!(NonNegativeI128, i128);
non_negative_integer!(NonNegativeIsize, isize);
