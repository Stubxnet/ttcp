/// Algorithme de détection d'anomalies de ttcp
/// 
/// Il utilise un système de détection d'anomalie basée sur l'analyse des temps de réponse.
/// Fait aussi une moyenne et un écart type.
pub fn detect_anomalies(timings: &[u128]) {
    if timings.is_empty() {
        println!("Aucun temps de réponse à analyser.");
        return;
    }

    let mean: f64 = timings.iter().map(|&t| t as f64).sum::<f64>() / timings.len() as f64;
    let variance: f64 = timings.iter().map(|&t| {
        let diff = t as f64 - mean;
        diff * diff
    }).sum::<f64>() / timings.len() as f64;
    let stddev = variance.sqrt();

    println!("Moyenne des temps de réponse: {:.2} ms", mean);
    println!("Écart type des temps de réponse: {:.2} ms", stddev);

    let threshold = 3.0 * stddev;

    for &time in timings {
        if (time as f64 - mean).abs() > threshold {
            println!("Anomalie détectée: {} ms", time);
        }
    }

    let min_time = *timings.iter().min().unwrap() as f64;
    let max_time = *timings.iter().max().unwrap() as f64;

    if max_time > mean + 5.0 * stddev {
        println!("Anomalie potentielle détectée: temps de réponse exceptionnellement long: {} ms", max_time);
    }
    if min_time < mean - 5.0 * stddev {
        println!("Anomalie potentielle détectée: temps de réponse exceptionnellement court: {} ms", min_time);
    }
}
/// Fonction pour afficher les headers
/// 
/// Affiche les en-têtes reçus
pub fn print_headers(response_headers: &[u8]) {
    println!("En-têtes reçus :");
    println!("{}", String::from_utf8_lossy(response_headers));
}

/// Fonction pour afficher le contenu
/// 
/// Affiche le contenu reçu
pub fn print_content(response_content: &[u8]) {
    println!("Contenu reçu :");
    println!("{}", String::from_utf8_lossy(response_content));
}
