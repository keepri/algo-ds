use std::{
    cell::{Ref, RefCell},
    default::Default,
    fmt::Debug,
    // sync::{Arc, Mutex},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct SNode<T> {
    value: T,
    // single thread
    prev: Option<Rc<RefCell<SNode<T>>>>,
    // multi thread
    // prev: Option<Arc<Mutex<SNode<T>>>>,
}

#[derive(Debug, Clone)]
pub struct Stack<T> {
    head: Option<Rc<RefCell<SNode<T>>>>,
    tail: Option<Rc<RefCell<SNode<T>>>>,
    len: usize,
}

#[allow(dead_code)]
impl<T> Stack<T>
where
    T: Default + Debug,
{
    pub fn default() -> Self {
        return Stack {
            head: None,
            tail: None,
            len: 0,
        };
    }

    pub fn new() -> Self {
        return Stack {
            head: None,
            tail: None,
            len: 0,
        };
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn push(&mut self, item: T) -> () {
        let node = Rc::new(RefCell::new(SNode {
            value: item,
            prev: None,
        }));

        if let Some(tail) = self.tail.take() {
            node.borrow_mut().prev = Some(tail);
        } else {
            self.head = Some(Rc::clone(&node));
        };

        self.tail = Some(node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(tail) = self.tail.take() {
            let mut tail = tail.borrow_mut();
            self.tail = tail.prev.clone().take();

            if self.len == 1 {
                self.head = None;
            }

            self.len -= 1;
            return Some(std::mem::replace(&mut tail.value, Default::default()));
        } else {
            return None;
        };
    }

    pub fn peek(&self) -> Option<Ref<T>> {
        if let Some(tail) = self.tail.as_ref() {
            return Some(Ref::map(tail.borrow(), |node| &node.value));
        } else {
            return None;
        }
    }

    pub fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
