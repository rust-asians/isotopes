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

pub(super) use non_negative_operator;
