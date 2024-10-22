pub mod bubble_sort;
pub mod merge_sort;
pub mod quick_sort;

pub trait OrderedCopy: PartialOrd + Copy {}

impl<T: PartialOrd + Copy> OrderedCopy for T {}

pub trait IterativeBubbleSort<T> {
    fn iterative_bubble_sort(&mut self);
}

pub trait RecursiveBubbleSort<T> {
    fn recursive_bubble_sort(&mut self);
}
// Alias genérico que pode ser reutilizado
// To create a new sorting algorithm, you need to:
// 1 . Create a file in this folder named new_sort.
// 2 . Add the file to the rust tree :
//      pub mod new_sort; [On the start of this file]
// 3. Create your code inside your new file:
// This is the default template to create new sorting algorithm for Vec<T>.
// pub trait NewSort<T: PartialOrd + Copy> {
//    fn new_sort(&mut self);
// }
// impl<T> NewSort<T> for Vec<T>
// where
//    T: PartialOrd + Copy,
// {
//     fn new_sort(&mut self){
//         [Your code goes here]
//     }
// }
