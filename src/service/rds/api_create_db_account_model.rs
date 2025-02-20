use crate::volcengine::error::error;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::rds_account;

/// Implements the `ApiRequest` trait for the `CreateDbAccountReq` struct.
/// This implementation allows the `CreateDbAccountReq` struct to be formatted
/// into a format suitable for making API requests, including converting it to
/// a hashmap for query parameters and a byte vector for the request body.
impl request::ApiRequest for rds_account::CreateDbAccountReq {
    /// Converts the `CreateDbAccountReq` instance into a hashmap of string key - value pairs.
    /// This hashmap can be used to construct query parameters for an API request.
    /// In this implementation, an empty hashmap is returned as the current logic
    /// comments out the actual formatting and simply creates a new, empty hashmap.
    ///
    /// # Returns
    /// - A `HashMap<String, String>` containing the formatted request data (empty in this case).
    fn to_hashmap(&self) -> HashMap<String, String> {
        // This line would format the request into a hashmap if uncommented.
        // request::Request::format_request_to_hashmap(self)
        HashMap::new()
    }

    /// Converts the `CreateDbAccountReq` instance into a byte vector representing the request body.
    /// It serializes the `CreateDbAccountReq` struct into a JSON byte vector.
    /// If the serialization fails, the program will panic as `unwrap` is used.
    ///
    /// # Returns
    /// - A `Vec<u8>` representing the serialized JSON data of the request body.
    fn to_body(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
        // This line would return an empty vector if uncommented.
        // Vec::new()
    }
}

/// Implements the `ApiResponse` trait for the `CreateDbAccountResp` struct.
/// This implementation is responsible for converting an HTTP response into the `CreateDbAccountResp` struct.
impl response::ApiResponse for rds_account::CreateDbAccountResp {
    /// Converts an HTTP response into the `CreateDbAccountResp` struct.
    /// It first parses the JSON response body, then updates the current `CreateDbAccountResp` instance
    /// with the parsed data. If there is an error during parsing, it returns an `Error` struct.
    ///
    /// # Arguments
    /// - `&mut self`: A mutable reference to the `CreateDbAccountResp` instance.
    /// - `http_response`: A `reqwest::Response` containing the HTTP response data.
    ///
    /// # Returns
    /// - On success, returns `Ok(())`.
    /// - On error, returns an `Error` struct indicating the reason for the failure,
    ///   such as a parsing error.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Parse the JSON response body into a `CreateDbAccountResp` struct.
        // If the parsing fails, map the error to an `ErrParseResponse` error type.
        let parsed_response: volcengine_sdk_protobuf::protobuf::rds_account::CreateDbAccountResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?;

        // Update the current `CreateDbAccountResp` instance with the parsed response data.
        *self = parsed_response;

        // Return `Ok(())` to indicate successful conversion of the HTTP response to the struct.
        Ok(())
    }
}
