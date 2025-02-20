/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 15:10:33
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 18:36:59
 * @Description: create login profile model
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::iam_policy;

/// Implementation of the `ApiRequest` trait for the `ListPoliciesReq` struct.
/// This implementation enables the `ListPoliciesReq` struct to be transformed into a format
/// suitable for making API requests. It provides methods to convert the request into a hashmap
/// and a request body.
impl request::ApiRequest for iam_policy::ListPoliciesReq {
    /// Converts the `ListPoliciesReq` instance into a hashmap of string key - value pairs.
    /// This hashmap can be used to construct query parameters or form data for an API request.
    ///
    /// # Returns
    /// - A `HashMap<String, String>` containing the formatted request data.
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self)
    }

    /// Converts the `ListPoliciesReq` instance into a byte vector representing the request body.
    /// In this case, an empty vector is returned, indicating that there is no specific body data
    /// for this request.
    ///
    /// # Returns
    /// - A `Vec<u8>` representing the request body.
    fn to_body(&self) -> Vec<u8> {
        Vec::new()
    }
}

/// Implementation of the `ApiResponse` trait for the `ListPoliciesResp` struct.
/// This implementation is responsible for converting an HTTP response into the `ListPoliciesResp` struct.
/// It parses the JSON response body and updates the current instance with the parsed data.
impl response::ApiResponse for iam_policy::ListPoliciesResp {
    /// Converts an HTTP response into the `ListPoliciesResp` struct.
    /// It first parses the JSON response body, then updates the current `ListPoliciesResp` instance
    /// with the parsed data. If there is an error during parsing, it returns an `Error` struct.
    ///
    /// # Arguments
    /// - `&mut self`: A mutable reference to the `ListPoliciesResp` instance.
    /// - `http_response`: A `reqwest::Response` containing the HTTP response data.
    ///
    /// # Returns
    /// - On success, returns `Ok(())`.
    /// - On error, returns an `Error` struct indicating the reason for the failure,
    ///   such as a parsing error.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Parse the JSON response body into a `ListPoliciesResp` struct.
        // If the parsing fails, map the error to an `ErrParseResponse` error type.
        let parsed_response: volcengine_sdk_protobuf::protobuf::iam_policy::ListPoliciesResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?;

        // Update the current `ListPoliciesResp` instance with the parsed response data.
        *self = parsed_response;

        // Return `Ok(())` to indicate successful conversion of the HTTP response to the struct.
        Ok(())
    }
}
