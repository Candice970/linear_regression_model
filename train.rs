//!
use burn::optim::SGD;
use burn::tensor::Tensor;
use burn::Module;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn train(model: &mut impl Module, x_train: &Tensor, y_train: &Tensor, learning_rate: f64, epochs: usize) {
    let mut optimizer = SGD::new(model.params(), learning_rate);

    for epoch in 0..epochs {
        let output = model.forward(x_train);
        let loss = (output - y_train).pow(2).mean();

        model.zero_grad();
        loss.backward();
        optimizer.step();

        println!("Epoch {}: Loss = {}", epoch + 1, loss);
    }
}
