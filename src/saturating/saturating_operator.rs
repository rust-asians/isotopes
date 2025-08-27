use crate::bound::bound_error::BoundError;
use crate::bound::min_max::Max;
use crate::bound::min_max::Min;
use crate::checked::checked_operator::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use std::ops::{Add, Div, Mul, Sub};

macro_rules! saturating_operator {
    {
        $struct_name: ident,
        $trait_name: ident,
        $function_name: ident,
        $checked_struct_name: ident,
        $checked_function_name: ident
    } => {
        pub struct $struct_name<T>(T)
        where
            T: $checked_struct_name<Error = BoundError> + Min + Max;

        impl<T> $struct_name<T>
        where
            T: $checked_struct_name<Error = BoundError> + Min + Max,
        {
            pub fn new(checked: T) -> Self {
                $struct_name(checked)
            }

            pub fn get(self) -> T {
                self.0
            }
        }

        impl<T, Rhs> $trait_name<Rhs> for $struct_name<T>
        where
            T: $checked_struct_name<Error = BoundError> + Min + Max,
            Rhs: Into<$struct_name<T>>,
        {
            type Output = Self;

            fn $function_name(self, rhs: Rhs) -> Self::Output {
                let checked = self
                    .get()
                    .$checked_function_name(rhs.into().get())
                    .unwrap_or_else(saturate);
                Self::new(checked)
            }
        }

        impl<T> From<T> for $struct_name<T>
        where
            T: $checked_struct_name<Error = BoundError> + Min + Max,
        {
            fn from(value: T) -> Self {
                Self::new(value)
            }
        }

        // TODO: Idk how to make this work
        // impl<T> From<$struct_name<T>> for T
        // where
        //     T: $checked_struct_name<Error = BoundError> + Min + Max,
        // {
        //     fn from(value: $struct_name<T>) -> Self {
        //         value.get()
        //     }
        // }
    };
}

saturating_operator!(SaturatingAdd, Add, add, CheckedAdd, checked_add);
saturating_operator!(SaturatingSub, Sub, sub, CheckedSub, checked_sub);
saturating_operator!(SaturatingMul, Mul, mul, CheckedMul, checked_mul);
saturating_operator!(SaturatingDiv, Div, div, CheckedDiv, checked_div);

fn saturate<T>(bound_error: BoundError) -> T
where
    T: Min + Max,
{
    match bound_error {
        BoundError::Underflow => T::min(),
        BoundError::Overflow => T::max(),
    }
}
