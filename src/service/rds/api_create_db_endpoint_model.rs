/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:39:54
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 10:44:45
 * @Description: api create db endpoint model
 */
use crate::volcengine::error::error;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::rds_endpoint;

/// Implements the `ApiRequest` trait for the `CreateDbEndpointReq` struct.
/// This implementation enables the `CreateDbEndpointReq` struct to be transformed
/// into a format appropriate for making API requests. Specifically, it offers
/// methods to convert the struct into a hashmap for query parameters and a byte
/// vector for the request body.
impl request::ApiRequest for rds_endpoint::CreateDbEndpointReq {
    /// Converts the `CreateDbEndpointReq` instance into a hashmap of string key - value pairs.
    /// This hashmap can be utilized to construct the query parameters for an API request.
    /// Currently, the implementation returns an empty hashmap since the actual
    /// formatting logic is commented out.
    ///
    /// # Returns
    /// - A `HashMap<String, String>` representing the formatted request data (empty in this case).
    fn to_hashmap(&self) -> HashMap<String, String> {
        // Uncomment this line to format the request into a hashmap.
        // request::Request::format_request_to_hashmap(self)
        return HashMap::new();
    }

    /// Converts the `CreateDbEndpointReq` instance into a byte vector representing the request body.
    /// It serializes the struct into a JSON byte vector. Note that if the serialization fails,
    /// the program will panic due to the use of `unwrap`.
    ///
    /// # Returns
    /// - A `Vec<u8>` representing the serialized JSON data of the request body.
    fn to_body(&self) -> Vec<u8> {
        return serde_json::to_vec(self).unwrap();
    }
}

/// Implements the `ApiResponse` trait for the `CreateDbEndpointResp` struct.
/// This implementation is in charge of converting an HTTP response into the
/// `CreateDbEndpointResp` struct. It parses the JSON response body and updates
/// the current instance with the parsed data.
impl response::ApiResponse for rds_endpoint::CreateDbEndpointResp {
    /// Converts an HTTP response into the `CreateDbEndpointResp` struct.
    /// First, it parses the JSON response body. Then, it updates the current
    /// `CreateDbEndpointResp` instance with the parsed data. If there is an error
    /// during parsing, it returns an `Error` struct.
    ///
    /// # Arguments
    /// - `&mut self`: A mutable reference to the `CreateDbEndpointResp` instance.
    /// - `http_response`: A `reqwest::Response` containing the HTTP response data.
    ///
    /// # Returns
    /// - On success, returns `Ok(())`.
    /// - On error, returns an `Error` struct indicating the reason for the failure,
    ///   such as a parsing error.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Parse the JSON response body into a `CreateDbEndpointResp` struct.
        // If the parsing fails, map the error to an `ErrParseResponse` error type.
        let parsed_response: volcengine_sdk_protobuf::protobuf::rds_endpoint::CreateDbEndpointResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?;

        // Update the current `CreateDbEndpointResp` instance with the parsed response data.
        *self = parsed_response;

        // Return `Ok(())` to indicate successful conversion of the HTTP response to the struct.
        return Ok(());
    }
}
