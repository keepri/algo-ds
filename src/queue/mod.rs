// use std::{
//     cell::RefCell,
//     // sync::{Arc, Mutex},
//     rc::Rc,
// };
//
// struct QNode<T> {
//     value: T,
//     // single thread
//     next: Option<Rc<RefCell<QNode<T>>>>,
//     // multi thread
//     // next: Option<Arc<Mutex<QNode<T>>>>,
// }
//
// pub struct Queue<T> {
//     head: Option<Rc<RefCell<QNode<T>>>>,
//     tail: Option<Rc<RefCell<QNode<T>>>>,
//     len: usize,
// }
//
// impl<T> Queue<T> {
//     fn new() -> Self {
//         return Queue {
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
//     fn enqueue(&mut self, item: T) -> () {
//         let node = Rc::new(RefCell::new(QNode {
//             value: item,
//             next: None,
//         }));
//
//         if let Some(tail) = self.tail.take() {
//             tail.borrow_mut().next = Some(Rc::clone(&node));
//         } else {
//             self.head = Some(Rc::clone(&node));
//         };
//
//         self.tail = Some(node);
//         self.len += 1;
//     }
//
//     fn dequeue(&mut self) -> Option<T> {
//         if let Some(head) = self.head.take() {
//             let mut head = head.borrow_mut();
//             self.head = head.next.take();
//
//             if self.len == 1 {
//                 self.tail = None;
//             }
//
//             self.len -= 1;
//             return Some(head.value);
//         } else {
//             return None;
//         }
//     }
//
//     fn peek(&mut self) -> Option<&T> {
//         if let Some(head) = self.head.as_ref() {
//             return Some(&head.borrow().value);
//         } else {
//             return None;
//         }
//     }
// }
