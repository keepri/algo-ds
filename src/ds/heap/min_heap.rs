use std::{default::Default, fmt::Debug};

#[derive(Debug, Clone, Hash)]
pub struct MinHeap<T> {
    data: Vec<T>,
}

#[allow(dead_code)]
impl<T> MinHeap<T>
where
    T: Debug + Default + Ord,
{
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn insert(&mut self, value: T) -> () {
        self.data.push(value);
        self.heapify_up(self.data.len() - 1);
    }

    pub fn delete(&mut self) -> Option<T> {
        let len = self.data.len();
        return match len {
            0 | 1 => self.data.pop(),
            _ => {
                self.data.swap(0, len - 1);
                let value = self.data.pop();
                self.heapify_down(0);

                return value;
            }
        };
    }

    pub fn peek(&mut self) -> &T {
        return &self.data[0];
    }

    pub fn drop(&mut self) -> () {
        self.data.clear();
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

impl<T> Default for MinHeap<T>
where
    T: Debug + Default + Ord,
{
    fn default() -> Self {
        return Self { data: Vec::new() };
    }
}
