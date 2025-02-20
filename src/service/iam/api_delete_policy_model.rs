/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 15:10:33
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 17:26:18
 * @Description: Model for deleting a policy in IAM (Identity and Access Management)
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::iam_policy;

// Implementation of the ApiRequest trait for the DeletePolicyReq struct
// This allows the struct to be used as a request object in API calls to delete a policy
impl request::ApiRequest for iam_policy::DeletePolicyReq {
    /// Converts the request into a HashMap of key-value pairs for use in HTTP requests
    /// This method is typically used when sending the request to the API server to delete a policy
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

// Implementation of the ApiResponse trait for the DeletePolicyResp struct
// This allows the struct to parse HTTP responses and convert them into a structured object containing information about the deleted policy
impl response::ApiResponse for iam_policy::DeletePolicyResp {
    /// Converts the HTTP response into a structured response object
    /// This method handles parsing the JSON response from the API server and updating the current object with the parsed data
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Parse the JSON response body
        let parsed_response: volcengine_sdk_protobuf::protobuf::iam_policy::DeletePolicyResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?; // Handle JSON parsing errors

        // Update the current object with the parsed response
        *self = parsed_response;

        // Return Ok(()) to indicate successful parsing
        Ok(())
    }
}
