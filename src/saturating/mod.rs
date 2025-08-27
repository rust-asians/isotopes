pub mod saturating_operators;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug, Default)]
pub struct Saturating<T>(T);

impl<T> Saturating<T> {
    pub fn new(checked: T) -> Self {
        Self(checked)
    }

    pub fn get(self) -> T {
        self.0
    }
}

pub trait IntoSaturating<T> {
    fn into_saturating(self) -> Saturating<T>;
}

impl<T> IntoSaturating<T> for T {
    fn into_saturating(self) -> Saturating<T> {
        Saturating::new(self)
    }
}

impl<T> From<T> for Saturating<T> {
    fn from(value: T) -> Self {
        value.into_saturating()
    }
}
