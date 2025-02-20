/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 15:10:33
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 18:49:13
 * @Description: create login profile model
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::iam_policy;

/// Implements the `ApiRequest` trait for the `UpdatePolicyReq` struct.
/// This implementation allows the `UpdatePolicyReq` struct to be formatted
/// into a format suitable for API requests, including a hashmap for query parameters
/// and an empty request body in this case.
impl request::ApiRequest for iam_policy::UpdatePolicyReq {
    /// Converts the `UpdatePolicyReq` instance into a hashmap of string key - value pairs.
    /// This hashmap can be used to construct query parameters for an API request.
    ///
    /// # Returns
    /// - A `HashMap<String, String>` containing the formatted request data.
    fn to_hashmap(&self) -> HashMap<String, String> {
        // Use the utility function to format the request into a hashmap
        request::Request::format_request_to_hashmap(self)
    }

    /// Converts the `UpdatePolicyReq` instance into a byte vector representing the request body.
    /// In this implementation, an empty vector is returned, indicating that there is no
    /// specific data to be sent in the request body for this type of request.
    ///
    /// # Returns
    /// - A `Vec<u8>` representing the request body (empty in this case).
    fn to_body(&self) -> Vec<u8> {
        Vec::new()
    }
}

/// Implements the `ApiResponse` trait for the `UpdatePolicyResp` struct.
/// This implementation is responsible for converting an HTTP response into the `UpdatePolicyResp` struct
/// and handling error cases by updating the response metadata when necessary.
impl response::ApiResponse for iam_policy::UpdatePolicyResp {
    /// Converts an HTTP response into the `UpdatePolicyResp` struct.
    /// It first checks the HTTP status code, parses the JSON response body,
    /// updates the current `UpdatePolicyResp` instance with the parsed data,
    /// and if the request was not successful, it updates the response metadata with the error code.
    ///
    /// # Arguments
    /// - `&mut self`: A mutable reference to the `UpdatePolicyResp` instance.
    /// - `http_response`: A `reqwest::Response` containing the HTTP response data.
    ///
    /// # Returns
    /// - On success, returns `Ok(())`.
    /// - On error, returns an `Error` struct indicating the reason for the failure,
    ///   such as a parsing error.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Retrieve the HTTP status code from the response
        let http_status = http_response.status();

        // Parse the JSON response body into an `UpdatePolicyResp` struct.
        // If the parsing fails, map the error to an `ErrParseResponse` error type.
        let parsed_response: volcengine_sdk_protobuf::protobuf::iam_policy::UpdatePolicyResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?;

        // Update the current `UpdatePolicyResp` instance with the parsed response data.
        *self = parsed_response;

        // Check if the HTTP request was not successful (status code is not in the 200 - 299 range).
        if !http_status.is_success() {
            // Check if the `response_metadata` field exists in the current `UpdatePolicyResp` instance.
            if let Some(mut response_metadata) = self.response_metadata.take() {
                // Ensure that the `error` field in the `response_metadata` exists.
                // If it doesn't, insert a default `ResponseMetadataErr` struct.
                let response_metadata_error = response_metadata.error.get_or_insert_with(
                    volcengine_sdk_protobuf::protobuf::iam_policy::ResponseMetadataErr::default,
                );

                // Set the `code_n
                response_metadata_error.code_n = Some(http_status.as_u16().into());

                // Update `response_metadata`
                self.response_metadata = Some(response_metadata);
            }
        }

        // return
        Ok(())
    }
}
