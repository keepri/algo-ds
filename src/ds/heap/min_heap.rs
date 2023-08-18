use std::{default::Default, fmt::Debug};

#[derive(Debug, Clone)]
pub struct MinHeap<T> {
    data: Vec<T>,
}

impl<T: Debug + Default + Ord> MinHeap<T> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        return Self { data: Vec::new() };
    }

    #[allow(dead_code)]
    pub fn default() -> Self {
        return Self { data: Vec::new() };
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        return self.data.len();
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, value: T) -> () {
        self.data.push(value);
        self.heapify_up(self.data.len() - 1);
    }

    #[allow(dead_code)]
    pub fn delete(&mut self) -> Option<T> {
        let len = self.data.len();
        return match len {
            0 => None,
            1 => self.data.pop(),
            _ => {
                self.data.swap(0, len - 1);
                let value = self.data.pop();
                self.heapify_down(0);

                return value;
            }
        };
    }

    #[allow(dead_code)]
    pub fn peek(&mut self) -> &T {
        return &self.data[0];
    }

    #[allow(dead_code)]
    pub fn drop(&mut self) -> () {
        self.data = Vec::new();
    }

    fn heapify_up(&mut self, index: usize) -> () {
        if index == 0 {
            return;
        }

        let parent_idx = self.parent_idx(index);

        if self.data[index] < self.data[parent_idx] {
            self.data.swap(parent_idx, index);
            self.heapify_up(parent_idx);
        }
    }

    fn heapify_down(&mut self, index: usize) -> () {
        let len = self.data.len();

        if index >= len {
            return;
        }

        let left_idx = self.left_child_idx(index);
        if left_idx >= len {
            return;
        }

        let right_idx = self.right_child_idx(index);

        let mut smallest_child_idx = left_idx;
        if right_idx < len && self.data[right_idx] < self.data[left_idx] {
            smallest_child_idx = right_idx;
        }

        if self.data[smallest_child_idx] < self.data[index] {
            self.data.swap(smallest_child_idx, index);
            self.heapify_down(smallest_child_idx);
        }
    }

    fn parent_idx(&self, index: usize) -> usize {
        return (index - 1) / 2;
    }

    fn left_child_idx(&self, index: usize) -> usize {
        return 2 * index + 1;
    }

    fn right_child_idx(&self, index: usize) -> usize {
        return 2 * index + 2;
    }
}
