use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::time::Instant;
use anyhow::Result;

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

pub fn measure_tcp_timings(host: &str, port: u16) -> Result<(Vec<u128>, Vec<u8>, Vec<u8>), io::Error> {
    let mut timings = Vec::new();

    // Measure SYN time
    let start_time = Instant::now();
    let mut stream = TcpStream::connect((host, port))?;
    let syn_time = start_time.elapsed().as_millis();
    timings.push(syn_time);
    println!("Response time to SYN request: {} ms", syn_time);

    // Prepare and send the HTTP request
    let request = format!("GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", host);
    
    // Measure request time
    let start_time = Instant::now();
    stream.write_all(request.as_bytes())?;
    let request_time = start_time.elapsed().as_millis();
    timings.push(request_time);
    println!("Response time to ACK request: {} ms", request_time);

    // Measure response times
    let start_time = Instant::now();
    let mut response_headers = Vec::new();
    let mut buffer = vec![0; 1024];
    let mut read_bytes = 0;

    while let Ok(bytes) = stream.read(&mut buffer[read_bytes..]) {
        if bytes == 0 { break; }
        read_bytes += bytes;
        if response_headers.ends_with(b"\r\n\r\n") {
            break;
        }
    }
    
    response_headers.truncate(read_bytes);
    
    let response_time = start_time.elapsed().as_millis();
    timings.push(response_time);
    println!("Response time to headers content: {} ms", response_time);

    // Reading the content
    let mut response_content = Vec::new();
    let _ = stream.read_to_end(&mut response_content)?;

    // Measure connection end time
    let start_time = Instant::now();
    drop(stream);
    let fin_time = start_time.elapsed().as_millis();
    timings.push(fin_time);
    println!("Response time to connection end: {} ms", fin_time);

    Ok((timings, response_content, response_headers))
}