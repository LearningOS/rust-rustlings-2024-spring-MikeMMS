/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::cmp::PartialOrd;

fn quicksort_helper<T: PartialOrd + Clone>(slice: &mut [T], l: usize, r: usize) {
    if l >= r {
        return;
    }
    let mid = (l + r) / 2;
    let pivot = slice[mid].clone();
    let mut i = if l == 0 { 0 } else { l - 1 };
    let mut j = r + 1;
    while i < j {
        i = if i == 0 { 0 } else { i + 1 };
        while slice[i] < pivot {
            i += 1;
        }
        j -= 1;
        while slice[j] > pivot {
            j -= 1;
        }
        if i < j {
            slice.swap(i, j);
        }
    }
    quicksort_helper(slice, l, j);
    quicksort_helper(slice, j + 1, r);
}

fn sort<T: PartialOrd + Clone>(array: &mut [T]) {
    let len = array.len();
    quicksort_helper(array, 0, len - 1);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
