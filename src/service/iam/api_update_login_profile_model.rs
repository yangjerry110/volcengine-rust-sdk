/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 14:50:11
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 18:43:26
 * @Description: api update login profile model
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::iam_user;

/// Implementation of the `ApiRequest` trait for the `UpdateLoginProfileReq` struct.
/// This implementation allows the `UpdateLoginProfileReq` struct to be formatted
/// into a hashmap and a request body for making API requests.
impl request::ApiRequest for iam_user::UpdateLoginProfileReq {
    /// Converts the `UpdateLoginProfileReq` instance into a hashmap of string key - value pairs.
    /// This hashmap can be used to construct the query parameters or form data of an API request.
    ///
    /// # Returns
    /// - A `HashMap<String, String>` containing the formatted request data.
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self)
    }

    /// Converts the `UpdateLoginProfileReq` instance into a byte vector representing the request body.
    /// In this case, an empty vector is returned as the body, indicating that there is no
    /// specific body data for this request.
    ///
    /// # Returns
    /// - A `Vec<u8>` representing the request body.
    fn to_body(&self) -> Vec<u8> {
        Vec::new()
    }
}

/// Implementation of the `ApiResponse` trait for the `UpdateLoginProfileResp` struct.
/// This implementation is responsible for converting an HTTP response into the `UpdateLoginProfileResp` struct.
impl response::ApiResponse for iam_user::UpdateLoginProfileResp {
    /// Converts an HTTP response into the `UpdateLoginProfileResp` struct.
    /// It first parses the JSON response body, then updates the current `UpdateLoginProfileResp` instance
    /// with the parsed data. If there is an error during parsing, it returns an `Error` struct.
    ///
    /// # Arguments
    /// - `&mut self`: A mutable reference to the `UpdateLoginProfileResp` instance.
    /// - `http_response`: A `reqwest::Response` containing the HTTP response data.
    ///
    /// # Returns
    /// - On success, returns `Ok(())`.
    /// - On error, returns an `Error` struct indicating the reason for the failure,
    ///   such as a parsing error.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Parse the JSON response body into an `UpdateLoginProfileResp` struct.
        // If the parsing fails, map the error to an `ErrParseResponse` error type.
        let parsed_response: volcengine_sdk_protobuf::protobuf::iam_user::UpdateLoginProfileResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?;

        // Update the current `UpdateLoginProfileResp` instance with the parsed response data.
        *self = parsed_response;

        // Return `Ok(())` to indicate successful conversion of the HTTP response to the struct.
        Ok(())
    }
}
