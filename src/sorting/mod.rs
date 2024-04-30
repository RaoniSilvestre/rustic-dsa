pub mod bubble_sort;
pub mod merge_sort;
pub mod quick_sort;

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
