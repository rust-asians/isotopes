use crate::bound::min_max::Max;
use crate::bound::min_max::Min;

macro_rules! primitive_min {
    ($type_name: ident) => {
        impl Min for $type_name {
            fn min() -> Self {
                $type_name::MIN
            }
        }
    };
}

macro_rules! primitive_max {
    ($type_name: ident) => {
        impl Max for $type_name {
            fn max() -> Self {
                $type_name::MAX
            }
        }
    };
}

macro_rules! primitive_min_max {
    ($type_name: ident) => {
        primitive_min!($type_name);
        primitive_max!($type_name);
    };
}

primitive_min_max!(u8);
primitive_min_max!(u16);
primitive_min_max!(u32);
primitive_min_max!(u64);
primitive_min_max!(u128);
primitive_min_max!(usize);

primitive_min_max!(i8);
primitive_min_max!(i16);
primitive_min_max!(i32);
primitive_min_max!(i64);
primitive_min_max!(i128);
primitive_min_max!(isize);

primitive_min_max!(f32);
primitive_min_max!(f64);
