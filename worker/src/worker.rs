use crate::connexion::Connection;
use crate::processor::process_task;

pub struct Worker {
    pub server_address: String,
    pub connection_name: String,
    pub default_port: u16,
    pub kill_connection: bool,
}

impl Worker {
    pub fn new(server_address: String, connection_name: String, default_port: u16) -> Self {
        Self {
            server_address,
            connection_name,
            default_port,
            kill_connection: false,
        }
    }

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

    pub fn stop(&mut self) {
        self.kill_connection = true;
    }
}
