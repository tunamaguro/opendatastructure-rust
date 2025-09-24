use chapter1::{Deque, List};

use crate::Array;

pub struct ArrayDeque<T> {
    a: Array<T>,
    j: usize,
    n: usize,
}

impl<T> ArrayDeque<T> {
    pub fn with_capacity(len: usize) -> Self {
        ArrayDeque {
            a: Array::with_capacity(len),
            j: 0,
            n: 0,
        }
    }

    fn resize(&mut self) {
        let mut new_a = Array::with_capacity((self.size() * 2).max(1));
        let a_size = self.a.length();
        for k in 0..self.n {
            core::mem::swap(&mut self.a[(self.j + k) % a_size], &mut new_a[k]);
        }
        self.a = new_a;
        self.j = 0;
    }
}

impl<T> List<T> for ArrayDeque<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, i: usize) -> Option<&T> {
        if i < self.n {
            let a_size = self.a.length();
            self.a[(self.j + i) % a_size].as_ref()
        } else {
            None
        }
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        if i < self.n {
            let a_size = self.a.length();
            self.a[(self.j + i) % a_size].replace(x)
        } else {
            None
        }
    }

    fn add(&mut self, i: usize, x: T) -> Option<T> {
        if self.size() + 1 >= self.a.length() {
            self.resize();
        }

        // 範囲外
        if i > self.size() {
            return Some(x);
        }
        let a_size = self.a.length();
        if i < self.size() / 2 {
            // 0..iを左へシフト
            // jが0の時、aの後端へ移動させる。そうでなければ左に1つずらす
            self.j = if self.j == 0 { a_size - 1 } else { self.j - 1 };
            for k in 0..i {
                self.a
                    .swap((self.j + k) % a_size, (self.j + k + 1) % a_size);
            }
        } else {
            // i..nを右へシフト
            for k in (i..self.n).rev() {
                self.a
                    .swap((self.j + k) % a_size, (self.j + k + 1) % a_size);
            }
        };
        let y = self.a[(self.j + i) % a_size].replace(x);
        self.n += 1;
        y
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        // 範囲外
        if i >= self.size() {
            return None;
        };
        let a_size = self.a.length();
        let x = self.a[(self.j + i) % a_size].take();
        if i < self.size() / 2 {
            // 0..iを右へシフト
            for k in (0..i).rev() {
                self.a
                    .swap((self.j + k) % a_size, (self.j + k + 1) % a_size);
            }
            // jを1つ右へずらす
            self.j = (self.j + 1) % a_size;
        } else {
            // (i+1)..(n-1)を左へシフト
            for k in i..(self.size() - 1) {
                self.a
                    .swap((self.j + k) % a_size, (self.j + k + 1) % a_size);
            }
        };
        self.n -= 1;
        if 3 * self.size() < self.a.length() {
            self.resize();
        }
        x
    }
}

impl<T> Deque<T> for ArrayDeque<T> {
    fn add_front(&mut self, x: T) -> Option<T> {
        List::add(self, 0, x)
    }

    fn remove_front(&mut self) -> Option<T> {
        List::remove(self, 0)
    }

    fn add_back(&mut self, x: T) -> Option<T> {
        List::add(self, self.size(), x)
    }

    fn remove_back(&mut self) -> Option<T> {
        if self.size() == 0 {
            None
        } else {
            List::remove(self, self.size() - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation() {
        // Initialize
        let mut a = ArrayDeque::with_capacity(12);
        for (i, c) in "abcdefgh".chars().enumerate() {
            a.add(i, c);
        }

        // Remove
        let x = a.remove(2);
        assert_eq!(x, Some('c'));

        // Add
        a.add(4, 'x');
        a.add(3, 'y');
        a.add(3, 'z');

        // Check
        // 図では`abdyzexfgh`になっているが、`add(3,'z')`のときなぜか4個動き'z'が4個めに入っている
        // 記載のロジックだと'z'が3個めに入る`abdzyexfgh`が正しいと思われる
        for (i, c) in "abdzyexfgh".chars().enumerate() {
            let x = a.get(i);
            assert_eq!(x, Some(&c));
        }
    }
}
