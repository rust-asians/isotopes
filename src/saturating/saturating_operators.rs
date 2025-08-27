use crate::bound::bound_error::BoundError;
use crate::bound::min_max::Max;
use crate::bound::min_max::Min;
use crate::checked::checked_operators::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use crate::saturating::Saturating;
use std::ops::{Add, Div, Mul, Sub};

macro_rules! saturating_operator {
    {
        $trait_name: ident,
        $function_name: ident,
        $checked_name: ident,
        $checked_function_name: ident
    } => {
        impl<T, Rhs> $trait_name<Rhs> for Saturating<T>
        where
            T: $checked_name<Error = BoundError> + Min + Max,
            Rhs: Into<Saturating<T>>,
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
    };
}

saturating_operator!(Add, add, CheckedAdd, checked_add);
saturating_operator!(Sub, sub, CheckedSub, checked_sub);
saturating_operator!(Mul, mul, CheckedMul, checked_mul);
saturating_operator!(Div, div, CheckedDiv, checked_div);

fn saturate<T>(bound_error: BoundError) -> T
where
    T: Min + Max,
{
    match bound_error {
        BoundError::Underflow => T::min(),
        BoundError::Overflow => T::max(),
    }
}
