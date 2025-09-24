use chapter1::Queue;

use crate::Array;

pub struct ArrayQueue<T> {
    a: Array<T>,
    j: usize,
    n: usize,
}

impl<T> ArrayQueue<T> {
    pub fn with_capacity(len: usize) -> Self {
        Self {
            a: Array::with_capacity(len),
            j: 0,
            n: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    fn resize(&mut self) {
        let mut new_a = Array::with_capacity((self.n * 2).max(1));
        let a_size = self.a.length();
        for k in 0..self.n {
            core::mem::swap(&mut self.a[(self.j + k) % a_size], &mut new_a[k]);
        }
        self.a = new_a;
        self.j = 0;
    }
}

impl<T> Queue<T> for ArrayQueue<T> {
    fn add(&mut self, x: T) -> Option<T> {
        let a_size = self.a.length();
        if self.size() + 1 >= a_size {
            self.resize();
        };

        let y = self.a[(self.j + self.n) % a_size].replace(x);
        self.n += 1;
        y
    }

    fn remove(&mut self) -> Option<T> {
        let x = self.a[self.j].take();
        self.j = (self.j + 1) % self.a.length();
        self.n -= 1;
        if self.a.length() >= 3 * self.n {
            self.resize();
        }
        x
    }
}
