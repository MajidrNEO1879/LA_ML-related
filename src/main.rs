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


fn main()
{
    let train = load_data("./dataset_test_1.csv");
    let test = load_data("./dataset_train_1.csv");

    let features = train.nfeatures();
    let targets = train.ntargets();

    println!(
        "training with {} samples, testing with {} samples, {} features and {} target",
        train.nsamples(),
        test.nsamples(),
        features,
        targets
    );

    println!("plotting data...");
    plot_data(&train);
}