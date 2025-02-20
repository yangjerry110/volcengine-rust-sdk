/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:39:54
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 11:27:41
 * @Description: Implementation of request and response handling for modifying the allow list of an RDS instance.
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::rds_allow;

/// Implementation of the `ApiRequest` trait for the `ModifyAllowListReq` structure.
/// This implementation provides the necessary methods to convert the request into a format suitable for sending over HTTP.
impl request::ApiRequest for rds_allow::ModifyAllowListReq {
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

/// Implementation of the `ApiResponse` trait for the `ModifyAllowListResp` structure.
/// This implementation provides the necessary methods to parse the HTTP response into a structured response object.
impl response::ApiResponse for rds_allow::ModifyAllowListResp {
    /// Deserializes the HTTP response into a structured response object.
    /// This method takes an `http_response` object, parses its JSON body, and updates the current response object with the parsed data.
    ///
    /// # Arguments
    /// * `http_response` - The HTTP response object received from the server.
    ///
    /// # Returns
    /// * `Result<(), error::Error>` - Returns `Ok(())` if the response is successfully parsed, or an error if parsing fails.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Parse the JSON response body into the expected structure.
        let parsed_response: volcengine_sdk_protobuf::protobuf::rds_allow::ModifyAllowListResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?;

        // Update the current response object with the parsed data.
        *self = parsed_response;

        // Return successfully if no errors occurred.
        Ok(())
    }
}
