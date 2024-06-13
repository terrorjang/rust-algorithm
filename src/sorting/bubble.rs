pub fn bubble_sort(arr: &mut Vec<i32>) {
    let len = arr.len();

    let mut max = arr.len();

    while max > 0 {
        for i in 1..max {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
            }
        }

        max -= 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut arr = vec![6, 5, 3, 1, 8, 7, 2, 4];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
