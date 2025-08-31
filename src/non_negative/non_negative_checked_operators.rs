macro_rules! non_negative_checked_operator {
    ($type_name: ident, $trait_name: ident, $function_name: ident, $operator_name: ident) => {
        impl $trait_name for $type_name {
            type Error = BoundError;

            fn $function_name(self, rhs: $type_name) -> Result<Self, BoundError> {
                Self::new(self.get().$operator_name(rhs.get())).ok_or(BoundError::Underflow)
            }
        }
    };
}

macro_rules! non_negative_checked_operators {
    ($type_name: ident) => {
        non_negative_checked_operator!($type_name, CheckedAdd, checked_add, add);
        non_negative_checked_operator!($type_name, CheckedSub, checked_sub, add);
        non_negative_checked_operator!($type_name, CheckedMul, checked_mul, add);
        non_negative_checked_operator!($type_name, CheckedDiv, checked_div, add);
    };
}

pub(super) use non_negative_checked_operator;
pub(super) use non_negative_checked_operators;
