pub fn select_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut mi = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[mi] {
                mi = j;
            } 
        }
        arr.swap(i, mi);
    }
}

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn insert_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let tmp = unsafe { core::ptr::read(&mut arr[i]) };
        for j in (0..i).rev() {
            if tmp < arr[j] {
                arr.swap(j, j + 1);
            } else {
                break;
            }
        }
    }
}

pub fn count_sort<T: Ord, F, G>(arr: &mut [T], discrete: F, reveal: G)
where 
    F: Fn(T) -> usize, 
    G: Fn(usize) -> T 
{
    let (mut min, mut max) = (usize::max_value(), usize::min_value());
    for elem in arr.iter() {
        let mapped = discrete(unsafe { core::ptr::read(elem) });
        if mapped < min {
            min = mapped
        }
        if mapped > max {
            max = mapped
        }
    }
    if max == usize::min_value() && min == usize::max_value() {
        return; // empty array, nothing to do
    }
    let mut cnt = vec![0usize; max - min + 1];    
    for elem in arr.iter() {
        let mapped = discrete(unsafe { core::ptr::read(elem) });
        cnt[mapped - min] += 1;
    }
    let mut ptr = 0;
    for i in min..=max {
        for _j in 0..cnt[i - min] {
            arr[ptr] = reveal(i);
            ptr += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    macro_rules! sort_test {
        ($fn_name: ident) => {
#[test]
fn $fn_name() {
    let mut arr = vec![1,2,2,2,1,1,1,2,2,2,2,2,1,1,2,2];
    super::$fn_name(&mut arr);
    assert_eq!(arr, vec![1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2]);
    let mut arr = vec![1, 9, 7, 2, 3, 4, 5, 8, 0, 6];
    super::$fn_name(&mut arr);
    assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut arr: Vec<u8> = vec![];
    super::$fn_name(&mut arr);
    assert_eq!(arr, vec![]);
    let mut arr = vec![1];
    super::$fn_name(&mut arr);
    assert_eq!(arr, vec![1]);
}
        };
    }
    sort_test!(select_sort);
    sort_test!(bubble_sort);
    sort_test!(insert_sort);

    #[test]
    fn count_sort() {
        let mut arr = vec![1,2,2,2,1,1,1,2,2,2,2,2,1,1,2,2];
        super::count_sort(&mut arr, |a| a, |a| a);
        assert_eq!(arr, vec![1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2]);
        let mut arr = vec![1, 9, 7, 2, 3, 4, 5, 8, 0, 6];
        super::count_sort(&mut arr, |a| a, |a| a);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut arr: Vec<usize> = vec![];
        super::count_sort(&mut arr, |a| a, |a| a);
        assert_eq!(arr, vec![]);
        let mut arr = vec![1];
        super::count_sort(&mut arr, |a| a, |a| a);
        assert_eq!(arr, vec![1]);
    }
}
