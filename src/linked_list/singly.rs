use core::marker::PhantomData;
use core::ptr::NonNull;

pub struct SinglyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    _marker: PhantomData<Box<Node<T>>>
}

struct Node<T> {
    inner: T,
    next: Option<NonNull<Node<T>>>
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

pub struct Iter<T> {
    head: Option<NonNull<Node<T>>>,
    _marker: PhantomData<Box<Node<T>>>, // should this be &'a Node<T>?
}