use std::net::TcpStream;
use std::io::{Write, Read};
use anyhow::Result;

const BUFFER_SIZE: usize = 1024;

/// Measures reponse time:
/// 
/// # Args
/// 
/// * `host` - Host to test.
/// * `port` - Port to test.
///
/// # Returns
/// 
/// Result contains HEADERS, response content and response time.
pub fn measure_tcp_timings(host: &str, port: u16) -> Result<(Vec<u128>, Vec<u8>, Vec<u8>)> {
    let mut timings = Vec::new();

    let start_time = std::time::Instant::now();
    let mut stream = TcpStream::connect((host, port))?;
    let syn_time = start_time.elapsed().as_millis();
    timings.push(syn_time);
    println!("Response time to SYN request: {} ms", syn_time);

    let request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", host);
    let start_time = std::time::Instant::now();
    stream.write_all(request.as_bytes())?;
    let ack_time = start_time.elapsed().as_millis();
    timings.push(ack_time);
    println!("Response time to ACK request: {} ms", ack_time);

    let start_time = std::time::Instant::now();
    let response_headers = read_data(&mut stream, b"\r\n\r\n")?;
    let headers_time = start_time.elapsed().as_millis();
    timings.push(headers_time);
    println!("Response time to HEADERS request: {} ms", headers_time);

    let start_time = std::time::Instant::now();
    let response_content = read_data(&mut stream, b"")?;
    let content_time = start_time.elapsed().as_millis();
    timings.push(content_time);
    println!("Response time to content request: {} ms", content_time);

    let start_time = std::time::Instant::now();
    drop(stream);
    let fin_time = start_time.elapsed().as_millis();
    timings.push(fin_time);
    println!("Response time to connexion end: {} ms", fin_time);
    Ok((timings, response_content, response_headers))
}

fn read_data(stream: &mut TcpStream, terminator: &[u8]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    loop {
        let mut buf = [0; BUFFER_SIZE];
        let n = stream.read(&mut buf)?;
        if n == 0 {
            break;
        }
        data.extend_from_slice(&buf[..n]);
        if data.ends_with(terminator) {
            break;
        }
    }
    Ok(data)
}
