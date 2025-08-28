macro_rules! non_negative_min_max{
    ($type_name: ident) => {
        impl Min for $type_name {
            fn min() -> Self {
                Self::zero()
            }
        }

        impl Max for $type_name {
            fn max() -> Self {
                unsafe { Self::new_unchecked(Max::max()) }
            }
        }
    };
}

pub(super) use non_negative_min_max;
