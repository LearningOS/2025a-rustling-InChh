/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd + Clone>(array: &mut [T]) {
    let len = array.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    let (left_array, right_array) = array.split_at_mut(mid);
    sort(left_array);
    sort(right_array);
    let mut sorted_left = left_array.to_vec();
    let mut sorted_right = right_array.to_vec();
    merge(array, &sorted_left, &sorted_right);
}

fn merge<T: PartialOrd + Clone>(array: &mut [T], left_array: &[T], right_array: &[T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left_array.len() && j < right_array.len() {
        if left_array[i] <= right_array[j] {
            array[k] = left_array[i].clone();
            i += 1;
        } else {
            array[k] = right_array[j].clone();
            j += 1;
        }
        k += 1;
    }
    while i < left_array.len() {
        array[k] = left_array[i].clone();
        i += 1;
        k += 1;
    }
    while j < right_array.len() {
        array[k] = right_array[j].clone();
        j += 1;
        k += 1;
    }
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
