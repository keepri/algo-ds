use std::{
    cell::{Ref, RefCell},
    default::Default,
    fmt::Debug,
    // sync::{Arc, Mutex},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct QNode<T> {
    value: T,
    // single thread
    next: Option<Rc<RefCell<QNode<T>>>>,
    // multi thread
    // next: Option<Arc<Mutex<QNode<T>>>>,
}

#[derive(Debug, Clone)]
pub struct Queue<T> {
    head: Option<Rc<RefCell<QNode<T>>>>,
    tail: Option<Rc<RefCell<QNode<T>>>>,
    len: usize,
}

impl<T: Default + Debug> Queue<T> {
    #[allow(dead_code)]
    pub fn default() -> Self {
        return Queue {
            head: None,
            tail: None,
            len: 0,
        };
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        return Queue {
            head: None,
            tail: None,
            len: 0,
        };
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        return self.len;
    }

    #[allow(dead_code)]
    pub fn enqueue(&mut self, item: T) -> () {
        let node = Rc::new(RefCell::new(QNode {
            value: item,
            next: None,
        }));

        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(Rc::clone(&node));
        } else {
            self.head = Some(Rc::clone(&node));
        };

        self.tail = Some(node);
        self.len += 1;
    }

    #[allow(dead_code)]
    pub fn dequeue(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            let mut head = head.borrow_mut();
            self.head = head.next.take();

            if self.len == 1 {
                self.tail = None;
            }

            self.len -= 1;
            return Some(std::mem::replace(&mut head.value, Default::default()));
        } else {
            return None;
        }
    }

    #[allow(dead_code)]
    pub fn peek(&self) -> Option<Ref<T>> {
        if let Some(head) = self.head.as_ref() {
            return Some(Ref::map(head.borrow(), |node| &node.value));
        } else {
            return None;
        }
    }
}
