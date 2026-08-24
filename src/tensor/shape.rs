pub type Shape = [u32; 8];

pub trait IntoShape {
    fn shape(self) -> Shape;
}

impl IntoShape for Shape {
    fn shape(self) -> Shape {
        self
    }
}

impl IntoShape for u32 {
    fn shape(self) -> Shape {
        let mut shape = Shape::default();
        shape.fill(1);
        shape[0] = self;
        shape
    }
}

impl IntoShape for (u32, ) {
    fn shape(self) -> Shape {
        let mut shape = Shape::default();
        shape.fill(1);
        shape[0] = self.0;
        shape
    }
}

impl IntoShape for (u32, u32) {
    fn shape(self) -> Shape {
        let mut shape = Shape::default();
        shape.fill(1);
        shape[0] = self.0;
        shape[1] = self.1;
        shape
    }
}

impl IntoShape for (u32, u32, u32) {
    fn shape(self) -> Shape {
        let mut shape = Shape::default();
        shape.fill(1);
        shape[0] = self.0;
        shape[1] = self.1;
        shape[2] = self.2;
        shape
    }
}

impl IntoShape for (u32, u32, u32, u32) {
    fn shape(self) -> Shape {
        let mut shape = Shape::default();
        shape.fill(1);
        shape[0] = self.0;
        shape[1] = self.1;
        shape[2] = self.2;
        shape[3] = self.3;
        shape
    }
}

impl IntoShape for (u32, u32, u32, u32, u32) {
    fn shape(self) -> Shape {
        let mut shape = Shape::default();
        shape.fill(1);
        shape[0] = self.0;
        shape[1] = self.1;
        shape[2] = self.2;
        shape[3] = self.3;
        shape[4] = self.4;
        shape
    }
}

impl IntoShape for (u32, u32, u32, u32, u32, u32) {
    fn shape(self) -> Shape {
        let mut shape = Shape::default();
        shape.fill(1);
        shape[0] = self.0;
        shape[1] = self.1;
        shape[2] = self.2;
        shape[3] = self.3;
        shape[4] = self.4;
        shape[5] = self.5;
        shape
    }
}

impl IntoShape for (u32, u32, u32, u32, u32, u32, u32) {
    fn shape(self) -> Shape {
        let mut shape = Shape::default();
        shape.fill(1);
        shape[0] = self.0;
        shape[1] = self.1;
        shape[2] = self.2;
        shape[3] = self.3;
        shape[4] = self.4;
        shape[5] = self.5;
        shape[6] = self.6;
        shape
    }
}

impl IntoShape for (u32, u32, u32, u32, u32, u32, u32, u32) {
    fn shape(self) -> Shape {
        let mut shape = Shape::default();
        shape.fill(1);
        shape[0] = self.0;
        shape[1] = self.1;
        shape[2] = self.2;
        shape[3] = self.3;
        shape[4] = self.4;
        shape[5] = self.5;
        shape[6] = self.6;
        shape[7] = self.7;
        shape
    }
}