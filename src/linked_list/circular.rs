use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ptr::NonNull;
use core::mem;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::cmp::Ordering;
use core::iter::FromIterator;

pub struct CircularLinkedList<T> {
    tail: Option<NonNull<Node<T>>>,
    _marker: PhantomData<Box<Node<T>>>
}

struct Node<T> {
    elem: T,
    next: Option<NonNull<Node<T>>>,
}

pub struct Iter<'a, T: 'a> {
    cur: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a Node<T>>
}

pub struct IterMut<'a, T: 'a> {
    cur: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a mut Node<T>>,
}

pub struct IntoIter<T> {
    list: CircularLinkedList<T> 
}

impl<T> CircularLinkedList<T> {
    pub fn new() -> Self {
        Self {
            tail: None,
            _marker: PhantomData
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tail.is_none()
    }

    pub fn clear(&mut self) {
        *self = Self::new()
        // drop(self) is called
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { 
            cur: self.tail.as_ref().map(|tail_ptr| unsafe { tail_ptr.as_ref() }.next.unwrap()), 
            tail: self.tail,
            _marker: PhantomData
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { 
            cur: self.tail.as_mut().map(|tail_ptr| unsafe { tail_ptr.as_mut() }.next.unwrap()), 
            tail: self.tail,
            _marker: PhantomData
        }
    }

    pub fn contains(&self, value: &T) -> bool
    where T: PartialEq<T> 
    {
        self.iter().any(|elem| elem == value)
    }
}

impl<T> Drop for CircularLinkedList<T> {
    fn drop(&mut self) {
        let mut cur = self.tail;
        while let Some(node_ptr) = cur {
            let node = unsafe { Box::from_raw(node_ptr.as_ptr()) };
            cur = node.next;
            if cur == self.tail { break }
            // drop(node) is called
        }
    }
}

// O(1) operations
impl<T> CircularLinkedList<T> {
    pub fn back(&self) -> Option<&T> {
        self.tail.as_ref().map(|node_ptr| &unsafe { node_ptr.as_ref() }.elem)
    }
    
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail.as_mut().map(|node_ptr| &mut unsafe { node_ptr.as_mut() }.elem)
    }

    pub fn front(&self) -> Option<&T> {
        self.tail.as_ref().map(|node_ptr| &unsafe { &*node_ptr.as_ref().next.unwrap().as_ptr() }.elem)
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.tail.as_mut().map(|node_ptr| &mut unsafe { &mut *node_ptr.as_mut().next.unwrap().as_ptr() }.elem)
    }
    
    pub fn push_front(&mut self, elem: T) {
        if let Some(mut tail_ptr) = self.tail {
            let head_ptr = unsafe { tail_ptr.as_ref() }.next;
            let node = Box::new(Node { elem, next: head_ptr });
            let new_head_ptr = NonNull::new(Box::into_raw(node));
            assert!(new_head_ptr.is_some());
            unsafe { tail_ptr.as_mut() }.next = new_head_ptr;
        } else {
            let node = Box::new(Node { elem, next: None });
            let new_head_ptr = NonNull::new(Box::into_raw(node));
            assert!(new_head_ptr.is_some());
            unsafe { new_head_ptr.unwrap().as_mut() }.next = new_head_ptr;
            self.tail = new_head_ptr;
        } 
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.tail.map(|mut tail_ptr| {
            let head_ptr = unsafe { tail_ptr.as_ref() }.next; 
            assert!(head_ptr.is_some());
            if head_ptr == self.tail {
                self.tail = None;
            } else {
                let new_head_ptr = unsafe { head_ptr.unwrap().as_ref() }.next; 
                unsafe { tail_ptr.as_mut() }.next = new_head_ptr;
            }
            let head = unsafe { Box::from_raw(head_ptr.unwrap().as_ptr()) };
            head.elem
        })
    }

    pub fn push_back(&mut self, elem: T) {
        self.push_front(elem);
        assert!(self.tail.is_some());
        self.tail = unsafe { self.tail.unwrap().as_ref() }.next;
    }

    pub fn append(&mut self, other: &mut Self) {
        match (self.tail, other.tail) {
            (None, _) => mem::swap(self, other),
            (Some(_tail_ptr), None) => {}, // append an empty list, nothing to do
            (Some(mut tail_ptr), Some(mut other_tail_ptr)) => {
                mem::swap(&mut unsafe { tail_ptr.as_mut() }.next, &mut unsafe { other_tail_ptr.as_mut() }.next);
            }
        }
    }
}

// O(n) operations
impl<T> CircularLinkedList<T> {
    pub fn len(&self) -> usize {
        let mut cur = self.tail;
        let mut ans = 0;
        while let Some(node_ptr) = cur {
            ans += 1;
            cur = unsafe { node_ptr.as_ref() }.next;
            if cur == self.tail { break }
        } 
        ans
    } 

    pub fn pop_back(&mut self) -> Option<T> {
        let mut new_tail = None;
        let mut cur = self.tail;
        while let Some(node_ptr) = cur {
            new_tail = cur;
            cur = unsafe { node_ptr.as_ref() }.next;
            if cur == self.tail { break }
        } 
        drop(cur);
        new_tail.map(|mut new_tail_ptr| {
            assert!(self.tail.is_some());
            unsafe { new_tail_ptr.as_mut() }.next = unsafe { self.tail.unwrap().as_ref() }.next;
            let ans = unsafe { Box::from_raw(self.tail.unwrap().as_ptr()) };
            self.tail = new_tail;
            ans.elem
        })
    }

    pub fn split_off(&mut self, at: usize) -> CircularLinkedList<T> {
        if at == 0 {
            return mem::replace(self, Self::new())
        }
        let mut cur = self.tail;
        let mut next_id = 0;
        while let Some(node_ptr) = cur {
            if next_id == at {
                break;
            }
            cur = unsafe { node_ptr.as_ref() }.next;
            next_id += 1;
        }
        assert!(at <= next_id, "Cannot split off a nonexistent index");
        if let Some(mut node_ptr) = cur {
            let second_part = Self {
                tail: self.tail,
                _marker: PhantomData
            };
            mem::swap(&mut unsafe { node_ptr.as_mut() }.next, 
                &mut unsafe { self.tail.unwrap().as_mut() }.next);
            second_part
        } else {
            Self::new()
        }
    }
}
impl<'a, T> IntoIterator for CircularLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

impl<'a, T> IntoIterator for &'a CircularLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut CircularLinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<T: fmt::Debug> fmt::Debug for CircularLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: Hash> Hash for CircularLinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in self {
            item.hash(state);
        }
    }
}

impl<T: PartialEq> PartialEq for CircularLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other)
    }

    fn ne(&self, other: &Self) -> bool {
        self.iter().ne(other)
    }
}

impl<T: Eq> Eq for CircularLinkedList<T> {}

impl<T: PartialOrd> PartialOrd for CircularLinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for CircularLinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T> Extend<T> for CircularLinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter {
            self.push_back(elem);
        }
    }
}

impl<'a, T: 'a + Copy> Extend<&'a T> for CircularLinkedList<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

impl<T> FromIterator<T> for CircularLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T: Clone> Clone for CircularLinkedList<T> {
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

impl<T> Default for CircularLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<T: Send> Send for CircularLinkedList<T> {}

unsafe impl<T: Sync> Sync for CircularLinkedList<T> {}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.cur == self.tail {
            return None;
        }
        self.cur.map(|cur_ptr| {
            let node = unsafe { &*cur_ptr.as_ptr() };
            self.cur = node.next;
            &node.elem
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
        if self.cur == self.tail {
            return None;
        }
        self.cur.map(|cur_ptr| {
            let node = unsafe { &mut *cur_ptr.as_ptr() };
            self.cur = node.next;
            &mut node.elem
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
