use csv::ReaderBuilder;
use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use ndarray::{prelude::*, OwnedRepr};
use ndarray_csv::Array2Reader;
use plotlib::{
    grid::Grid,
    page::Page,
    repr::Plot,
    style::{PointMarker, PointStyle},
    view::{ContinuousView, View},
};
mod simple_regression;
mod random_lr;
mod gradient_descent;
mod simple_linear_reg_2;
use plotters::prelude::*;
use std::fs;
fn load_data (path:&str) ->Dataset<f64, &'static str>
{
    let mut reader = ReaderBuilder::new().delimiter(b',').from_path(path).expect("something went wrong");
    let array:Array2<f64> = reader.deserialize_array2_dynamic().expect("msg");
    let (data, targets) = (
        array.slice(s![.., 0..2]).to_owned(),
        array.column(2).to_owned(),
    );
    let feature_names = vec!["test 1", "test 2"];
    Dataset::new(data, targets)
        .map_targets(|x| {
            if *x as usize == 1 {
                "accepted"
            } else {
                "denied"
            }
        })
        .with_feature_names(feature_names)
}

fn plot_data(
    train: &DatasetBase<
        ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
        ArrayBase<OwnedRepr<&'static str>, Dim<[usize; 2]>>,
    >,
) {
    let mut positive = vec![];
    let mut negative = vec![];

    let records = train.records().clone().into_raw_vec();
    let features: Vec<&[f64]> = records.chunks(2).collect();
    let targets = train.targets().clone().into_raw_vec();
    for i in 0..features.len() {
        let feature = features.get(i).expect("feature exists");
        if let Some(&"accepted") = targets.get(i) {
            positive.push((feature[0], feature[1]));
        } else {
            negative.push((feature[0], feature[1]));
        }
    }

    let plot_positive = Plot::new(positive)
        .point_style(
            PointStyle::new()
                .size(2.0)
                .marker(PointMarker::Square)
                .colour("#00ff00"),
        )
        .legend("Exam Results".to_string());

    let plot_negative = Plot::new(negative).point_style(
        PointStyle::new()
            .size(2.0)
            .marker(PointMarker::Circle)
            .colour("#ff0000"),
    );

    let grid = Grid::new(0, 0);

    let mut image = ContinuousView::new()
        .add(plot_positive)
        .add(plot_negative)
        .x_range(0.0, 120.0)
        .y_range(0.0, 120.0)
        .x_label("Test 1")
        .y_label("Test 2");

    image.add_grid(grid);

    Page::single(&image)
        .save("plot.svg")
        .expect("can generate svg for plot");
}


//training the model 
fn iterate_with_values(
    train: &DatasetBase<
        ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
        ArrayBase<OwnedRepr<&'static str>, Dim<[usize; 2]>>,
    >,
    test: &DatasetBase<
        ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
        ArrayBase<OwnedRepr<&'static str>, Dim<[usize; 2]>>,
    >,
    threshold: f64,
    max_iterations: u64,
) -> ConfusionMatrix<&'static str> {
    let model = LogisticRegression::default()
        .max_iterations(max_iterations)
        .gradient_tolerance(0.0001)
        .fit(train)
        .expect("can train model");

    let validation = model.set_threshold(threshold).predict(test);

    let confusion_matrix = validation
        .confusion_matrix(test)
        .expect("can create confusion matrix");

    confusion_matrix
}
fn main()
{
    // let train = load_data("./dataset_test_1.csv");
    // let test = load_data("./dataset_train_1.csv");

    // let features = train.nfeatures();
    // let targets = train.ntargets();

    // println!(
    //     "training with {} samples, testing with {} samples, {} features and {} target",
    //     train.nsamples(),
    //     test.nsamples(),
    //     features,
    //     targets
    // );

    // println!("plotting data...");
    // plot_data(&train);


    // //
    // println!("training and testing model...");
    // let mut max_accuracy_confusion_matrix = iterate_with_values(&train, &test, 0.01, 100);
    // let mut best_threshold = 0.0;
    // let mut best_max_iterations = 0;
    // let mut threshold = 0.02;

    // for max_iterations in (1000..5000).step_by(500) {
    //     while threshold < 1.0 {
    //         let confusion_matrix = iterate_with_values(&train, &test, threshold, max_iterations);

    //         if confusion_matrix.accuracy() > max_accuracy_confusion_matrix.accuracy() {
    //             max_accuracy_confusion_matrix = confusion_matrix;
    //             best_threshold = threshold;
    //             best_max_iterations = max_iterations;
    //         }
    //         threshold += 0.01;
    //     }
    //     threshold = 0.02;
    // }

    // println!(
    //     "most accurate confusion matrix: {:?}",
    //     max_accuracy_confusion_matrix
    // );
    // println!(
    //     "with max_iterations: {}, threshold: {}",
    //     best_max_iterations, best_threshold
    // );
    // println!("accuracy {}", max_accuracy_confusion_matrix.accuracy(),);
    // println!("precision {}", max_accuracy_confusion_matrix.precision(),);
    // println!("recall {}", max_accuracy_confusion_matrix.recall(),);

    //simple linear reg 
    // simple_regression::simple_reg();

    //random data distribution
    //random_lr::random_data_linear_reg();
    let z = simple_linear_reg_2::generating_data(50, 5.0);







    
    //plottig the data:
    if !fs::metadata("./images").is_ok() {
        fs::create_dir("./images").unwrap();
    }
    
    let root_area = BitMapBackend::new("./images/2.6.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("scatter plot", ("sans-serif", 40))
        .build_cartesian_2d(-10.0f64..11.0f64, -30.0f64..50.0f64)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        z
            .iter()
            .map(|point:&(f64,f64)| Circle::new(*point, 5, &BLUE)),
    )
    .unwrap();
}