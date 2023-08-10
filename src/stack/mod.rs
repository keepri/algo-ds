// use std::{
//     cell::RefCell,
//     rc::Rc,
//     // sync::{Arc, Mutex},
// };
//
// struct SNode<T> {
//     value: T,
//     // single thread
//     prev: Option<Rc<RefCell<SNode<T>>>>,
//     // multi thread
//     // prev: Option<Arc<Mutex<SNode<T>>>>,
// }
//
// pub struct Stack<T> {
//     head: Option<Rc<RefCell<SNode<T>>>>,
//     tail: Option<Rc<RefCell<SNode<T>>>>,
//     len: usize,
// }
//
// impl<T> Stack<T> {
//     fn new() -> Self {
//         return Stack {
//             head: None,
//             tail: None,
//             len: 0,
//         };
//     }
//
//     fn len(&self) -> usize {
//         return self.len;
//     }
//
//     fn push(&mut self, item: T) -> () {
//         let node = Rc::new(RefCell::new(SNode {
//             value: item,
//             prev: None,
//         }));
//
//         if let Some(head) = self.head.take() {
//             head.borrow_mut().prev = Some(Rc::clone(&node));
//         } else {
//             self.tail = Some(Rc::clone(&node));
//         };
//
//         self.head = Some(node);
//         self.len += 1;
//     }
//
//     fn pop(&mut self) -> Option<T> {
//         if let Some(tail) = self.tail.take() {
//             let mut tail = tail.borrow_mut();
//             self.tail = tail.prev.take();
//
//             if self.len == 1 {
//                 self.head = self.tail.clone();
//             }
//
//             self.len -= 1;
//             return Some(tail.value);
//         } else {
//             return None;
//         };
//     }
//
//     fn peek(&self) -> Option<&T> {
//         if let Some(tail) = self.tail.as_ref() {
//             return Some(&tail.borrow().value);
//         } else {
//             return None;
//         }
//     }
// }
