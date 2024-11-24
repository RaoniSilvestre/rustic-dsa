pub mod bubble;
pub mod merge;
pub mod quick;

pub trait OrderedCopy: PartialOrd + Copy {}

impl<T: PartialOrd + Copy> OrderedCopy for T {}

pub type SortFunction<T> = fn(&mut Vec<T>);

pub trait IsSorted<T: PartialOrd> {
    fn is_sortted(&self) -> bool;
}

impl<T> IsSorted<T> for Vec<T>
where
    T: PartialOrd,
{
    fn is_sortted(&self) -> bool {
        for i in 1..self.len() {
            if self[i - 1] > self[i] {
                return false;
            }
        }

        true
    }
}
