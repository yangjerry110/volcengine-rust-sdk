/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 15:10:33
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 17:31:24
 * @Description: Model for detaching a user policy in IAM (Identity and Access Management)
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::iam_policy;

// Implementation of the ApiRequest trait for the DetachUserPolicyReq struct
// This allows the struct to be used as a request object in API calls to detach a policy from an IAM user
impl request::ApiRequest for iam_policy::DetachUserPolicyReq {
    /// Converts the request into a HashMap of key-value pairs for use in HTTP requests
    /// This method is typically used when sending the request to the API server to detach a policy from a user
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

// Implementation of the ApiResponse trait for the DetachUserPolicyResp struct
// This allows the struct to parse HTTP responses and convert them into a structured object containing information about the detached policy
impl response::ApiResponse for iam_policy::DetachUserPolicyResp {
    /// Converts the HTTP response into a structured response object
    /// This method handles parsing the JSON response from the API server and updating the current object with the parsed data
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Parse the JSON response body
        let parsed_response: volcengine_sdk_protobuf::protobuf::iam_policy::DetachUserPolicyResp =
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
