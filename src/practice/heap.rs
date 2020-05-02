pub struct MyHeap<T: PartialOrd> {
    a: Vec<T>,
}

impl<T: PartialOrd> MyHeap<T> {
    #[inline]
    pub fn new() -> Self { Self { a: Vec::new() } }

    #[inline]
    pub fn push(&mut self, x: T) {
        self.a.push(x);
        self.float(self.size() - 1);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        match self.a.last() {
            None => None,
            Some(_) => {
                let n = self.size();
                self.a.swap(0, n - 1);
                (self.a.pop(), self.sink(0)).0
            }
        }
    }

    #[inline]
    pub fn top(&self) -> Option<&T> { self.a.first() }

    #[inline]
    pub fn size(&self) -> usize { self.a.len() }

    pub fn float(&mut self, mut i: usize) {
        while i > 0 {
            let p = to_parent(i);
            if self.a[i] > self.a[p] { self.a.swap(i, p); } else { break; }
            i = p;
        }
    }

    pub fn sink(&mut self, mut i: usize) {
        let n = self.size();
        let mut l = to_left(i);
        while l < n {
            let at_max = {
                let mut m = i;
                if self.a[l] > self.a[i] { m = l; }
                if l + 1 < n && self.a[l + 1] > self.a[m] { m = l + 1; }
                m
            };
            if i != at_max {
                self.a.swap(i, at_max);
                i = at_max;
                l = to_left(i);
            } else { break; }
        }
    }
}

#[inline]
fn to_parent(i: usize) -> usize {
    (i + 1) / 2 - 1
}

#[inline]
fn to_left(i: usize) -> usize {
    i * 2 + 1
}

// #[inline]
// fn to_right(i: usize) -> usize {
//     i * 2 + 2
// }

pub fn heap_sort<T: PartialOrd + Default + Clone>(arr: &mut [T]) {
    let mut heap = MyHeap::new();
    arr.iter().for_each(|x| heap.push(x.clone()));
    let mut ret = Vec::new();
    while heap.size() > 0 { ret.push(heap.pop().unwrap()); }
    arr.clone_from_slice(&ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut h = MyHeap::new();
        h.push(1);
        h.push(2);
        h.push(3);
        h.push(1);
        assert_eq!(h.size(), 4);
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(2));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.size(), 0);
    }

    #[test]
    fn sort() {
        let mut a = vec![4, 3, 12, 4, 3, 234, 3, 42, 121, 4, 5, 6, 7346, 1, 12];
        let mut b = a.clone();
        b.sort_unstable_by(|x, y| y.cmp(x));
        heap_sort(a.as_mut_slice());
        assert_eq!(a, b);
    }
}
