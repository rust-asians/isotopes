use crate::bound::bound_error::BoundError;
use crate::bound::bound_error::BoundError::{Overflow, Underflow};
use crate::bound::min_max::Min;
use crate::checked::checked_operators::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

macro_rules! primitive_checked_operator {
    (
        $type_name: ty,
        $trait_name: ident,
        $function_name: ident,
        $checked_operator_name: ident,
        $saturating_operator_name: ident$(,)?
    ) => {
        impl $trait_name for $type_name {
            type Error = BoundError;

            fn $function_name(self, rhs: Self) -> Result<Self, Self::Error> {
                let error = || {
                    if self.$saturating_operator_name(rhs) == Min::min() {
                        Underflow
                    } else {
                        Overflow
                    }
                };

                self.$checked_operator_name(rhs).ok_or_else(error)
            }
        }
    };
}

macro_rules! primitive_checked_operators {
    ($type_name: ty) => {
        primitive_checked_operator!(
            $type_name,
            CheckedAdd,
            checked_add,
            checked_add,
            saturating_add,
        );
        primitive_checked_operator!(
            $type_name,
            CheckedSub,
            checked_sub,
            checked_sub,
            saturating_sub,
        );
        primitive_checked_operator!(
            $type_name,
            CheckedMul,
            checked_mul,
            checked_mul,
            saturating_mul,
        );
        primitive_checked_operator!(
            $type_name,
            CheckedDiv,
            checked_div,
            checked_div,
            saturating_div,
        );
    };
}

primitive_checked_operators!(u8);
primitive_checked_operators!(u16);
primitive_checked_operators!(u32);
primitive_checked_operators!(u64);
primitive_checked_operators!(u128);
primitive_checked_operators!(usize);

primitive_checked_operators!(i8);
primitive_checked_operators!(i16);
primitive_checked_operators!(i32);
primitive_checked_operators!(i64);
primitive_checked_operators!(i128);
primitive_checked_operators!(isize);
