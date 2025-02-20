/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 10:25:09
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 16:50:20
 * @Description: api describe instances model
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response;
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::ecs_zone;

// Implement the ApiRequest trait for the DescribeRegionsReq struct.
// This allows the struct to be used as a request object in API calls.
impl request::ApiRequest for ecs_zone::DescribeRegionsReq {
    // Converts the request into a HashMap of key-value pairs for use in HTTP requests.
    // This method is typically used when sending the request to the API server.
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self)
    }

    // Prepares the request body as a byte vector.
    // For this request, the body is an empty vector since it's a GET request.
    fn to_body(&self) -> Vec<u8> {
        Vec::new()
    }
}

// Implement the ApiResponse trait for the DescribeRegionsResp struct.
// This allows the struct to parse HTTP responses and convert them into a structured object.
impl response::ApiResponse for ecs_zone::DescribeRegionsResp {
    // Converts the HTTP response into a structured response object.
    // This method handles parsing the JSON response from the API server.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Parse the JSON response from the HTTP response.
        // The `json()` method deserializes the response body into the target type.
        let parsed_response: volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeRegionsResp =
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
