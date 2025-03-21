//! Worker module for connecting to a fractal computation server, retrieving tasks,
//! computing results, and sending them back.
//!
//! Each worker establishes a TCP connection to the server, requests tasks,
//! computes the pixel data, and submits the results continuously until instructed to stop.

use crate::connexion::Connection;
use crate::processor::process_task;

/// A worker responsible for connecting to the server, receiving tasks,
/// processing them, and returning the results.
///
/// Workers loop until the connection is terminated or an unrecoverable error occurs.
pub struct Worker {
    /// Server IP or hostname to connect to.
    pub server_address: String,
    /// A logical name to identify the connection or worker.
    pub connection_name: String,
    /// Default port on which the server listens for connections.
    pub default_port: u16,
    /// Flag used to break the run loop and gracefully terminate the worker.
    pub kill_connection: bool,
}

impl Worker {
    /// Creates a new `Worker` instance.
    ///
    /// # Arguments
    /// - `server_address`: The address of the server.
    /// - `connection_name`: A label for the worker (can be used for logging).
    /// - `default_port`: The port to connect to.
    pub fn new(server_address: String, connection_name: String, default_port: u16) -> Self {
        Self {
            server_address,
            connection_name,
            default_port,
            kill_connection: false,
        }
    }

    /// Starts the worker loop.
    ///
    /// The worker:
    /// 1. Connects to the server and sends the initial request.
    /// 2. Reads a `FragmentTask` from the server.
    /// 3. Processes the task.
    /// 4. Sends the result back to the server.
    /// 5. Repeats until `kill_connection` is set to true or a failure occurs.
    ///
    /// # Arguments
    /// - `request`: JSON-formatted `FragmentRequest` string to be sent to the server.
    pub fn run(&mut self, request: String) {
        while !self.kill_connection {
            match Connection::connect(&self.server_address, self.default_port) {
                Ok(mut stream) => {
                    if let Err(e) = stream.send_request(&request) {
                        eprintln!("Failed to send request: {}", e);
                        break;
                    }

                    let (mut task, mut id) = match stream.read_task() {
                        Ok(data) => data,
                        Err(e) => {
                            eprintln!("Failed to read task: {}", e);
                            break;
                        }
                    };

                    loop {
                        let mut task_stream = Connection::connect(&self.server_address, self.default_port)
                            .expect("Failed to reconnect to server");

                        let (result, pixels) = process_task(&task, &id);
                        if let Err(e) = task_stream.send_result(&result, &id, &pixels) {
                            eprintln!("Failed to send result: {}", e);
                            break;
                        }

                        match task_stream.read_task() {
                            Ok((new_task, new_id)) => {
                                task = new_task;
                                id = new_id;
                            }
                            Err(e) => {
                                eprintln!("Failed to get new task: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                    self.kill_connection = true;
                }
            }
        }
    }

    /// Signals the worker to stop after the current loop iteration.
    ///
    /// This is useful for gracefully terminating the worker from the outside.
    pub fn stop(&mut self) {
        self.kill_connection = true;
    }
}
