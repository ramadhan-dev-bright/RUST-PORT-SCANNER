use tokio::net::TcpStream;
use std::net::IpAddr;
use std::time::Duration;
use colored::*;
use futures::stream::{self, StreamExt};
use std::env; 

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let target_str = args.get(1).map(|s| s.as_str()).unwrap_or("127.0.0.1");
    let target: IpAddr = target_str.parse().expect("IP Address tidak valid!");

    let concurrency = 100; 
    let timeout = Duration::from_millis(200); 

    println!("{}", "========================================".cyan());
    println!("{}", "   [ NEURON PORT SCANNER RUST V20.2 ]".bold().white());
    println!("   TARGET: {} | MODE: PARALLEL", target.to_string().yellow());
    println!("{}", "========================================".cyan());
    
    let mut scan_results = stream::iter(1..1024)
        .map(|port| async move {
            if port % 100 == 0 {
                eprint!("\rScanning: {}%", (port as f32 / 1024.0 * 100.0) as i32);
            }
            match tokio::time::timeout(timeout, TcpStream::connect((target, port))).await {
                Ok(Ok(_)) => Some(port), 
                _ => None,
            }
        })
        .buffer_unordered(concurrency);
    let mut found = 0;
    let mut open_list = Vec::new();
    while let Some(port_res) = scan_results.next().await {
        if let Some(p) = port_res {
            println!("\r[{}] Port {} is {}", "✓".green(), p, "OPEN".green().bold());
            open_list.push(p);
            found += 1;
        }
    }
    println!("\n\n{}", ">>> SCAN COMPLETED <<<".yellow());
    println!("Summary: Found {} REAL open ports.", found);
    if !open_list.is_empty() {
        println!("Open Ports: {:?}", open_list);
    }
}
