mod pre_implemented;

use std::ptr::NonNull;
use std::marker::PhantomData;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node {
            val,
            next: None,
            prev: None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<T>,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    current: Option<NonNull<Node<T>>>,
}

pub struct Iter<'a, T> {
    head: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
            _marker: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        let current = self.head;
        Cursor { list: self, current }
    }

    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let current = self.tail;
        Cursor { list: self, current }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            len: self.len,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        let curr = self.current?;
        unsafe {
            Some(&mut (*curr.as_ptr()).val)
        }
    }

    pub fn next(&mut self) -> Option<&mut T> {
        let curr = self.current?;
        unsafe {
            let next_ptr = (*curr.as_ptr()).next;
            self.current = next_ptr;
            self.peek_mut()
        }
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        let curr = self.current?;
        unsafe {
            let prev_ptr = (*curr.as_ptr()).prev;
            self.current = prev_ptr;
            self.peek_mut()
        }
    }

    pub fn take(&mut self) -> Option<T> {
        let curr = self.current?;
        unsafe {
            let node = Box::from_raw(curr.as_ptr());
            let next = node.next;
            let prev = node.prev;

            if let Some(n) = next {
                (*n.as_ptr()).prev = prev;
            } else {
                self.list.tail = prev;
            }

            if let Some(p) = prev {
                (*p.as_ptr()).next = next;
            } else {
                self.list.head = next;
            }

            self.list.len -= 1;
            self.current = if next.is_some() { next } else { prev };

            Some(node.val)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        let new_node = Box::into_raw(Box::new(Node::new(element)));
        let new_ptr = unsafe { NonNull::new_unchecked(new_node) };

        match self.current {
            None => {
                if self.list.head.is_none() {
                    self.list.head = Some(new_ptr);
                    self.list.tail = Some(new_ptr);
                    self.current = Some(new_ptr);
                } else {
                    let tail = self.list.tail.unwrap();
                    unsafe {
                        (*new_ptr.as_ptr()).prev = Some(tail);
                        (*tail.as_ptr()).next = Some(new_ptr);
                    }
                    self.list.tail = Some(new_ptr);
                    self.current = Some(new_ptr);
                }
            }
            Some(curr) => {
                unsafe {
                    let next = (*curr.as_ptr()).next;
                    (*new_ptr.as_ptr()).prev = Some(curr);
                    (*new_ptr.as_ptr()).next = next;
                    (*curr.as_ptr()).next = Some(new_ptr);

                    if let Some(n) = next {
                        (*n.as_ptr()).prev = Some(new_ptr);
                    } else {
                        self.list.tail = Some(new_ptr);
                    }
                }
            }
        }
        self.list.len += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        let new_node = Box::into_raw(Box::new(Node::new(element)));
        let new_ptr = unsafe { NonNull::new_unchecked(new_node) };

        match self.current {
            None => {
                if self.list.head.is_none() {
                    self.list.head = Some(new_ptr);
                    self.list.tail = Some(new_ptr);
                    self.current = Some(new_ptr);
                } else {
                    let head = self.list.head.unwrap();
                    unsafe {
                        (*new_ptr.as_ptr()).next = Some(head);
                        (*head.as_ptr()).prev = Some(new_ptr);
                    }
                    self.list.head = Some(new_ptr);
                    self.current = Some(new_ptr);
                }
            }
            Some(curr) => {
                unsafe {
                    let prev = (*curr.as_ptr()).prev;
                    (*new_ptr.as_ptr()).next = Some(curr);
                    (*new_ptr.as_ptr()).prev = prev;
                    (*curr.as_ptr()).prev = Some(new_ptr);

                    if let Some(p) = prev {
                        (*p.as_ptr()).next = Some(new_ptr);
                    } else {
                        self.list.head = Some(new_ptr);
                    }
                }
            }
        }
        self.list.len += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            return None;
        }
        let head = self.head?;
        unsafe {
            let node = &*head.as_ptr();
            self.head = node.next;
            self.len -= 1;
            Some(&node.val)
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut curr = self.head;
        while let Some(node_ptr) = curr {
            unsafe {
                let boxed = Box::from_raw(node_ptr.as_ptr());
                curr = boxed.next;
            }
        }
    }
}

#[cfg(feature = "advanced")]
unsafe impl<T: Send> Send for LinkedList<T> {}

#[cfg(feature = "advanced")]
unsafe impl<T: Sync> Sync for LinkedList<T> {}
