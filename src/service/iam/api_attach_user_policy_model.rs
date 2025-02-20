/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 15:10:33
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 17:00:35
 * @Description: Model for attaching a user policy to an IAM user.
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::iam_policy;

// Implementation of the ApiRequest trait for the AttachUserPolicyReq struct.
// This allows the struct to be used as a request object in API calls to attach a policy to an IAM user.
impl request::ApiRequest for iam_policy::AttachUserPolicyReq {
    /// Converts the request into a HashMap of key-value pairs for use in HTTP requests.
    /// This method is typically used when sending the request to the API server to attach a policy to a user.
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self)
    }

    /// Prepares the request body as a byte vector.
    /// For this request, the body is an empty vector since it's a GET request.
    fn to_body(&self) -> Vec<u8> {
        Vec::new()
    }
}

// Implementation of the ApiResponse trait for the AttachUserPolicyResp struct.
// This allows the struct to parse HTTP responses and convert them into a structured object containing information about the policy attachment.
impl response::ApiResponse for iam_policy::AttachUserPolicyResp {
    /// Converts the HTTP response into a structured response object.
    /// This method handles parsing the JSON response from the API server and updating the current object with the parsed data.
    /// It also checks the HTTP status code to determine if the request was successful.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Obtain the HTTP status code
        let http_status = http_response.status();

        // Parse the JSON response from the HTTP response.
        // The `json()` method deserializes the response body into the target type.
        let parsed_response: volcengine_sdk_protobuf::protobuf::iam_policy::AttachUserPolicyResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?; // Handle JSON parsing errors

        // Update the current object with the parsed response.
        *self = parsed_response;

        // Check if the HTTP status is not successful.
        if !http_status.is_success() {
            // Ensure that the `response_metadata` is present.
            if let Some(mut response_metadata) = self.response_metadata.take() {
                // Ensure that the `error` field is present.
                let response_metadata_error = response_metadata.error.get_or_insert_with(
                    volcengine_sdk_protobuf::protobuf::iam_policy::ResponseMetadataErr::default,
                );

                // Set the `code_n` field to the HTTP status code as a numeric value.
                response_metadata_error.code_n = Some(http_status.as_u16().into());

                // Update the `response_metadata` with the error information.
                self.response_metadata = Some(response_metadata);
            }
        }

        // Return Ok(()) to indicate successful parsing.
        Ok(())
    }
}
