pub fn mean(xs: &[f64]) -> f64 {
    return xs.iter().sum::<f64>() / xs.len() as f64;
}