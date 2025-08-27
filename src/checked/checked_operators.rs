macro_rules! checked_operator {
    ($trait_name: ident, $function_name: ident) => {
        pub trait $trait_name {
            type Error;

            fn $function_name(self, rhs: Self) -> Result<Self, Self::Error>
            where
                Self: Sized;
        }
    };
}

checked_operator!(CheckedAdd, checked_add);
checked_operator!(CheckedSub, checked_sub);
checked_operator!(CheckedMul, checked_mul);
checked_operator!(CheckedDiv, checked_div);
