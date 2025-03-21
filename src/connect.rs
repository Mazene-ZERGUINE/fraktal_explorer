use std::net::Ipv4Addr;

use networking::FragmentRequest;
use worker::Worker;
use crate::cli::read_user_input;

pub fn handle_connect_command(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    let args = cmd.trim_start_matches("worker connect ").trim();
    let parts: Vec<&str> = args.split(':').collect();

    if parts.len() != 2 {
        println!("Invalid format. Use: worker connect <ip>:<port>");
        return Ok(());
    }

    let ip = parts[0];
    let port = parts[1];

    if ip != "localhost" && ip.parse::<Ipv4Addr>().is_err() {
        println!("Invalid IP address.");
        return Ok(());
    }

    start_worker(ip, port)
}

fn start_worker(ip: &str, port: &str) -> Result<(), Box<dyn std::error::Error>> {
    let connection_name = read_user_input("Enter connection name: ");
    let workload = read_user_input("Enter max workload: ");

    let workload = workload
        .parse::<u32>()
        .map_err(|_| "Invalid number for workload")?;

    let mut worker = Worker::new(ip.to_string(), connection_name.clone(), port.parse()?);
    let request = FragmentRequest::builder()
        .with_worker_name(connection_name)
        .with_max_work_load(workload)
        .build()?;

    let request_str = request.to_json()?;
    worker.run(request_str);
    Ok(())
}
