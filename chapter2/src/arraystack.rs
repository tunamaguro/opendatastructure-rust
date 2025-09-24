use chapter1::{List, Stack};

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

        if i <= self.n {
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

impl<T> Stack<T> for ArrayStack<T> {
    fn push(&mut self, x: T) -> Option<T> {
        List::add(self, self.size(), x)
    }

    fn pop(&mut self) -> Option<T> {
        self.remove(self.size() - 1)
    }
}

#[cfg(test)]
mod tests {
    use chapter1::List;

    #[test]
    fn operation() {
        // Initialize
        let mut a = super::ArrayStack::with_capacity(6);
        a.add(0, 'b');
        a.add(1, 'r');
        a.add(2, 'e');
        a.add(3, 'd');

        // Add
        a.add(2, 'e');
        a.add(5, 'r');
        a.add(5, 'e');

        // remove
        let x = a.remove(4);
        assert_eq!(x, Some('d'));
        let x = a.remove(4);
        assert_eq!(x, Some('e'));
        let x = a.remove(4);
        assert_eq!(x, Some('r'));

        // set
        a.set(2, 'i');

        // check
        assert_eq!(a.get(0), Some(&'b'));
        assert_eq!(a.get(1), Some(&'r'));
        assert_eq!(a.get(2), Some(&'i'));
        assert_eq!(a.get(3), Some(&'e'));
    }
}
