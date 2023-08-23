use std::{
    cell::RefCell,
    rc::Rc,
    // sync::{Arc, Mutex},
};

#[allow(dead_code)]
struct LNode<T> {
    value: T,
    // single thread
    next: Option<Rc<RefCell<LNode<T>>>>,
    prev: Option<Rc<RefCell<LNode<T>>>>,
    // multi thread
    // next: Option<Arc<Mutex<LNode<T>>>>,
    // prev: Option<Arc<Mutex<LNode<T>>>>,
}

#[allow(dead_code)]
struct MyLinkedList<T> {
    head: Option<Rc<RefCell<LNode<T>>>>,
    tail: Option<Rc<RefCell<LNode<T>>>>,
    len: usize,
}

trait LinkedListTrait<T> {
    fn len(&self) -> usize;
    fn insert_at(&self, item: T, index: usize) -> ();
    fn remove(&self, item: T) -> Option<T>;
    fn remove_at(&self, index: usize) -> Option<T>;
    fn append(&self, item: T) -> ();
    fn prepend(&self, item: T) -> ();
    fn get(&self, index: usize) -> Option<T>;
}

#[allow(dead_code, unused_variables)]
impl<T> LinkedListTrait<T> for MyLinkedList<T> {
    fn len(&self) -> usize {
        return self.len;
    }

    fn insert_at(&self, item: T, index: usize) -> () {
        todo!()
    }

    fn remove(&self, item: T) -> Option<T> {
        todo!()
    }

    fn remove_at(&self, index: usize) -> Option<T> {
        todo!()
    }

    fn append(&self, item: T) -> () {
        todo!()
    }

    fn prepend(&self, item: T) -> () {
        todo!()
    }

    fn get(&self, index: usize) -> Option<T> {
        todo!()
    }
}
