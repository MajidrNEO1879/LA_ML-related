// taken from mathematical proof runnig a simulation for a 3x + 8 with the noise of 5
use ndarray::prelude::*;
use ndarray::{Array2, Array1, ArrayView2};
use ndarray_linalg::Solve;
use rand::prelude::*;
use rand_distr::{Distribution, Normal};

pub fn generating_data(n: i32, noise_factor: f64) ->Vec<(f64, f64)> {
    let seed = 42_u64;
    let mut rng = StdRng::seed_from_u64(seed);

    let x = Array1::linspace(-10.0, 10.0, n as usize);
    let slope = 3.0;
    let intercept = 8.0;

    let noise = Array1::from(rand_n(n)) * noise_factor;

    let y = &(&x * slope) + intercept + &noise;
    let pairs: Vec<(f64, f64)> = x.iter().zip(y.iter()).map(|(&x_val, &y_val)| (x_val, y_val)).collect();
    pairs
}

fn rand_n(n: i32) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 1.0).unwrap();

    (0..n).map(|_| normal.sample(&mut rng)).collect()
}



//closed form
// fn add_bias_term(X: &Array2<f64>) -> Array2<f64> {
//     let ones = Array1::<f64>::ones(X.nrows());
//     let X_b = ndarray::stack![ndarray::Axis(1), ones.insert_axis(ndarray::Axis(1)), X.view()];
//     X_b
// }

// fn linear_regression_closed_form(X: &Array2<f64>, y: &Array1<f64>) -> Array1<f64> {
//     let X_b = add_bias_term(X);

//     let Xt = X_b.t();
//     let Xt_X = Xt.dot(&X_b);

//     let Xt_X_inv = Xt_X.solve_into().expect("Matrix inversion failed"); //here?

//     // compute X_b^T * y (use .view() to treat y as a view)
//     let Xt_y = Xt.dot(&y.view());

//     let w = Xt_X_inv.dot(&Xt_y);
    
//     w
// }

