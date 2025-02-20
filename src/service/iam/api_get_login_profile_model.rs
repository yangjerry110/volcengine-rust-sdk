/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 15:10:33
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 18:16:31
 * @Description: get login profile model
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::iam_user;

/// Implementation of the `ApiRequest` trait for the `GetLoginProfileReq` struct.
/// This implementation provides methods to convert the request into a hashmap and a request body.
impl request::ApiRequest for iam_user::GetLoginProfileReq {
    /// Converts the `GetLoginProfileReq` instance into a hashmap of string key - value pairs.
    ///
    /// # Returns
    /// - A `HashMap<String, String>` containing the formatted request data.
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self)
    }

    /// Converts the `GetLoginProfileReq` instance into a byte vector representing the request body.
    /// In this case, an empty vector is returned as the body.
    ///
    /// # Returns
    /// - A `Vec<u8>` representing the request body.
    fn to_body(&self) -> Vec<u8> {
        Vec::new()
    }
}

/// Implementation of the `ApiResponse` trait for the `GetLoginProfileResp` struct.
/// This implementation provides a method to convert an HTTP response into the `GetLoginProfileResp` struct.
impl response::ApiResponse for iam_user::GetLoginProfileResp {
    /// Converts an HTTP response into the `GetLoginProfileResp` struct.
    /// It also handles error cases and updates the response metadata if the HTTP status is not successful.
    ///
    /// # Arguments
    /// - `&mut self`: A mutable reference to the `GetLoginProfileResp` instance.
    /// - `http_response`: A `reqwest::Response` containing the HTTP response data.
    ///
    /// # Returns
    /// - On success, returns `Ok(())`.
    /// - On error, returns an `Error` struct indicating the reason for the failure, such as a parsing error.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Get the HTTP status code
        let http_status = http_response.status();

        // Parse the JSON response body
        let parsed_response: volcengine_sdk_protobuf::protobuf::iam_user::GetLoginProfileResp =
            http_response
                .json()
                .await
                .map_err(error::Error::ErrParseResponse)?;

        // Update the current object
        *self = parsed_response;

        // Check if it's an error request
        if !http_status.is_success() {
            // Check if `response_metadata` exists
            if let Some(mut response_metadata) = self.response_metadata.take() {
                // Ensure the `error` field exists
                let response_metadata_error = response_metadata.error.get_or_insert_with(
                    volcengine_sdk_protobuf::protobuf::iam_user::ResponseMetadataErr::default,
                );

                // Set the `code_n` field
                response_metadata_error.code_n = Some(http_status.as_u16().into());

                // Update `response_metadata`
                self.response_metadata = Some(response_metadata);
            }
        }

        // Return result
        Ok(())
    }
}
