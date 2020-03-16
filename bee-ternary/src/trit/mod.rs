pub mod balanced;
pub mod unbalanced;

// Reexports
pub use self::{
    balanced::Btrit,
    unbalanced::Utrit,
};

use std::fmt;

pub trait Trit: Copy + Sized + fmt::Debug + From<i8> + Into<i8> + PartialEq + ToggleTernary {
    // TODO: Use std::convert::TryFrom
    fn try_from(x: i8) -> Result<Self, ()>;
    fn checked_increment(self) -> Option<Self>;

    fn zero() -> Self;
}

pub trait ToggleTernary: Sized {
    type Target: ToggleTernary<Target=Self>;

    fn toggle(self) -> Self::Target;
}
