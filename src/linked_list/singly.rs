use core::marker::PhantomData;
use core::ptr::NonNull;
use core::iter::FusedIterator;
use core::fmt;
use core::cmp::Ordering;
use core::iter::FromIterator;
use core::hash::{Hasher, Hash};
use core::mem;

pub struct SinglyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    _marker: PhantomData<Box<Node<T>>>
}

struct Node<T> {
    inner: T,
    next: Option<NonNull<Node<T>>>
}

#[derive(Clone)]
pub struct Iter<'a, T: 'a> {
    cur: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a Node<T>>, 
}

pub struct IterMut<'a, T: 'a> {
    cur: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a Node<T>>, 
}

#[derive(Clone)]
pub struct IntoIter<T> {
    list: SinglyLinkedList<T>
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            _marker: PhantomData
        }
    }

    pub fn len(&self) -> usize {
        let mut cur = self.head;
        let mut ans = 0;
        while let Some(node_ptr) = cur {
            ans += 1;
            cur = unsafe { node_ptr.as_ref() }.next;
        }
        ans
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            cur: self.head,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            cur: self.head,
            _marker: PhantomData,
        }
    }

    pub fn contains(&self, value: &T) -> bool
    where T: PartialEq<T> 
    {
        self.iter().any(|elem| elem == value)
    }
}

impl<T> SinglyLinkedList<T> {
    fn destroy(&mut self) {
        let mut cur = self.head;
        while let Some(node_ptr) = cur {
            let node = unsafe { Box::from_raw(node_ptr.as_ptr()) };
            cur = node.next;
            // drop(node) is called
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new()
        // drop(self) is called
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        self.destroy()
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node_ptr| &unsafe { node_ptr.as_ref() }.inner)
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node_ptr| &mut unsafe { node_ptr.as_mut() }.inner)
    }

    pub fn push_front(&mut self, inner: T) {
        let node = Box::new(Node { inner, next: self.head });
        self.head = NonNull::new(Box::into_raw(node));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|head_ptr| {
            let node = unsafe { Box::from_raw(head_ptr.as_ptr()) };
            self.head = node.next;
            node.inner
        })
    }
}

impl<T> SinglyLinkedList<T> {
    fn tail(&self) -> Option<NonNull<Node<T>>> {
        let mut cur = self.head;
        let mut ans = None;
        while let Some(node_ptr) = cur {
            ans = cur;
            cur = unsafe { node_ptr.as_ref() }.next;
        }
        ans
    }
    
    pub fn append(&mut self, other: &mut Self) {
        if let Some(mut tail_ptr) = self.tail() {
            unsafe { tail_ptr.as_mut() }.next = other.head.take();
        } else {
            mem::swap(self, other)
        }
    }

    pub fn back(&self) -> Option<&T> {
        self.tail().as_ref().map(|node_ptr| &unsafe { &*node_ptr.as_ptr() }.inner)
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail().as_mut().map(|node_ptr| &mut unsafe { &mut *node_ptr.as_ptr() }.inner)
    }

    pub fn push_back(&mut self, inner: T) {
        let node = Box::new(Node { inner, next: None });
        let new_tail_ptr = NonNull::new(Box::into_raw(node));
        if let Some(mut tail_ptr) = self.tail() {
            unsafe { tail_ptr.as_mut() }.next = new_tail_ptr;
        } else {
            self.head = new_tail_ptr;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let (mut new_tail, mut old_tail) = (None, None);
        let mut cur = self.head;
        while let Some(node_ptr) = cur {
            new_tail = cur;
            if let Some(next_ptr) = unsafe { node_ptr.as_ref() }.next {
                old_tail = Some(next_ptr);
            }
            cur = unsafe { node_ptr.as_ref() }.next;
        }
        match (new_tail, old_tail) {
            (None, None) => None,
            (Some(node_ptr), None) => {
                let node = unsafe { Box::from_raw(node_ptr.as_ptr()) };
                self.head = None;
                Some(node.inner)
            },
            (Some(mut new_tail_ptr), Some(old_tail_ptr)) => {
                unsafe { new_tail_ptr.as_mut() }.next = None;
                let node = unsafe { Box::from_raw(old_tail_ptr.as_ptr()) };
                Some(node.inner)
            }
            _ => unreachable!()
        }
    }
}

impl<T> SinglyLinkedList<T> { 
    pub fn split_off(&mut self, at: usize) -> SinglyLinkedList<T> {
        if at == 0 {
            return mem::replace(self, Self::new())
        }
        let mut cur = self.head;
        let mut next_id = 0;
        while let Some(node_ptr) = cur {
            next_id += 1;
            if next_id == at {
                break;
            }
            cur = unsafe { node_ptr.as_ref() }.next;
        }
        if let Some(_) = cur {
            let second_part = Self {
                head: cur.take(),
                _marker: PhantomData
            };
            second_part
        } else {
            Self::new()
        }
    }
}

impl<'a, T> IntoIterator for SinglyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

impl<'a, T> IntoIterator for &'a SinglyLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut SinglyLinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<T: fmt::Debug> fmt::Debug for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: Hash> Hash for SinglyLinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in self {
            item.hash(state);
        }
    }
}

impl<T: PartialEq> PartialEq for SinglyLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other)
    }

    fn ne(&self, other: &Self) -> bool {
        self.iter().ne(other)
    }
}

impl<T: Eq> Eq for SinglyLinkedList<T> {}

impl<T: PartialOrd> PartialOrd for SinglyLinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for SinglyLinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T> Extend<T> for SinglyLinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item)
        }
    }
}

impl<'a, T: 'a + Copy> Extend<&'a T> for SinglyLinkedList<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

impl<T> FromIterator<T> for SinglyLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T: Clone> Clone for SinglyLinkedList<T> {
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

impl<T> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<T: Send> Send for SinglyLinkedList<T> {}

unsafe impl<T: Sync> Sync for SinglyLinkedList<T> {}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.cur.map(|cur_ptr| {
            let node = unsafe { &*cur_ptr.as_ptr() };
            self.cur = node.next;
            &node.inner
        })
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}

impl<'a, T: 'a + fmt::Debug> fmt::Debug for Iter<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Iter").finish()
    }
}

unsafe impl<'a, T: Send> Send for Iter<'a, T> {}

unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        self.cur.map(|cur_ptr| {
            let node = unsafe { &mut *cur_ptr.as_ptr() };
            self.cur = node.next;
            &mut node.inner
        })
    }
}

impl<'a, T> FusedIterator for IterMut<'a, T> {}

impl<'a, T: 'a + fmt::Debug> fmt::Debug for IterMut<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IterMut").finish()
    }
}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}

unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.list.pop_front()
    }
}

impl<T> FusedIterator for IntoIter<T> {}
