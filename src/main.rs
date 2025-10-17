use clap::{Parser, Subcommand};
use anyhow::Result;
use tcp::measure_tcp_timings;
use analyze::detect_anomalies;
use time::OffsetDateTime;

mod tcp;
mod analyze;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CommandLineInterface {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Measure {
        /// Host to connect to
        #[arg(short = 'H', long)]
        host: String,

        /// Port to connect to
        #[arg(short, long)]
        port: u16,

        /// Number of times to repeat the measurement (optional)
        #[arg(short, long)]
        repeat: Option<usize>,

        /// Print response content
        #[arg(long)]
        content: bool,

        /// Print response headers
        #[arg(long)]
        headers: bool,
    },
}

fn main() -> Result<()> {
    let cli = CommandLineInterface::parse();

    let start = OffsetDateTime::now_utc();
    println!("Starting ttcp at {} UTC", start);

    match cli.command {
        Commands::Measure { host, port, repeat, content, headers } => {
            let mut all_timings = Vec::new();

            let repeat_count = repeat.unwrap_or(1);

            for _ in 0..repeat_count {
                let (timings, response_content, response_headers) = measure_tcp_timings(&host, port)?;
                all_timings.extend(timings);
                
                if headers {
                    analyze::print_headers(&response_headers);
                }
                if content {
                    analyze::print_content(&response_content);
                }
            }

            detect_anomalies(&all_timings);
        }
    }

    Ok(())
}
