use std::io::{self, Write};

pub trait TensorReader: Send + 'static {
    fn read(&mut self, data: &[u8]);
    fn finish(&mut self) {}
    fn error(&mut self) {}
}

#[derive(Default)]
pub struct PrintTensorReader {
    started: bool,
    has_values: bool
}

impl PrintTensorReader {
    pub fn new() -> PrintTensorReader {
        PrintTensorReader::default()
    }
}

impl TensorReader for PrintTensorReader {
    fn read(&mut self, data: &[u8]) {
        assert!(data.len().is_multiple_of(size_of::<f32>()));

        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        if !self.started {
            write!(stdout, "[").unwrap();
            self.started = true;
        }

        for bytes in data.chunks_exact(size_of::<f32>()) {
            if self.has_values {
                write!(stdout, ", ").unwrap();
            }

            let value = f32::from_ne_bytes(bytes.try_into().unwrap());
            write!(stdout, "{value:?}").unwrap();
            self.has_values = true;
        }
    }

    fn finish(&mut self) {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        if self.started {
            writeln!(stdout, "]").unwrap();
        } else {
            writeln!(stdout, "[]").unwrap();
        }
    }
}
