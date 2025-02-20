/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-28 17:13:35
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 16:43:08
 * @Description: API model for running instances in ECS (Elastic Compute Service)
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::ecs_instance;

// Implementation of the ApiRequest trait for the RunInstancesReq struct.
// This allows the struct to be used as a request object in API calls to create new ECS instances.
impl request::ApiRequest for ecs_instance::RunInstancesReq {
    /// Converts the request into a HashMap of key-value pairs for use in HTTP requests.
    /// This method is typically used when sending the request to the API server to start new instances.
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self)
    }

    /// Prepares the request body as a byte vector.
    /// For this request, the body is an empty vector when using GET method.
    /// However, in practice, this request may use POST method with a body containing instance configurations.
    fn to_body(&self) -> Vec<u8> {
        Vec::new()
        // request::Request::format_request_to_body(self)
    }
}

// Implementation of the ApiResponse trait for the RunInstancesResp struct.
// This allows the struct to parse HTTP responses and convert them into a structured object containing information about the newly created instances.
impl response::ApiResponse for ecs_instance::RunInstancesResp {
    /// Converts the HTTP response into a structured response object.
    /// This method handles parsing the JSON response from the API server and updating the current object with the parsed data.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Parse the JSON response from the HTTP response.
        // The `json()` method deserializes the response body into the target type.
        let parsed_response: volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?; // Handle JSON parsing errors

        // Replace the current `self` with the parsed response.
        *self = parsed_response;

        // Return Ok(()) to indicate successful parsing.
        Ok(())
    }
}
