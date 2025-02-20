/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:39:54
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 11:14:44
 * @Description: Implementation of request and response handling for creating an RDS instance.
 */

use crate::volcengine::error::error;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::rds_instance;

/// Implementation of the `ApiRequest` trait for the `CreateDbInstanceReq` structure.
/// This implementation provides the necessary methods to convert the request into a format suitable for sending over HTTP.
impl request::ApiRequest for rds_instance::CreateDbInstanceReq {
    /// Converts the request into a `HashMap` of headers.
    /// For this specific request, no additional headers are required, so an empty `HashMap` is returned.
    fn to_hashmap(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    /// Serializes the request into a byte vector to be sent as the request body.
    /// This method uses `serde_json` to serialize the request into a JSON format, which is then converted into a byte vector.
    /// If serialization fails, it panics. In a production environment, this should be handled more gracefully.
    fn to_body(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
        // Alternatively, you could use the following method to handle errors:
        // request::Request::format_request_to_body(self)
    }
}

/// Implementation of the `ApiResponse` trait for the `CreateDbInstanceResp` structure.
/// This implementation provides the necessary methods to parse the HTTP response into a structured response object.
impl response::ApiResponse for rds_instance::CreateDbInstanceResp {
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
        let parsed_response: volcengine_sdk_protobuf::protobuf::rds_instance::CreateDbInstanceResp =
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
