use chapter1::{List, Queue};

use crate::arraystack::ArrayStack;

pub struct RandomQueue<T> {
    queue: ArrayStack<T>,
}

impl<T> RandomQueue<T> {
    pub fn with_capacity(len: usize) -> Self {
        Self {
            queue: ArrayStack::with_capacity(len),
        }
    }
}

impl<T> Queue<T> for RandomQueue<T> {
    fn add(&mut self, x: T) -> Option<T> {
        Queue::add(&mut self.queue, x)
    }

    fn remove(&mut self) -> Option<T> {
        let queue_size = self.queue.size();
        if queue_size == 0 {
            return None;
        }

        let i = fastrand::usize(0..queue_size);

        // 末尾とスワップする
        let x = self.queue.a[i].take();
        let last_idx = queue_size - 1;
        if i != last_idx {
            self.queue.a.swap(i, last_idx);
        }
        self.queue.n -= 1;
        if self.queue.a.length() >= 3 * self.queue.n {
            self.queue.resize();
        }

        x
    }
}
