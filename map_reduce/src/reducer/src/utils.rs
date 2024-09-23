pub fn mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

/// Функция для вычисления корреляции [ r = \frac{{n(\sum xy) - (\sum x)(\sum y)}}{{\sqrt{[n(\sum x^2) - (\sum x)^2][n(\sum y^2) - (\sum y)^2]}}} ]
pub fn correlation(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() != y.len() || x.len() == 0 {
        return None; // Массивы должны быть одинаковой длины и не пустыми
    }

    let n = x.len() as f64;
    
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();

    let sum_x_sq: f64 = x.iter().map(|&xi| xi * xi).sum();
    let sum_y_sq: f64 = y.iter().map(|&yi| yi * yi).sum();

    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(&xi, &yi)| xi * yi).sum();

    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x_sq - sum_x * sum_x) * (n * sum_y_sq - sum_y * sum_y)).sqrt();

    if denominator == 0.0 {
        return None; // Предотвращаем деление на 0
    }

    Some(numerator / denominator)
}