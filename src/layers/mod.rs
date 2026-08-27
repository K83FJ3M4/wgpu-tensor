use crate::{Tensor, TensorEncoder, TensorError};
pub use dense::Dense;

mod dense;

pub trait Layer {
    fn init(
        &self,
        encoder: &mut TensorEncoder,
    ) -> Result<(), TensorError>;

    fn forward<'scope>(
        &self,
        encoder: &mut TensorEncoder<'scope>,
        input: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError>;
}
