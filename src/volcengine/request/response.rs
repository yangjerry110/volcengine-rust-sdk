/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-21 18:19:47
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-08 10:49:53
 * @Description: response module for handling API responses
 */
use crate::volcengine::error::error;
use serde::{Deserialize, Serialize};
use std::future::Future;

/// Trait to handle API responses
///
/// This trait defines the method to convert an HTTP response into a struct.
pub trait ApiResponse {
    /// Converts the given `reqwest::Response` into a struct implementing `ApiResponse`
    ///
    /// # Arguments
    /// - `http_response`: The HTTP response received from an API request
    ///
    /// # Returns
    /// Returns a `Future` that resolves to a `Result` indicating success or failure.
    fn to_struct(
        &mut self,
        http_response: reqwest::Response,
    ) -> impl Future<Output = Result<(), error::Error>>;
}

/// Metadata structure for API responses
///
/// This struct holds essential metadata details from the API response.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ApiResponseMetadata {
    /// Unique request ID assigned to the request
    #[serde(rename = "RequestId")]
    pub request_id: String,

    /// The name of the action performed by the API
    #[serde(rename = "Action")]
    pub action: String,

    /// The version of the API used for the request
    #[serde(rename = "Version")]
    pub version: String,

    /// The service that processed the request
    #[serde(rename = "Service")]
    pub service: String,

    /// The region where the request was processed
    #[serde(rename = "Region")]
    pub region: String,

    /// Optional error data if the request failed
    #[serde(rename = "Error")]
    pub error: Option<ApiResponseMetadataErrData>,
}

/// Structure to hold error details in API responses
///
/// This struct captures error details such as error code and message.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ApiResponseMetadataErrData {
    /// Numeric error code (if available)
    #[serde(rename = "CodeN")]
    pub code_no: Option<i64>,

    /// Error code string
    #[serde(rename = "Code")]
    pub code: String,

    /// Error message describing the failure
    #[serde(rename = "Message")]
    pub message: String,
}
