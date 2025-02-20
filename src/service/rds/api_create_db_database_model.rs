/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:39:54
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 10:32:09
 * @Description:
 */
use crate::volcengine::error::error;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::rds_database;

/// Implements the `ApiRequest` trait for the `CreateDatabaseReq` struct.
/// This allows the `CreateDatabaseReq` struct to be formatted into a format
/// suitable for making API requests, such as converting it to a hashmap for
/// query parameters and a byte vector for the request body.
impl request::ApiRequest for rds_database::CreateDatabaseReq {
    /// Converts the `CreateDatabaseReq` instance into a hashmap of string key - value pairs.
    /// This hashmap can be used to construct query parameters for an API request.
    /// Currently, the implementation returns an empty hashmap as the actual formatting
    /// logic is commented out.
    ///
    /// # Returns
    /// - A `HashMap<String, String>` representing the request data in key - value pairs (empty here).
    fn to_hashmap(&self) -> HashMap<String, String> {
        // This line would format the request into a hashmap if uncommented.
        // request::Request::format_request_to_hashmap(self)
        HashMap::new()
    }

    /// Converts the `CreateDatabaseReq` instance into a byte vector representing the request body.
    /// It serializes the struct into a JSON byte vector. If serialization fails,
    /// the program will panic because `unwrap` is used.
    ///
    /// # Returns
    /// - A `Vec<u8>` representing the serialized JSON data of the request body.
    fn to_body(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

/// Implements the `ApiResponse` trait for the `CreateDatabaseResp` struct.
/// This is responsible for converting an HTTP response into the `CreateDatabaseResp` struct
/// and handling error cases by updating the response metadata if the request was not successful.
impl response::ApiResponse for rds_database::CreateDatabaseResp {
    /// Converts an HTTP response into the `CreateDatabaseResp` struct.
    /// It first retrieves the HTTP status code, parses the JSON response body,
    /// updates the current `CreateDatabaseResp` instance with the parsed data,
    /// and if the request was not successful, it updates the response metadata with the error information.
    ///
    /// # Arguments
    /// - `&mut self`: A mutable reference to the `CreateDatabaseResp` instance.
    /// - `http_response`: A `reqwest::Response` containing the HTTP response data.
    ///
    /// # Returns
    /// - On success, returns `Ok(())`.
    /// - On error, returns an `Error` struct indicating the reason for the failure,
    ///   such as a parsing error.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Get the HTTP status code of the response.
        let http_status = http_response.status();

        // Parse the JSON response body into a `CreateDatabaseResp` struct.
        // If the parsing fails, map the error to an `ErrParseResponse` error type.
        let parsed_response: volcengine_sdk_protobuf::protobuf::rds_database::CreateDatabaseResp =
            http_response
                .json()
                .await
                .map_err(error::Error::ErrParseResponse)?;

        // Update the current `CreateDatabaseResp` instance with the parsed response data.
        *self = parsed_response;

        // Check if the HTTP request was not successful (status code is not in the 200 - 299 range).
        if !http_status.is_success() {
            // Check if the `response_metadata` field exists in the current `CreateDatabaseResp` instance.
            if let Some(mut response_metadata) = self.response_metadata.take() {
                // Ensure that the `error` field in the `response_metadata` exists.
                // If it doesn't, insert a default `ResponseMetadataErr` struct.
                let response_metadata_error = response_metadata.error.get_or_insert_with(
                    volcengine_sdk_protobuf::protobuf::rds_database::ResponseMetadataErr::default,
                );

                // Set the `code_n` field in the `response_metadata_error` struct
                // to the HTTP status code.
                response_metadata_error.code_n = Some(http_status.as_u16().into());

                // Update the `response_metadata` field in the current `CreateDatabaseResp` instance.
                self.response_metadata = Some(response_metadata);
            }
        }

        // Return `Ok(())` to indicate successful conversion of the HTTP response to the struct.
        return Ok(());
    }
}
