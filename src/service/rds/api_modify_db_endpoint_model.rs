/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:39:54
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 11:30:41
 * @Description: Implementation of request and response handling for modifying a database endpoint.
 */
use crate::volcengine::error::error;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::rds_endpoint;

/// Implementation of the `ApiRequest` trait for the `ModifyDbEndpointReq` structure.
/// This implementation provides the necessary methods to convert the request into a format suitable for sending over HTTP.
impl request::ApiRequest for rds_endpoint::ModifyDbEndpointReq {
    /// Converts the request into a `HashMap` of headers.
    /// For this specific request, no additional headers are required, so an empty `HashMap` is returned.
    /// The `Request::format_request_to_hashmap` method is commented out as it is not used in this implementation.
    fn to_hashmap(&self) -> HashMap<String, String> {
        // request::Request::format_request_to_hashmap(self)
        HashMap::new()
    }

    /// Serializes the request into a byte vector to be sent as the request body.
    /// This method uses `serde_json` to serialize the request into a JSON format, which is then converted into a byte vector.
    /// If serialization fails, it panics. In a production environment, this should be handled more gracefully.
    fn to_body(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

/// Implementation of the `ApiResponse` trait for the `ModifyDbEndpointResp` structure.
/// This implementation provides the necessary methods to parse the HTTP response into a structured response object.
impl response::ApiResponse for rds_endpoint::ModifyDbEndpointResp {
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
        let parsed_response: volcengine_sdk_protobuf::protobuf::rds_endpoint::ModifyDbEndpointResp =
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
