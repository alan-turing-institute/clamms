use tch::Tensor;

/// Expands a slice of Tensors as a batch.
pub fn encode_batch(v: &[Tensor]) -> Tensor {
    Tensor::stack(&v, 0)
}
