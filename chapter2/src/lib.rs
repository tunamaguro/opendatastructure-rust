pub mod arraystack;

use std::ops::{Index, IndexMut};

struct Array<T> {
    a: Box<[Option<T>]>,
}

impl<T> Array<T> {
    fn with_capacity(len: usize) -> Self {
        let a = core::iter::repeat_with(|| None).collect::<Box<[_]>>();
        Self { a }
    }

    fn length(&self) -> usize {
        self.a.len()
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.a.swap(a, b);
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = Option<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.a[index]
    }
}

impl<T> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.a[index]
    }
}
