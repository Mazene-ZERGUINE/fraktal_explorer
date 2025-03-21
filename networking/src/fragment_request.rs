//! Fragment request module for managing task distribution across workers.
//!
//! This module includes a data structure to define a fragment request sent by
//! a worker and a builder pattern to construct it safely.

use serde::{Deserialize, Serialize};
use serde_json::json;

/// A request from a worker asking for a workload to compute.
///
/// This structure is typically sent to a task manager/coordinator
/// to request a portion of work (fragment) that can be processed.
///
/// # Fields
/// - `worker_name`: Name or identifier of the requesting worker.
/// - `maximal_work_load`: The maximum number of tasks this worker is willing to handle.
#[derive(Debug, Serialize, Deserialize)]
pub struct FragmentRequest {
    worker_name: String,
    maximal_work_load: u32,
}

/// A builder for safely constructing a [`FragmentRequest`].
///
/// Ensures required fields are set before creating the final request.
pub struct FragmentRequestBuilder {
    worker_name: Option<String>,
    max_work_load: Option<u32>,
}

impl FragmentRequest {
    /// Creates a new `FragmentRequest` from its raw fields.
    /// You should prefer using [`FragmentRequest::builder()`] in most cases.
    fn new(worker_name: String, maximal_work_load: u32) -> FragmentRequest {
        FragmentRequest {
            worker_name,
            maximal_work_load,
        }
    }

    /// Initializes a new builder for creating a `FragmentRequest`.
    ///
    /// # Example
    /// ```
    /// use networking::FragmentRequest;
    /// let request = FragmentRequest::builder()
    ///     .with_worker_name("worker-1".to_string())
    ///     .with_max_work_load(10)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> FragmentRequestBuilder {
        FragmentRequestBuilder {
            worker_name: None,
            max_work_load: None,
        }
    }

    /// Serializes the request to a JSON string wrapped inside a `FragmentRequest` key.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&json!({"FragmentRequest": self}))
    }
}

impl FragmentRequestBuilder {
    /// Sets the name of the worker requesting work.
    pub fn with_worker_name(mut self, worker_name: String) -> Self {
        self.worker_name = Some(worker_name);
        self
    }

    /// Sets the maximum workload the worker is capable of handling.
    pub fn with_max_work_load(mut self, max_work_load: u32) -> Self {
        self.max_work_load = Some(max_work_load);
        self
    }

    /// Builds the [`FragmentRequest`] if all required fields are set.
    ///
    /// # Errors
    /// Returns an error string if either `worker_name` or `max_work_load` is missing.
    pub fn build(self) -> Result<FragmentRequest, &'static str> {
        let worker_name = self.worker_name.ok_or("Worker name is missing")?;
        let max_work_load = self.max_work_load.ok_or("Max work load is missing")?;

        Ok(FragmentRequest::new(worker_name, max_work_load))
    }
}
