use rand::Rng;
use std::f64::consts::PI;

fn generate_data(num_samples: usize) -> Vec<(f64, f64)> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();

    for _ in 0..num_samples {
        let x = rng.gen_range(-10.0..10.0);
        let y = 2.0 * x + 1.0 + rng.gen_range(-1.0..1.0);
        data.push((x, y));
    }

    data
}
