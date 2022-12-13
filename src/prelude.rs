pub use crate::error::WunderError;

pub type Result<T> = std::result::Result<T, WunderError>;

pub struct W<T>(pub T);

impl<T> std::ops::Deref for W<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for W<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}