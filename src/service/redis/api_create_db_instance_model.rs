/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 17:33:27
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 15:59:27
 * @Description: API for creating a Redis database instance.
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::redis_instance;

/// Implementation of the `ApiRequest` trait for the `CreateDbInstanceReq` structure.
/// This implementation provides the necessary methods to convert the request into a format suitable for sending over HTTP.
impl request::ApiRequest for redis_instance::RedisCreateDbInstanceReq {
    /// Converts the request into a `HashMap` of headers.
    /// This method formats the request parameters into a `HashMap` that can be used as query parameters in the HTTP request.
    /// The `Request::format_request_to_hashmap` method is used to handle the conversion.
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self)
    }

    /// Serializes the request into a byte vector to be sent as the request body.
    /// For this specific request, no request body is required, so an empty byte vector is returned.
    fn to_body(&self) -> Vec<u8> {
        Vec::new()
    }
}

/// Implementation of the `ApiResponse` trait for the `CreateDbInstanceResp` structure.
/// This implementation provides the necessary methods to parse the HTTP response into a structured response object.
impl response::ApiResponse for redis_instance::RedisCreateDbInstanceResp {
    /// Deserializes the HTTP response into a structured response object.
    /// This method takes an `http_response` object, parses its JSON body, and updates the current response object with the parsed data.
    ///
    /// # Arguments
    /// * `http_response` - The HTTP response object received from the server.
    ///
    /// # Returns
    /// * `Result<(), error::Error>` - Returns `Ok(())` if the response is successfully parsed, or an error if parsing fails.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Get the HTTP status code from the response.
        let http_status = http_response.status();

        // Parse the JSON response body into the expected structure.
        let parsed_response: volcengine_sdk_protobuf::protobuf::redis_instance::RedisCreateDbInstanceResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?;

        // Update the current response object with the parsed data.
        *self = parsed_response;

        // Check if the HTTP status code indicates an error.
        if !http_status.is_success() {
            // Check if `response_metadata` exists in the response.
            if let Some(mut response_metadata) = self.response_metadata.take() {
                // Ensure the `error` field exists in `response_metadata`.
                let response_metadata_error = response_metadata.error.get_or_insert_with(
                    volcengine_sdk_protobuf::protobuf::redis_instance::ResponseMetadataErr::default,
                );

                // Set the `code_n` field to the HTTP status code.
                response_metadata_error.code_n = Some(http_status.as_u16().into());

                // Update `response_metadata` with the error information.
                self.response_metadata = Some(response_metadata);
            }
        }

        // Return successfully if no errors occurred.
        Ok(())
    }
}
