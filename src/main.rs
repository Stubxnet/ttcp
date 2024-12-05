use clap::{Arg, Command};
use anyhow::Result;
use tcp::measure_tcp_timings;
use analyze::detect_anomalies;
use time::OffsetDateTime;

mod components::tcp;
mod components::analyze;

fn main() -> Result<()> {
    let matches = Command::new("DPIDetect")
        .version("1.0")
        .author("0l-bitly <prmail@ik.me>")
        .about("Mesure les temps de réponse TCP pour un hôte donné, afin de mesurer les performances et détecter des anomalies.")
        .arg(Arg::new("host")
            .help("L'hôte à tester")
            .required(true)
            .index(1))
        .arg(Arg::new("port")
            .help("Le port à utiliser (par défaut 80)")
            .required(false)
            .default_value("80")
            .index(2))
        .arg(Arg::new("options")
            .help("Options : 'ph' pour afficher les headers, 'pc' pour afficher le contenu.")
            .required(false)
            .index(3))
        .arg(Arg::new("repeat")
            .help("Le nombre de fois à répéter la mesure des temps. Si supérieure à 1, une moyenne pour chaque reprise sera affichée ainsi qu'une moyenne générale.")
            .required(false)
            .default_value("1")
            .index(4))
        .get_matches();

    let start = OffsetDateTime::now_utc();
    println!("Démarrage de ttcp à {} CEST", start);

    let host = matches.get_one::<String>("host").unwrap();
    let port: u16 = matches.get_one::<String>("port").unwrap().parse()?;
    let repeat: usize = matches.get_one::<String>("repeat").unwrap().parse()?;

    let mut all_timings = Vec::new();

    for _ in 0..repeat {
        let (timings, response_content, response_headers) = measure_tcp_timings(host, port)?;
        all_timings.extend(timings);
        
        if matches.contains_id("options") {
            let options = matches.get_one::<String>("options").unwrap();
            if options.contains("ph") {
                analyze::print_headers(&response_headers);
            }
            if options.contains("pc") {
                analyze::print_content(&response_content);
            }
        }
    }

    detect_anomalies(&all_timings);

    Ok(())
}
