use rand::Rng; // Import random number generator

fn generate_data(n: usize) -> Vec<(f32, f32)> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();

    for _ in 0..n {
        let x: f32 = rng.gen_range(-10.0..10.0); // Random x between -10 and 10
        let noise: f32 = rng.gen_range(-1.0..1.0); // Small random noise
        let y = 2.0 * x + 1.0 + noise; // Apply function with noise
        data.push((x, y));
    }

    data
}

fn main() {
    let dataset = generate_data(100); // Generate 100 (x, y) points

    // Print first 10 data points
    for (x, y) in dataset.iter().take(10) {
        println!("x: {:.2}, y: {:.2}", x, y);
    }
}

