use ndarray::Array1;
use rand::prelude::*;
use rand_distr::{Normal, Distribution};
use std::fs::File;
use std::io::{Write, BufWriter};
use plotters::prelude::*;

pub fn save_grid_to_csv(grid: &[Vec<f64>], b0_vals: &[f64], b1_vals: &[f64], filename: &str) {
    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);

    // Write the header
    write!(writer, "b0,b1,loss\n").unwrap();

    // Write the grid data
    for (i, b0) in b0_vals.iter().enumerate() {
        for (j, b1) in b1_vals.iter().enumerate() {
            let loss = grid[i][j];
            write!(writer, "{},{},{}\n", b0, b1, loss).unwrap();
        }
    }

}

pub fn random_data_linear_reg() {
    // Generating random data with normal distribution to simulate a linear regression

    // Create a random number generator
    let mut rng = rand::thread_rng();
 
    let normal_x = Normal::new(3.0, 5.0).unwrap();
    let x: Vec<f64> = (0..1000)
        .map(|_| normal_x.sample(&mut rng))
        .collect();

    let normal_noise = Normal::new(0.0, 3.0).unwrap();
    let y: Vec<f64> = x.iter()
        .map(|x_val| {
            -15.0 * x_val + 20.0 + normal_noise.sample(&mut rng)
        })
        .collect();

    let n = x.len();
    // Analytical solution
    let X: f64 = x.iter().sum();
    let Y: f64 = y.iter().sum();
    let mut b_1 = (n as f64 * dot_product(&x, &y) - X * Y) / (n as f64 * dot_product(&x, &x) - X * X);
    let mut b_0 = (Y - b_1 * X) / n as f64;

    // Rounding to two decimals
    b_1 = (b_1 * 100.0).round() / 100.0;
    b_0 = (b_0 * 100.0).round() / 100.0;

    // Calculate MSE loss (for debugging or visualization purposes)
    let mse = MSE_loss(b_1, b_0, &x, &y);
    println!("MSE Loss: {}", mse);

    // Call generate_grid with the correct arguments
    let b0_range = (b_0 - 10.0, b_0 + 10.0);
    let b1_range = (b_1 - 10.0, b_1 + 10.0);
    let grid_size = 100;
    let z = generate_grid(MSE_loss, &x, &y, b0_range, b1_range, grid_size);

    // Print or save the grid (optional)
    for row in z.iter() {
        for val in row.iter() {
            print!("{:.2} ", val);
        }
        // println!();
    }
    let z = generate_grid(MSE_loss, &x, &y, b0_range, b1_range, grid_size);

    // Save the grid to a CSV file
    save_grid_to_csv(&z, &b0_vals, &b1_vals, "mse_loss_grid.csv");

    // Create a 2D heatmap plot
    let root = BitMapBackend::new("mse_loss_heatmap.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("MSE Loss Heatmap", ("sans-serif", 40))
        .build_cartesian_2d(
            b0_range.0..b0_range.1,
            b1_range.0..b1_range.1,
        )
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("b0")
        .y_desc("b1")
        .draw()
        .unwrap();

    // Create a heatmap
    chart.draw_heatmap(&b0_vals, &b1_vals, &z).unwrap();
}

pub fn dot_product(arr1: &Vec<f64>, arr2: &Vec<f64>) -> f64 {
    let n = arr1.len();
    let mut sum: f64 = 0.0;
    for i in 0..n {
        sum += arr1[i] * arr2[i];
    }
    sum
}

pub fn MSE_loss(b_1: f64, b_0: f64, x: &[f64], y: &[f64]) -> f64 {
    let pred: Vec<f64> = x.iter().map(|&val| b_0 + b_1 * val).collect();
    let loss: f64 = pred.iter().zip(y.iter()).map(|(&p, &t)| (p - t).powi(2)).sum();
    loss / y.len() as f64
}

// Grids
pub fn generate_linspace(start: f64, end: f64, num_points: usize) -> Vec<f64> {
    let step = (end - start) / (num_points as f64 - 1.0);
    (0..num_points).map(|i| start + i as f64 * step).collect()
}

pub fn generate_grid<F>(MSE_loss: F, x: &[f64], y: &[f64], b0_range: (f64, f64), b1_range: (f64, f64), grid_size: usize) -> Vec<Vec<f64>>
where
    F: Fn(f64, f64, &[f64], &[f64]) -> f64,
{
    // Use Linspace::new to create an iterator and collect into a vector
    let b0_vals: Vec<f64> = Array1::linspace(b0_range.0, b0_range.1, grid_size).to_vec();
    let b1_vals: Vec<f64> = Array1::linspace(b1_range.0, b1_range.1, grid_size).to_vec();

    let mut z = vec![vec![0.0; b1_vals.len()]; b0_vals.len()];
    for (i, &b0) in b0_vals.iter().enumerate() {
        for (j, &b1) in b1_vals.iter().enumerate() {
            z[i][j] = MSE_loss(b0, b1, x, y).ln(); 
        }
    }

    z
}
