use chapter1::List;

use crate::{Array, arraystack::ArrayStack};

pub struct RootishArrayStack<T> {
    blocks: ArrayStack<Array<T>>,
    n: usize,
}

impl<T> RootishArrayStack<T> {
    pub fn with_capacity() -> Self {
        Self {
            blocks: ArrayStack::with_capacity(0),
            n: 0,
        }
    }

    /// インデックス`i`が属するブロックのインデックスを返す
    fn i2b(i: usize) -> usize {
        let sq = 9 + 8 * i;
        let db = (-3.0 + (sq as f64).sqrt()) / 2.0;
        db.ceil() as usize
    }

    /// インデックス`i`が属するブロックのインデックスと、そのブロック内でのインデックスを返す
    fn i2bj(i: usize) -> (usize, usize) {
        let b = Self::i2b(i);
        let j = i - b * (b + 1) / 2;
        (b, j)
    }

    fn max_size(&self) -> usize {
        let r = self.blocks.size();
        r * (r + 1) / 2
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut Option<T>> {
        let (block_idx, j) = Self::i2bj(i);
        match self.blocks.get_mut(block_idx) {
            Some(Some(arr)) => Some(&mut arr[j]),
            _ => None,
        }
    }

    fn grow(&mut self) {
        let block_size = self.blocks.size();

        self.blocks
            .add(block_size, Array::with_capacity(block_size + 1));
    }

    fn shrink(&mut self) {
        let mut r = self.blocks.size();
        // (r-1)*r / 2 はブロックを1つ消した時の最大容量
        while r > 0 && (r - 1) * r / 2 >= self.size() {
            self.blocks.remove(r - 1);
            r -= 1;
        }
    }
}

impl<T> List<T> for RootishArrayStack<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, i: usize) -> Option<&T> {
        let (block_idx, j) = Self::i2bj(i);

        self.blocks
            .get(block_idx)
            .and_then(|block| block[j].as_ref())
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        let (block_idx, j) = Self::i2bj(i);
        match self.blocks.get_mut(block_idx) {
            Some(Some(arr)) => arr[j].replace(x),
            _ => Some(x),
        }
    }

    fn add(&mut self, i: usize, x: T) -> Option<T> {
        if self.max_size() < self.size() + 1 {
            self.grow();
        }
        self.n += 1;
        for j in (i..(self.size() - 1)).rev() {
            // Swap j and j+1 to shift right
            // 1度に2つ可変参照をsafeなRustでは取れないのでこうなっている
            let a = self.get_mut(j).and_then(|v| v.take());
            match self.get_mut(j + 1) {
                Some(dst) => {
                    *dst = a;
                }
                None => {}
            }
        }
        self.set(i, x)
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let x = match self.get_mut(i) {
            Some(val) => val.take(),
            None => return None,
        };
        for j in i..(self.size() - 1) {
            // Swap j and j+1 to shift left
            let a = self.get_mut(j + 1).and_then(|v| v.take());
            match self.get_mut(j) {
                Some(dst) => {
                    *dst = a;
                }
                None => {}
            }
        }
        self.n -= 1;

        self.shrink();
        x
    }
}

#[cfg(test)]
mod tests {
    use chapter1::List;

    #[test]
    fn operation() {
        // Initialize
        let mut a = super::RootishArrayStack::with_capacity();
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
