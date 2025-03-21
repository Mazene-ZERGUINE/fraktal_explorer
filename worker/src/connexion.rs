//! TCP connection handler between worker and fractal computation server.
//!
//! This module handles low-level communication for sending requests, reading tasks,
//! and sending results using a custom protocol with prefixed size headers.

use std::io::{self, Read, Write};
use std::net::TcpStream;

use serde_json::Value;
use common::PixelIntensity;
use networking::{FragmentResult, FragmentTask};

/// A TCP connection wrapper that facilitates communication between a worker and the server.
///
/// This struct handles sending/receiving JSON-based messages with binary payloads
/// for distributed fractal rendering tasks.
pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    /// Establishes a new TCP connection to the given server address and port.
    ///
    /// # Arguments
    /// * `addr` - Server address (IP or hostname).
    /// * `port` - TCP port to connect to.
    ///
    /// # Returns
    /// An instance of `Connection` if successful.
    pub fn connect(addr: &str, port: u16) -> io::Result<Self> {
        let stream = TcpStream::connect(format!("{}:{}", addr, port))?;
        Ok(Self { stream })
    }

    /// Sends a JSON request (e.g., `FragmentRequest`) to the server.
    ///
    /// The message is prefixed with two 4-byte headers:
    /// - Total message size
    /// - JSON payload size
    ///
    /// # Arguments
    /// * `request` - A valid JSON string representing the request.
    ///
    /// # Errors
    /// Returns an error if the connection is broken or the write fails.
    pub fn send_request(&mut self, request: &str) -> io::Result<()> {
        let json_size = request.len() as u32;
        self.stream.write_all(&json_size.to_be_bytes())?;
        self.stream.write_all(&json_size.to_be_bytes())?;
        self.stream.write_all(request.as_bytes())?;
        Ok(())
    }

    /// Reads a task from the server, including its metadata and optional binary data.
    ///
    /// The expected format:
    /// - 4 bytes: total size (JSON + binary)
    /// - 4 bytes: JSON size
    /// - JSON payload (as string)
    /// - binary payload (pixel data, raw bytes)
    ///
    /// # Returns
    /// A tuple of `FragmentTask` and a raw binary buffer (usually unused data or input).
    ///
    /// # Errors
    /// Returns a string describing the failure cause (e.g., deserialization, I/O).
    pub fn read_task(&mut self) -> Result<(FragmentTask, Vec<u8>), String> {
        let mut total_size_buf = [0; 4];
        self.stream.read_exact(&mut total_size_buf).map_err(|e| e.to_string())?;
        let total_size = u32::from_be_bytes(total_size_buf) as usize;

        let mut json_size_buf = [0; 4];
        self.stream.read_exact(&mut json_size_buf).map_err(|e| e.to_string())?;
        let json_size = u32::from_be_bytes(json_size_buf) as usize;

        let mut json_buf = vec![0; json_size];
        self.stream.read_exact(&mut json_buf).map_err(|e| e.to_string())?;
        let json_str = String::from_utf8_lossy(&json_buf);

        let mut data_buf = vec![0; total_size - json_size];
        self.stream.read_exact(&mut data_buf).map_err(|e| e.to_string())?;

        let json_value: Value = serde_json::from_str(&json_str).map_err(|e| e.to_string())?;

        if let Some(task_value) = json_value.get("FragmentTask") {
            let task: FragmentTask = serde_json::from_value(task_value.clone()).map_err(|e| e.to_string())?;
            Ok((task, data_buf))
        } else {
            Err("Missing FragmentTask field".to_string())
        }
    }

    /// Sends a completed fractal computation result back to the server.
    ///
    /// The message structure is:
    /// - 4 bytes: total size (JSON + binary)
    /// - 4 bytes: JSON size
    /// - JSON payload
    /// - raw `id` bytes (identifier)
    /// - binary data for each pixel: 8 bytes per pixel (4 bytes `zn`, 4 bytes `count`)
    ///
    /// # Arguments
    /// * `result` - The metadata describing the result (range, resolution, etc.).
    /// * `id` - The raw task ID as byte slice.
    /// * `pixels` - The computed pixel intensity values.
    ///
    /// # Errors
    /// Returns an error if sending fails at any point.
    pub fn send_result(
        &mut self,
        result: &FragmentResult,
        id: &[u8],
        pixels: &[PixelIntensity],
    ) -> io::Result<()> {
        let json = result.to_json().expect("failed to serialize");
        let json_bytes = json.as_bytes();
        let json_len = json_bytes.len() as u32;
        let total_len = json_len + (id.len() + pixels.len() * 8) as u32;

        self.stream.write_all(&total_len.to_be_bytes())?;
        self.stream.write_all(&json_len.to_be_bytes())?;
        self.stream.write_all(json_bytes)?;
        self.stream.write_all(id)?;

        for pixel in pixels {
            self.stream.write_all(&pixel.zn.to_be_bytes())?;
            self.stream.write_all(&pixel.count.to_be_bytes())?;
        }

        Ok(())
    }
}
