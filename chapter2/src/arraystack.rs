use chapter1::List;

use super::Array;

pub struct ArrayStack<T> {
    a: Array<T>,
    n: usize,
}

impl<T> ArrayStack<T> {
    pub fn with_capacity(len: usize) -> Self {
        let a = Array::with_capacity(len);
        Self { a, n: 0 }
    }

    fn resize(&mut self) {
        let mut new_a = Array::with_capacity((2 * self.n).max(1));
        for i in 0..self.n {
            core::mem::swap(&mut self.a[i], &mut new_a[i]);
        }
        self.a = new_a;
    }
}

impl<T> List<T> for ArrayStack<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, i: usize) -> Option<&T> {
        if i < self.n { self.a[i].as_ref() } else { None }
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        if i < self.n {
            self.a[i].replace(x)
        } else {
            Some(x)
        }
    }

    fn add(&mut self, i: usize, x: T) -> Option<T> {
        if self.n + 1 > self.a.length() {
            self.resize();
        }

        if i < self.n {
            // i..nを右に1つずらす
            for k in (i..self.n).rev() {
                self.a.swap(k, k + 1);
            }
            let y = self.a[i].replace(x);
            self.n += 1;
            y
        } else {
            Some(x)
        }
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if i < self.n {
            let x = self.a[i].take();
            for k in i..self.n {
                self.a.swap(k, k + 1);
            }
            self.n -= 1;
            if self.a.length() >= 3 * self.n {
                self.resize();
            }
            x
        } else {
            None
        }
    }
}
