/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-22 15:02:12
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 17:28:49
 * @Description: Model for deleting a user in IAM (Identity and Access Management)
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response;
use std::collections::HashMap;

// Implementation of the ApiRequest trait for the DeleteUserReq struct
// This allows the struct to be used as a request object in API calls to delete a user
impl request::ApiRequest for volcengine_sdk_protobuf::protobuf::iam_user::DeleteUserReq {
    /// Converts the request into a HashMap of key-value pairs for use in HTTP requests
    /// This method is typically used when sending the request to the API server to delete a user
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self)
    }

    /// Prepares the request body as a byte vector
    /// For this request, the body is an empty vector since it's a GET request
    fn to_body(&self) -> Vec<u8> {
        let result = Vec::new();
        result
    }
}

// Implementation of the ApiResponse trait for the DeleteUserResp struct
// This allows the struct to parse HTTP responses and convert them into a structured object containing information about the deleted user
impl response::ApiResponse for volcengine_sdk_protobuf::protobuf::iam_user::DeleteUserResp {
    /// Converts the HTTP response into a structured response object
    /// This method handles parsing the JSON response from the API server and updating the current object with the parsed data
    /// It also checks the HTTP status code to determine if the request was successful
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Get the HTTP status code
        let http_status = http_response.status();

        // Parse the JSON response body
        let parsed_response: volcengine_sdk_protobuf::protobuf::iam_user::DeleteUserResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?; // Handle JSON parsing errors

        // Update the current object with the parsed response
        *self = parsed_response;

        // Check if the HTTP status is not successful
        if !http_status.is_success() {
            // Check if the `response_metadata` exists
            if let Some(mut response_metadata) = self.response_metadata.take() {
                // Ensure the `error` field exists
                let response_metadata_error = response_metadata.error.get_or_insert_with(
                    volcengine_sdk_protobuf::protobuf::iam_user::ResponseMetadataErr::default,
                );

                // Set the `code_n` field
                response_metadata_error.code_n = Some(http_status.as_u16().into());

                // Update the `response_metadata`
                self.response_metadata = Some(response_metadata);
            }
        }

        // Return Ok(()) to indicate successful parsing
        Ok(())
    }
}
