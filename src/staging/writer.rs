use wgpu::WriteOnly;

pub trait TensorWriter {
    fn write(&mut self, chunk: WriteOnly<[u8]>);
    fn finish(&mut self) {}
}

impl<T: Iterator<Item = f32>> TensorWriter for T {
    fn write(&mut self, chunk: WriteOnly<[u8]>) {
        let len = chunk.len() / size_of::<f32>();
        chunk.write_iter(self.take(len).flat_map(f32::to_ne_bytes));
    }

    fn finish(&mut self) {
        assert!(
            self.next().is_none(),
            "iterator contains more elements than the tensor"
        );
    }
}
