pub trait MergeSort<T: PartialOrd + Copy> {
    fn merge_sort(&mut self);
}

impl<T> MergeSort<T> for Vec<T>
where
    T: PartialOrd + Copy,
{
    fn merge_sort(&mut self) {}
}
