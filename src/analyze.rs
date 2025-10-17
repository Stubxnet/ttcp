/// Anomaly detection alorithm (very bad, to improve absolutely)
/// Takes timings in entry.
pub fn detect_anomalies(timings: &[u128]) {
    if timings.is_empty() {
        println!("ERR: No response time to analyze.");
        return;
    }

    let mean: f64 = timings.iter().map(|&t| t as f64).sum::<f64>() / timings.len() as f64;
    let variance: f64 = timings.iter().map(|&t| {
        let diff = t as f64 - mean;
        diff * diff
    }).sum::<f64>() / timings.len() as f64;
    let stddev = variance.sqrt();

    println!("Response time average: {:.2} ms", mean);
    println!("Generic reponse time: {:.2} ms", stddev);

    let threshold = 3.0 * stddev;

    for &time in timings {
        if (time as f64 - mean).abs() > threshold {
            println!("Detected anomaly: {} ms", time);
        }
    }

    let min_time = *timings.iter().min().unwrap() as f64;
    let max_time = *timings.iter().max().unwrap() as f64;

    if max_time > mean + 5.0 * stddev {
        println!("Response time seems anormaly long with {} ms", max_time);
    }
    if min_time < mean - 5.0 * stddev {
        println!("Response time seems anormaly short with {} ms", min_time);
    }
}
/// Headers printing function.
/// 
/// Prints headers.
pub fn print_headers(response_headers: &[u8]) {
    println!("Received HEADERS (Raw):");
    println!("{}", String::from_utf8_lossy(response_headers));
}

/// Content printing function.
/// 
/// Prints received content (Raw).
pub fn print_content(response_content: &[u8]) {
    println!("Received content (Raw) :");
    println!("{}", String::from_utf8_lossy(response_content));
}
