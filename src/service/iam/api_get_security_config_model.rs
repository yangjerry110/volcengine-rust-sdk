/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 15:10:33
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 18:30:43
 * @Description: get login profile model
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::iam_user;

/// Implementation of the `ApiRequest` trait for the `GetSecurityConfigReq` struct.
/// This implementation allows the `GetSecurityConfigReq` struct to be formatted into a hashmap
/// and a request body for making API requests.
impl request::ApiRequest for iam_user::GetSecurityConfigReq {
    /// Converts the `GetSecurityConfigReq` instance into a hashmap of string key - value pairs.
    /// This hashmap can be used to construct the query parameters or form data of an API request.
    ///
    /// # Returns
    /// - A `HashMap<String, String>` containing the formatted request data.
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self)
    }

    /// Converts the `GetSecurityConfigReq` instance into a byte vector representing the request body.
    /// In this case, an empty vector is returned as the body, indicating that there is no
    /// specific body data for this request.
    ///
    /// # Returns
    /// - A `Vec<u8>` representing the request body.
    fn to_body(&self) -> Vec<u8> {
        let result = Vec::new();
        result
    }
}

/// Implementation of the `ApiResponse` trait for the `GetSecurityConfigResp` struct.
/// This implementation is responsible for converting an HTTP response into the `GetSecurityConfigResp` struct.
impl response::ApiResponse for iam_user::GetSecurityConfigResp {
    /// Converts an HTTP response into the `GetSecurityConfigResp` struct.
    /// It first parses the JSON response body and then updates the current `GetSecurityConfigResp` instance
    /// with the parsed data.
    ///
    /// # Arguments
    /// - `&mut self`: A mutable reference to the `GetSecurityConfigResp` instance.
    /// - `http_response`: A `reqwest::Response` containing the HTTP response data.
    ///
    /// # Returns
    /// - On success, returns `Ok(())`.
    /// - On error, returns an `Error` struct indicating the reason for the failure,
    ///   such as a parsing error.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Parse the JSON response body into a `GetSecurityConfigResp` struct.
        // If the parsing fails, map the error to an `ErrParseResponse` error type.
        let parsed_response: volcengine_sdk_protobuf::protobuf::iam_user::GetSecurityConfigResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?;

        // Update the current `GetSecurityConfigResp` instance with the parsed response data.
        *self = parsed_response;

        // Return `Ok(())` to indicate successful conversion of the HTTP response to the struct.
        Ok(())
    }
}
