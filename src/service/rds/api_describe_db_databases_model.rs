/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:39:54
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 11:22:31
 * @Description: Implementation of request and response handling for describing RDS databases.
 */
use crate::volcengine::error::error;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::rds_database;

/// Implementation of the `ApiRequest` trait for the `DescribeDatabasesReq` structure.
/// This implementation provides the necessary methods to convert the request into a format suitable for sending over HTTP.
impl request::ApiRequest for rds_database::DescribeDatabasesReq {
    /// Converts the request into a `HashMap` of headers.
    /// For this specific request, no additional headers are required, so an empty `HashMap` is returned.
    fn to_hashmap(&self) -> HashMap<String, String> {
        // request::Request::format_request_to_hashmap(self)
        HashMap::new()
    }

    /// Serializes the request into a byte vector to be sent as the request body.
    /// This method uses `serde_json` to serialize the request into a JSON format, which is then converted into a byte vector.
    fn to_body(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

/// Implementation of the `ApiResponse` trait for the `DescribeDatabasesResp` structure.
/// This implementation provides the necessary methods to parse the HTTP response into a structured response object.
impl response::ApiResponse for rds_database::DescribeDatabasesResp {
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

        // Convert the response body to a text string.
        let txt_response = http_response
            .text()
            .await
            .map_err(|e| error::Error::ErrParseResponse(e))?;

        // Sanitize the response text by replacing `null` values with empty arrays.
        // This is necessary because the response may contain `null` values that need to be handled properly.
        let sanitized_response =
            txt_response.replace("\"DatabasePrivileges\":null", "\"DatabasePrivileges\":[]"); // Replace `null` with an empty array

        // Parse the sanitized JSON response body into the expected structure.
        let parsed_response: volcengine_sdk_protobuf::protobuf::rds_database::DescribeDatabasesResp =
            serde_json::from_str(&sanitized_response).map_err(|e| error::Error::ErrParseJson(e))?;

        // Update the current response object with the parsed data.
        *self = parsed_response;

        // Check if the HTTP status code indicates an error.
        if !http_status.is_success() {
            // Check if `response_metadata` exists in the response.
            if let Some(mut response_metadata) = self.response_metadata.take() {
                // Ensure the `error` field exists in `response_metadata`.
                let response_metadata_error = response_metadata.error.get_or_insert_with(
                    volcengine_sdk_protobuf::protobuf::rds_database::ResponseMetadataErr::default,
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
