/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-22 15:02:12
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 18:33:29
 * @Description: get_user model
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response;
use std::collections::HashMap;

/// Implementation of the `ApiRequest` trait for the `GetUserReq` struct.
/// This implementation allows the `GetUserReq` struct to be formatted into a hashmap
/// and a request body for making API requests.
impl request::ApiRequest for volcengine_sdk_protobuf::protobuf::iam_user::GetUserReq {
    /// Converts the `GetUserReq` instance into a hashmap of string key - value pairs.
    /// This hashmap can be used to construct the query parameters or form data of an API request.
    ///
    /// # Returns
    /// - A `HashMap<String, String>` containing the formatted request data.
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self)
    }

    /// Converts the `GetUserReq` instance into a byte vector representing the request body.
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

/// Implementation of the `ApiResponse` trait for the `GetUserResp` struct.
/// This implementation is responsible for converting an HTTP response into the `GetUserResp` struct
/// and handling error cases by updating the response metadata if necessary.
impl response::ApiResponse for volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp {
    /// Converts an HTTP response into the `GetUserResp` struct.
    /// It first checks the HTTP status code, parses the JSON response body,
    /// updates the current `GetUserResp` instance with the parsed data,
    /// and if the request was not successful, it updates the response metadata with the error code.
    ///
    /// # Arguments
    /// - `&mut self`: A mutable reference to the `GetUserResp` instance.
    /// - `http_response`: A `reqwest::Response` containing the HTTP response data.
    ///
    /// # Returns
    /// - On success, returns `Ok(())`.
    /// - On error, returns an `Error` struct indicating the reason for the failure,
    ///   such as a parsing error.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Get the HTTP status code of the response
        let http_status = http_response.status();

        // Parse the JSON response body into a `GetUserResp` struct.
        // If the parsing fails, map the error to an `ErrParseResponse` error type.
        let parsed_response: volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp =
            http_response
                .json()
                .await
                .map_err(error::Error::ErrParseResponse)?;

        // Update the current `GetUserResp` instance with the parsed response data.
        *self = parsed_response;

        // Check if the HTTP request was not successful (status code is not in the 200 - 299 range).
        if !http_status.is_success() {
            // Check if the `response_metadata` field exists in the current `GetUserResp` instance.
            if let Some(mut response_metadata) = self.response_metadata.take() {
                // Ensure that the `error` field in the `response_metadata` exists.
                // If it doesn't, insert a default `ResponseMetadataErr` struct.
                let response_metadata_error = response_metadata.error.get_or_insert_with(
                    volcengine_sdk_protobuf::protobuf::iam_user::ResponseMetadataErr::default,
                );

                // Set the `code_n` field in the `response_metadata_error` struct
                // to the HTTP status code.
                response_metadata_error.code_n = Some(http_status.as_u16().into());

                // Update the `response_metadata` field in the current `GetUserResp` instance.
                self.response_metadata = Some(response_metadata);
            }
        }

        // Return `Ok(())` to indicate successful conversion of the HTTP response to the struct.
        Ok(())
    }
}
