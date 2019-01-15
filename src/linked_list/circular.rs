use core::marker::PhantomData;
use core::ptr::NonNull;

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
    _marker: PhantomData<&'a Node<T>>
}

pub struct IterMut<'a, T: 'a> {
    cur: Option<NonNull<Node<T>>>,
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

    pub fn is_empty(&self) -> bool {
        self.tail.is_none()
    }

    pub fn clear(&mut self) {
        *self = Self::new()
        // drop(self) is called
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

    pub fn push_back(&mut self, elem: T) {
        let node = Box::new(Node { elem, next: None });
        let new_tail_ptr = NonNull::new(Box::into_raw(node));
        assert!(new_tail_ptr.is_some());
        unsafe { new_tail_ptr.unwrap().as_mut() }.next = 
            if let Some(tail_ptr) = self.tail {
                unsafe { tail_ptr.as_ref() }.next
            } else {
                new_tail_ptr
            };
        self.tail = new_tail_ptr;
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
}