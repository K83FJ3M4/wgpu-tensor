pub trait IntoIndices {
    fn indices(self) -> impl Iterator<Item = usize>;
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