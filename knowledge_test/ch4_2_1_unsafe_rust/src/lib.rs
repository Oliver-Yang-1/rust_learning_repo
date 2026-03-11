use std::slice;

pub unsafe fn dereference_raw(ptr: *const i32) -> i32 {
    // 调用者必须保证 ptr 非空、已对齐，且指向有效的 i32。
    unsafe { *ptr }
}

pub fn split_at_mut_safe(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // 这里把不安全操作包进一个安全函数中，对外暴露安全 API。
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{dereference_raw, split_at_mut_safe};

    #[test]
    fn raw_pointer_can_be_dereferenced_in_unsafe_block() {
        let value = 5;
        let ptr = &value as *const i32;

        let result = unsafe { dereference_raw(ptr) };
        assert_eq!(result, 5);
    }

    #[test]
    fn split_at_mut_safe_splits_without_overlap() {
        let mut values = vec![1, 2, 3, 4, 5, 6];
        let (left, right) = split_at_mut_safe(&mut values, 3);

        assert_eq!(left, &mut [1, 2, 3]);
        assert_eq!(right, &mut [4, 5, 6]);
    }
}
