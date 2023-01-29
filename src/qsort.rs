#[allow(dead_code)]
fn partition<A: Ord + Copy>(arr: &mut [A], mut start: usize, mut end: usize) -> usize {
    let pivot_value = arr[(end + start) / 2];
    while start <= end {
        while arr[start] < pivot_value {
            start = start + 1;
        }
        while arr[end] > pivot_value {
            end = end - 1;
        }
        if start <= end {
            let tmp = arr[start];
            arr[start] = arr[end];
            arr[end] = tmp;

            start = start + 1;
            end = end - 1;
        }
    }
    start
}

#[allow(dead_code)]
pub fn qsort<A: Ord + Copy>(arr: &mut [A], start: usize, end: usize) {
    if end - start > 2 {
        let new_start = partition(arr, start, end);
        if new_start - start > 1 {
            qsort(arr, start, new_start - 1)
        }
        if new_start < end {
            qsort(arr, new_start, end);
        }
    }
}
