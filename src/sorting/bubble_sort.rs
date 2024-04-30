pub trait BubbleSort<T: PartialOrd + Copy> {
    fn bubble_sort(&mut self);
}

impl<T> BubbleSort<T> for Vec<T>
where
    T: PartialOrd + Copy,
{
    fn bubble_sort(&mut self) {
        for i in 0..self.len() {
            for j in 0..self.len() {
                if self[i] < self[j] {
                    self.swap(i, j)
                }
            }
        }
    }
}
