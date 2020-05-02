use std::fmt::Debug;

pub trait MySort {
    fn quick_sort(&mut self);
    fn merge_sort(&mut self);
}

impl<T: Ord + Debug + Default + Clone> MySort for [T] {
    #[inline]
    fn quick_sort(&mut self) {
        if self.len() <= 1 { return; }
        qsort(self);
    }

    fn merge_sort(&mut self) {
        let n = self.len();
        if n <= 1 { return; }
        let mid = n >> 1;
        let (left, right) = self.split_at_mut(mid); // [0..mid) [mid, len)
        left.merge_sort();
        right.merge_sort();
        let mut new = Vec::<T>::new();
        new.resize_with(n, T::default);
        let (mut i, mut j, mut k) = (0, 0, 0);
        while i < left.len() && j < right.len() {
            new[k] = if left[i] <= right[j] {
                (left[i].clone(), i += 1).0
            } else {
                (right[j].clone(), j += 1).0
            };
            k += 1;
        }
        if i < left.len() {
            for i in i..left.len() {
                new[k] = left[i].clone();
                k += 1;
            }
        } else {
            for j in j..right.len() {
                new[k] = right[j].clone();
                k += 1;
            }
        }
        self.clone_from_slice(&new);
    }
}

fn qsort<T: Ord + Debug>(a: &mut [T]) {
    let n = a.len();
    debug_assert!(n > 1);
    a.swap(0, 1); // place pivot to 0
    let pivot = unsafe { a.as_ptr().as_ref().unwrap() }; // pivot at 0
    let (mut i, mut j) = (1, n - 1);
    // partition:
    while i < j {
        while i < j && &a[i] <= pivot { i += 1; }
        while i < j && pivot < &a[j] { j -= 1; }
        if i < j {
            unsafe { std::ptr::swap(a.as_mut_ptr().add(i), a.as_mut_ptr().add(j)); }
            i += 1;
            j -= 1;
        }
    }
    debug_assert_eq!(i, j);
    if pivot < &a[i] { a.swap(0, i - 1) } // place pivot to mid
    else { a.swap(0, i); }
    let (l, r) = a.split_at_mut(i);
    // println!("{:?}, {:?}", l, r);
    if 1 < i { qsort(l); }
    if i < n - 1 { qsort(r); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            vec![0, 1, 214, 214, 214, 21, 9, 2],
            vec![],
            vec![9, 3, 5, 6, 14, 5, 6, 6, 2, 6, 19, 21, 912, 312, 312, 4, 21, 2, 435, 4645, 26, 43, 5, 23],
            vec![1],
            vec![1, 34, 3, 5],
            vec![1, 2, 3],
            vec![2, 2, 2],
        ];
        for mut a in cases {
            let mut b = a.clone();
            b.sort_unstable();
            a.merge_sort();
            assert_eq!(a, b);
        }
    }
}

