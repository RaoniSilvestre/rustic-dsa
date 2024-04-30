use std::fmt::Display;

pub trait QuickSort<T: PartialOrd + Copy> {
    fn quick_sort(&mut self);
}

impl<T> QuickSort<T> for Vec<T>
where
    T: PartialOrd + Copy + Display,
{
    fn quick_sort(&mut self) {
        sorting(self, 0, self.len() - 1)
    }
}

fn sorting<T: PartialOrd + Copy + Display>(lista: &mut Vec<T>, left: usize, right: usize) {
    if left < right {
        let index_pivot = partition(lista, left, right);
        sorting(lista, left, index_pivot);
        sorting(lista, index_pivot + 1, right);
    }
}

fn partition<T: PartialOrd + Copy>(lista: &mut Vec<T>, mut left: usize, mut right: usize) -> usize {
    let pivot = left;
    left = left + 1;

    while right >= left {
        while left < lista.len() && lista[left] < lista[pivot] {
            left = left + 1;
        }

        while lista[right] > lista[pivot] {
            if right == 0 {
                break;
            }
            right = right - 1;
        }

        if right >= left {
            lista.swap(left, right);
            if right != 0 {
                right -= 1;
            }
        }
    }

    lista.swap(pivot, right);

    right
}
