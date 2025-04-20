use std::cmp;

pub fn calculate_stats(data: &[i32]) -> [f64; 8] {
    if data.is_empty() {
        println!("No data provided.");
        return [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    }
    let mut sorted = data.to_owned();
    sorted.sort();

    let count = sorted.len();
    let min = sorted[0];
    let max = sorted[count - 1];

    // Helper for percentile (nearest-rank method)
    let percentile = |p: f64| -> f64 {
        let rank = (p * (count as f64 - 1.0)).round() as usize;
        sorted[cmp::min(rank, count - 1)] as f64
    };

    let p25 = percentile(0.25);
    let p50 = percentile(0.5);
    let p75 = percentile(0.75);

    let mean = data.iter().sum::<i32>() as f64 / count as f64;
    let variance = data
        .iter()
        .map(|&x| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / count as f64;
    let std = variance.sqrt();

    [
        count as f64,
        mean,
        std,
        min as f64,
        p25,
        p50,
        p75,
        max as f64,
    ]
}
