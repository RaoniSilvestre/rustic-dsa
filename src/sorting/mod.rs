pub mod bubble;
pub mod merge;
pub mod quick;

pub trait OrderedCopy: PartialOrd + Copy {}

impl<T: PartialOrd + Copy> OrderedCopy for T {}
