use std::ops::{Index, IndexMut};

pub struct List<T> {
    head: Link<T>,
    len: usize,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    #[inline]
    fn new(data: T) -> Self { Self { data, next: None } }
}

impl<T> List<T> {
    #[inline]
    pub fn new() -> Self { Self { head: None, len: 0 } }

    pub fn push_front(&mut self, data: T) {
        self.len += 1;
        let old_head = self.head.take();
        self.head = Some(Box::new(Node { data, next: old_head }));
    }

    #[inline]
    pub fn push_back(&mut self, data: T) {
        self.len += 1;
        *self.tail_mut() = Some(Box::new(Node::new(data)));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = self.head.take();
        match old_head {
            None => None,
            Some(old_head_node) => {
                self.len -= 1;
                self.head = old_head_node.next;
                Some(old_head_node.data)
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        (match self.tail_1_mut() {
            None => return None,
            Some(back) => Some(back.take().unwrap().data)
        }, self.len -= 1).0
    }

    #[inline]
    pub fn front(&self) -> Option<&T> { self.head.as_ref().map(|node| &node.data) }

    #[inline]
    pub fn back(&self) -> Option<&T> { self.tail_1().map(|link| &link.as_ref().unwrap().data) }

    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> { self.head.as_mut().map(|node| &mut node.data) }

    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> { self.tail_1_mut().map(|link| &mut link.as_mut().unwrap().data) }

    #[inline]
    pub fn len(&self) -> usize { self.len }

    fn at(&self, mut index: usize) -> Option<&Link<T>> {
        let mut ptr = &self.head;
        while index > 0 {
            match ptr {
                None => break, // tail link
                Some(node) => ptr = &node.next,
            }
            index -= 1;
        }
        if index > 0 { None } else { Some(ptr) }
    }

    fn at_mut(&mut self, mut index: usize) -> Option<&mut Link<T>> {
        let mut ptr = &mut self.head;
        while index > 0 {
            match ptr {
                None => break,
                Some(node) => ptr = &mut node.next,
            }
            index -= 1;
        }
        if index > 0 { None } else { Some(ptr) }
    }

    fn tail(&self) -> &Link<T> {
        let mut ptr = &self.head;
        loop {
            match ptr {
                None => return ptr,
                Some(node) => ptr = &node.next,
            }
        }
    }

    fn tail_mut(&mut self) -> &mut Link<T> {
        let mut ptr = &mut self.head;
        loop {
            match ptr {
                None => return ptr,
                Some(node) => ptr = &mut node.next,
            }
        }
    }

    /// The last but one. If Some(link), the link here must target some node
    #[inline]
    fn tail_1(&self) -> Option<&Link<T>> { if self.len > 0 { self.at(self.len - 1) } else { None } }

    /// The last but one. If Some(link), the link here must target some node
    #[inline]
    fn tail_1_mut(&mut self) -> Option<&mut Link<T>> { if self.len > 0 { self.at_mut(self.len - 1) } else { None } }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // while self.pop_front().is_some() {}
        let mut cur_link = self.head.take();
        while let Some(node) = &mut cur_link { cur_link = node.next.take(); }
    }
}

// ==== Iterator api ====:

pub struct Iter<'a, T> {
    cur: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.map(|node| {
            self.cur = node.next.as_ref().map(|node| node.as_ref());
            &node.data
        })
    }
}

pub struct IterMut<'a, T> {
    cur: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        self.cur.take().map(|node| {
            self.cur = node.next.as_mut().map(|node| node.as_mut());
            &mut node.data
        })
    }
}

impl<T> List<T> {
    #[inline]
    pub fn iter(&self) -> Iter<T> { Iter { cur: self.head.as_ref().map(|node| node.as_ref()) } }
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> { IterMut { cur: self.head.as_mut().map(|node| node.as_mut()) } }
}

pub struct IntoIter<T> {
    cur: Link<T>
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.take().map(|node| {
            self.cur = node.next;
            node.data
        })
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(mut self) -> IntoIter<T> {
        IntoIter { cur: self.head.take() }
    }
}

// ==== Insert/remove api ====:

impl<T> List<T> {
    pub fn insert(&mut self, index: usize, data: T) -> bool {
        let inserted = match self.at_mut(index) {
            None => false, // out of bounds
            Some(link) => {
                let new_node = Box::new(Node {
                    data,
                    next: link.take(),
                });
                *link = Some(new_node);
                true
            }
        };
        if inserted { self.len += 1; }
        inserted
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let removed = match self.at_mut(index) {
            None => None, // out of bounds
            Some(link) => {
                match link.take() {
                    None => None, // at tail
                    Some(node_to_remove) => {
                        *link = node_to_remove.next;
                        Some(node_to_remove.data)
                    }
                }
            }
        };
        if removed.is_some() { self.len -= 1; }
        removed
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output { &self.at(index).unwrap().as_ref().unwrap().data }
}

impl<T> IndexMut<usize> for List<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.at_mut(index).unwrap().as_mut().unwrap().data }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);
        assert_eq!(list.len(), 0);

        // Populate list
        list.push_front(1);
        list.push_back(2);
        list.push_front(3);
        assert_eq!(list.front(), Some(&3));
        *list.front_mut().unwrap() = -1;
        assert_eq!(list.front(), Some(&-1));
        *list.front_mut().unwrap() = 3;
        assert_eq!(list.back(), Some(&2));
        *list.back_mut().unwrap() = -5;
        assert_eq!(list.back(), Some(&-5));
        *list.back_mut().unwrap() = 2;
        assert_eq!(list.len(), 3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.len(), 1);

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);
        assert_eq!(list[0], 1);
        assert_eq!(list[1], 4);
        assert_eq!(list[2], 5);
        list[2] = -5;

        // Check normal removal
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(-5));

        // Check exhaustion
        assert_eq!(list.back(), Some(&4));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn iterator() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        let v = list.iter().collect::<Vec<_>>();
        assert_eq!(v, [&3, &2, &1]);
        list.iter_mut().for_each(|x| *x = *x * *x);
        let v = list.into_iter().collect::<Vec<_>>();
        assert_eq!(v, [9, 4, 1]);
    }

    #[test]
    fn insert_remove() {
        // insert
        let mut list = List::new();
        assert!(list.insert(0, 1));
        list.push_back(2);
        list.push_back(3);
        let v = list.iter().collect::<Vec<_>>();
        assert_eq!(v, [&1, &2, &3]);
        assert!(list.insert(0, 0));
        assert!(list.insert(4, -1));
        let v = list.iter().collect::<Vec<_>>();
        assert_eq!(v, [&0, &1, &2, &3, &-1]);
        assert!(!list.insert(6, 1));
        let v = list.iter().collect::<Vec<_>>();
        assert_eq!(v, [&0, &1, &2, &3, &-1]);
        assert_eq!(list.len(), 5);

        // remove
        assert!(list.remove(6).is_none());
        assert_eq!(list.remove(0), Some(0));
        assert_eq!(list.remove(1), Some(2));
        assert_eq!(list.remove(3), None);
        let v = list.iter().collect::<Vec<_>>();
        assert_eq!(v, [&1, &3, &-1]);
        assert_eq!(list.remove(2), Some(-1));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.remove(0), Some(3));
        assert_eq!(list.remove(0), None);
        assert_eq!(list.len(), 0);
    }
}