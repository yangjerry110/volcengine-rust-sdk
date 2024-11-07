/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 16:35:22
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-28 16:28:58
 * @Description: request
 */
use crate::volcengine::client::client_info;
use crate::volcengine::client::config;
use crate::volcengine::error::error;
use crate::volcengine::request::handles;
use crate::volcengine::request::operation;
use crate::volcengine::request::request;
use crate::volcengine::request::send;
use crate::volcengine::request::send::SendRequest;
use std::collections::HashMap;
use std::future::Future;

/// Trait representing an API request with methods to convert the request into a HashMap and a byte array.
pub trait ApiRequest: Send {
    fn to_hashmap(&self) -> HashMap<String, String>; // Converts the request to a HashMap.
    fn to_body(&self) -> Vec<u8>; // Converts the request to a byte array for sending.
}

/// Trait for handling requests to the Volcengine API.
pub trait RequestVolcengine {
    /// Formats the provided request into a HashMap.
    ///
    /// # Parameters
    /// - `request`: A reference to a serializable object.
    ///
    /// # Returns
    /// Returns a HashMap<String, String> representation of the request.
    fn format_request_to_hashmap<T: serde::Serialize>(request: &T) -> HashMap<String, String>;

    /// Sends an API request and returns the response.
    ///
    /// # Parameters
    /// - `request`: An instance implementing the ApiRequest trait.
    ///
    /// # Returns
    /// Returns a Result with the reqwest::Response or an error.
    fn send<T: request::ApiRequest>(
        &self,
        request: T,
    ) -> impl Future<Output = Result<reqwest::Response, error::Error>>;
}

/// Represents a request to the Volcengine API.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Request {
    pub config: config::Config,               // Configuration for the request.
    pub client_info: client_info::ClientInfo, // Information about the client making the request.
    pub handles: handles::Handles,            // Handles for managing request operations.
    pub operation: operation::Operation,      // Operation details for the request.
}

impl Request {
    /// Constructs a new RequestBuilder instance.
    pub fn builder() -> RequestBuilder {
        RequestBuilder {
            config: None,
            client_info: None,
            handles: None,
            operation: None,
        }
    }
}

impl RequestVolcengine for Request {
    /// Formats the request into a HashMap.
    ///
    /// # Parameters
    /// - `request`: A reference to a serializable object.
    ///
    /// # Returns
    /// Returns a HashMap<String, String> representation of the request, filtering out null or empty values.
    fn format_request_to_hashmap<T: serde::Serialize>(request: &T) -> HashMap<String, String> {
        // Serialize the request into a JSON value.
        let value = serde_json::to_value(request).unwrap();
        // Convert the JSON value into a map of key-value pairs.
        let map = value.as_object().unwrap();

        // Convert each entry from Value to String, filtering out empty or null values.
        map.iter()
            .filter_map(|(k, v)| {
                if v.is_null() || v == "" || v == "null" {
                    None // Skip null or empty values.
                } else {
                    Some((k.clone(), v.to_string().replace("\"", ""))) // Remove quotes and keep non-empty values.
                }
            })
            .collect()
    }

    /// Sends the request to the Volcengine API.
    ///
    /// # Parameters
    /// - `request`: An instance implementing the ApiRequest trait.
    ///
    /// # Returns
    /// Returns a Result containing the reqwest::Response or an error.
    async fn send<T: request::ApiRequest>(
        &self,
        request: T,
    ) -> Result<reqwest::Response, error::Error> {
        send::Send::set_request(self).send(&request).await
    }
}

/// Builder for constructing a Request instance.
pub struct RequestBuilder {
    pub config: Option<config::Config>, // Optional configuration for the request.
    pub client_info: Option<client_info::ClientInfo>, // Optional client information.
    pub handles: Option<handles::Handles>, // Optional handles for request operations.
    pub operation: Option<operation::Operation>, // Optional operation details.
}

impl RequestBuilder {
    /// Sets the configuration for the request builder.
    pub fn with_config(mut self, config: &config::Config) -> Self {
        self.config = Some(config.clone());
        self
    }

    /// Sets the client information for the request builder.
    pub fn with_client_info(mut self, client_info: &client_info::ClientInfo) -> Self {
        self.client_info = Some(client_info.clone());
        self
    }

    /// Sets the handles for the request builder.
    pub fn with_handles(mut self, handles: &handles::Handles) -> Self {
        self.handles = Some(handles.clone());
        self
    }

    /// Sets the operation details for the request builder.
    pub fn with_operation(mut self, operation: &operation::Operation) -> Self {
        self.operation = Some(operation.clone());
        self
    }

    /// Builds the final Request object.
    ///
    /// # Returns
    /// Returns a Result containing the constructed Request or an error if required fields are missing.
    pub fn build(self) -> Result<Request, error::Error> {
        // Check if config is provided.
        if self.config.is_none() {
            return Err(error::Error::ErrUtilRequestBuildRequestNo(
                "config".to_string(),
            ));
        }

        // Check if client_info is provided.
        if self.client_info.is_none() {
            return Err(error::Error::ErrUtilRequestBuildRequestNo(
                "client_info".to_string(),
            ));
        }

        // Check if handles are provided.
        if self.handles.is_none() {
            return Err(error::Error::ErrUtilRequestBuildRequestNo(
                "handles".to_string(),
            ));
        }

        // Check if operation is provided.
        if self.operation.is_none() {
            return Err(error::Error::ErrUtilRequestBuildRequestNo(
                "operation".to_string(),
            ));
        }

        // Return the constructed Request.
        Ok(Request {
            config: self.config.unwrap(),
            client_info: self.client_info.unwrap(),
            handles: self.handles.unwrap(),
            operation: self.operation.unwrap(),
        })
    }
}
