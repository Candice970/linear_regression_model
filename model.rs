//!
pub mod model;
use burn::{Module, Linear, MSELoss};
use burn::optim::{SGD, Optimizer};
use burn::tensor::{Tensor, TensorView};
use burn::ndarray::{Array, ArrayView};

struct LinearRegression {
    linear: Linear,
}

impl LinearRegression {
    fn new(input_dim: usize, output_dim: usize) -> Self {
        Self {
            linear: Linear::new(input_dim, output_dim),
        }
    }

    fn forward(&self, x: &Tensor) -> Tensor {
        self.linear.forward(x)
    }
}

impl Module for LinearRegression {}
