macro_rules! non_negative_operator {
    ($type_name: ident, $trait_name: ident, $function_name: ident) => {
        impl $trait_name for $type_name {
            type Output = $type_name;

            fn $function_name(self, rhs: Self) -> Self::Output {
                unsafe { Self::new_unchecked(self.get().$function_name(rhs.get())) }
            }
        }
    };
}

macro_rules! non_negative_operators {
    ($type_name: ident) => {
        non_negative_operator!($type_name, Add, add);
        non_negative_operator!($type_name, Mul, mul);
        non_negative_operator!($type_name, Div, div);
    };
}

pub(super) use non_negative_operator;
pub(super) use non_negative_operators;
