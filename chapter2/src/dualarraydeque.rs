use chapter1::List;

use crate::{Array, arraystack::ArrayStack};

pub struct DualArrayDeque<T> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

impl<T> DualArrayDeque<T> {
    pub fn with_capacity(len: usize) -> Self {
        let n_front = len / 2;
        let n_back = len - n_front;
        DualArrayDeque {
            front: ArrayStack::with_capacity(n_front),
            back: ArrayStack::with_capacity(n_back),
        }
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut Option<T>> {
        if i < self.size() {
            let front_size = self.front.size();
            if i < front_size {
                self.front.get_mut(front_size - i - 1)
            } else {
                self.back.get_mut(i - front_size)
            }
        } else {
            None
        }
    }

    fn balance(&mut self) {
        let back_too_big = 3 * self.front.size() < self.back.size();
        let front_too_big = 3 * self.back.size() < self.front.size();
        if back_too_big || front_too_big {
            let n = self.front.size() + self.back.size();
            let n_front = n / 2;
            let mut new_front = Array::with_capacity((2 * n_front).max(1));
            for i in 0..n_front {
                if let Some(p_cur) = self.get_mut(i) {
                    core::mem::swap(&mut new_front[n_front - i - 1], p_cur);
                }
            }

            let n_back = n - n_front;
            let mut new_back = Array::with_capacity((2 * n_back).max(1));
            for i in 0..n_back {
                if let Some(p_cur) = self.get_mut(n_front + i) {
                    core::mem::swap(&mut new_back[i], p_cur);
                }
            }

            self.front.a = new_front;
            self.front.n = n_front;
            self.back.a = new_back;
            self.back.n = n_back;
        }
    }
}

impl<T> List<T> for DualArrayDeque<T> {
    fn size(&self) -> usize {
        self.front.size() + self.back.size()
    }

    fn get(&self, i: usize) -> Option<&T> {
        if i < self.size() {
            let front_size = self.front.size();
            if i < front_size {
                self.front.get(front_size - i - 1)
            } else {
                self.back.get(i - front_size)
            }
        } else {
            None
        }
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        if i < self.size() {
            let front_size = self.front.size();
            if i < front_size {
                self.front.set(front_size - i - 1, x)
            } else {
                self.back.set(i - front_size, x)
            }
        } else {
            Some(x)
        }
    }

    fn add(&mut self, i: usize, x: T) -> Option<T> {
        if i > self.size() {
            return Some(x);
        }

        let y = if i < self.front.size() {
            self.front.add(self.front.size() - i, x)
        } else {
            self.back.add(i - self.front.size(), x)
        };
        self.balance();
        y
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if i >= self.size() {
            return None;
        }

        let x = if i < self.front.size() {
            self.front.remove(self.front.size() - i - 1)
        } else {
            self.back.remove(i - self.front.size())
        };
        self.balance();
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation() {
        // Initialize
        let mut a = DualArrayDeque::with_capacity(10);
        for (i, c) in "abcd".chars().enumerate() {
            a.add(i, c);
        }

        // Add
        a.add(3, 'x');
        a.add(4, 'y');

        // Remove
        let x = a.remove(0);
        assert_eq!(x, Some('a'));

        // Check
        for (i, c) in "bcxyd".chars().enumerate() {
            let x = a.get(i);
            assert_eq!(x, Some(&c));
        }
    }
}
