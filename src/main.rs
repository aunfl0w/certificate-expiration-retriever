use clap::Parser;
use native_tls::TlsConnector;
use std::process;
use std::{net::TcpStream, time::SystemTime};
use x509_parser::prelude::*;

/// Retrieve the expiration date of an X509 certificate from a host's certificate handshake.
/// Direct network connectivity to the host is required.
/// 
/// OUTPUT
/// host | days valid | expiration date 
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// FQDN of host
    #[clap(short, long)]
    host: String,
    #[clap(short, long, default_value_t = 443)]
    port: u16,
}

fn main() {
    let args = Cli::parse();
    let host_port = format!("{}:{}", args.host, args.port);

    let stream = match TcpStream::connect(host_port.clone()) {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("Host lookup error: {}", err.to_string());
            process::exit(1);
        }
    };

    let connector = match TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()
    {
        Ok(c) => c,
        Err(err) => {
            eprintln!("{}", err.to_string());
            process::exit(2);
        }
    };

    let stream_tls = match connector.connect(&host_port, stream) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Connection error: {}", err.to_string());
            process::exit(3);
        }
    };

    let der = stream_tls
        .peer_certificate()
        .unwrap()
        .unwrap()
        .to_der()
        .unwrap();

    let (_leftover, cert) = X509Certificate::from_der(&der).unwrap();
    let days = get_days_valid(cert.validity());
    println!(
        "{} | {:.2} | {} ",
        args.host,
        days,
        cert.validity().not_after.to_rfc2822()
    );
}

fn get_days_valid(validity: &Validity) -> f64 {
    let end = validity.not_after.timestamp() as u64;
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    (end as f64 - now as f64) / (3600.0 * 24.0)
}
