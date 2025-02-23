# Linear Regression Model in Rust using Burn

## Introduction
This project implements a simple **linear regression model** using the **Burn** library in Rust. The model predicts **y = 2x + 1** using synthetic data. The goal is to understand how to build and train a neural network in Rust using the Burn framework.

## Project Setup & Running the Code
### Prerequisites
Ensure you have the following installed:
- **Rust** (latest stable version) → [Install Rust](https://www.rust-lang.org/tools/install)
- **Cargo** (Rust package manager)
- **Burn 0.16.0** → The machine learning framework used
- **Git** → To push your code to GitHub

### Setup Instructions
1. **Clone the Repository**
   ```sh
   git clone <your-repository-url>
   cd <your-repository-folder>
   ```
2. **Ensure Dependencies are Installed**
   ```sh
   cargo check
   ```
3. **Run the Model**
   ```sh
   cargo run
   ```
4. **Check for Errors** (if any issues arise)
   ```sh
   cargo check
   ```

## Approach
### 1. Generating Synthetic Data
- Used the **rand** crate to generate random `x` values.
- Applied the equation **y = 2x + 1** with slight noise for realism.

### 2. Implementing the Model
- Used `Linear<NdArray>` from Burn to create a **single-layer** model.
- Implemented a `forward()` function to compute predictions.

### 3. Training the Model
- Used **Stochastic Gradient Descent (SGD)** as the optimizer.
- Minimized **Mean Squared Error (MSE)** as the loss function.
- Trained for **100 epochs**.

## Results & Evaluation
- The model successfully approximates **y = 2x + 1**.
- The loss decreases over time, showing successful learning.
- Printed output shows input and predicted values.
- I got more assistance from MetaAI  and Chatgpt

## Reflection on Learning
### Challenges Faced
- Encountered multiple errors with Burn’s API (e.g., `ndarray` import issues).
- Had to correctly initialize the **device** for computations.
- Ensured the Cargo.toml file remained unchanged per the assignment rules.

### AI and Documentation Assistance
- Used **Burn Documentation** extensively: [Burn 0.16.0 Docs](https://docs.rs/burn/0.16.0/burn/)
- Sought help from **Rust's official documentation**: [Rust Docs](https://doc.rust-lang.org/)
- Learned how to use **GitHub for version control**: [GitHub Guide](https://docs.github.com/en/get-started)

## Submission Instructions
1. **Push Code to GitHub**
   ```sh
   git add .
   git commit -m "Final submission"
   git push origin main
   ```
2. Ensure Cargo.toml remains unchanged.
3. Include this README.md file in the repository.
4. Submit the GitHub repository link on Blackboard LMS.

## Resources Used
- Rust Language: [Rust Docs](https://doc.rust-lang.org/)
- Burn Library: [Burn Docs](https://docs.rs/burn/0.16.0/burn/)
- Rust Rover: [Rust Rover Docs](https://www.jetbrains.com/help/rust/)
- GitHub: [GitHub Guide](https://docs.github.com/en/get-started)

---
### **Evaluation Criteria**
✔ **Correct Implementation**: Model correctly predicts **y = 2x + 1**
✔ **Code Quality**: Clean, structured, well-documented
✔ **README File**: Detailed, informative, reflective
✔ **LaTeX Writeup**: Clear, concise documentation
✔ **Adherence to Requirements**: **Cargo.toml remains unchanged**

