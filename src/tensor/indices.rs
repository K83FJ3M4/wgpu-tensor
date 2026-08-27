use std::array::from_fn;

use crate::Shape;

pub trait IntoIndices {
    fn indices(self) -> impl Iterator<Item = usize>;
}

pub(crate) struct ShapeDiff {
    lhs: Shape,
    rhs: Shape
}

pub(crate) struct AllDimensions;

impl IntoIndices for AllDimensions {
    fn indices(self) -> impl Iterator<Item = usize> {
        let indices: [usize; 8] = from_fn(|i| i);
        indices.into_iter()
    }
}

impl ShapeDiff {
    pub(crate) fn new(lhs: Shape, rhs: Shape) -> ShapeDiff {
        ShapeDiff { lhs, rhs }
    }
}

impl IntoIndices for ShapeDiff {
    fn indices(self) -> impl Iterator<Item = usize> {
        self.lhs.into_iter()
        .zip(self.rhs)
        .enumerate()
        .filter_map(|(i, (lhs, rhs))| {
            if lhs != rhs {
                Some(i)
            } else {
                None
            }
        })
    }
}

impl IntoIndices for usize {
    fn indices(self) -> impl Iterator<Item = usize> {
        [self].into_iter()
    }
}

impl IntoIndices for (usize, ) {
    fn indices(self) -> impl Iterator<Item = usize> {
        [self.0].into_iter()
    }
}

impl IntoIndices for (usize, usize) {
    fn indices(self) -> impl Iterator<Item = usize> {
        [self.0, self.1].into_iter()
    }
}

impl IntoIndices for (usize, usize, usize) {
    fn indices(self) -> impl Iterator<Item = usize> {
        [self.0, self.1, self.2].into_iter()
    }
}

impl IntoIndices for (usize, usize, usize, usize) {
    fn indices(self) -> impl Iterator<Item = usize> {
        [self.0, self.1, self.2, self.3].into_iter()
    }
}

impl IntoIndices for (usize, usize, usize, usize, usize) {
    fn indices(self) -> impl Iterator<Item = usize> {
        [self.0, self.1, self.2, self.3, self.4].into_iter()
    }
}

impl IntoIndices for (usize, usize, usize, usize, usize, usize) {
    fn indices(self) -> impl Iterator<Item = usize> {
        [self.0, self.1, self.2, self.3, self.4, self.5].into_iter()
    }
}

impl IntoIndices for (usize, usize, usize, usize, usize, usize, usize) {
    fn indices(self) -> impl Iterator<Item = usize> {
        [self.0, self.1, self.2, self.3, self.4, self.5, self.6].into_iter()
    }
}

impl IntoIndices for (usize, usize, usize, usize, usize, usize, usize, usize) {
    fn indices(self) -> impl Iterator<Item = usize> {
        [self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7].into_iter()
    }
}
