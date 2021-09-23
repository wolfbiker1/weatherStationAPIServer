pub mod trend_handler {
    pub fn calc_trend(dataset: &[f64]) -> Vec<f64> {
        let x_ges: f64 = (0..dataset.len() + 1).fold(0.0, |a, b| a as f64 + b as f64);
        let y_ges: f64 = dataset.iter().sum();
        let x_m = 1_f64 / dataset.len() as f64 * x_ges as f64;
        let y_m = 1_f64 / dataset.len() as f64 * y_ges as f64;

        let mut counter: f64 = 0.0;
        let mut denominator: f64 = 0.0;
        for (i, item) in dataset.iter().enumerate() {
            counter += (i as f64 - x_m) * (item - y_m);
            denominator += (i as f64 - x_m) * (i as f64 - x_m);
        }
        let m = counter / denominator;
        let b = y_m - m * x_m;
        vec![m, b]
    }
}
