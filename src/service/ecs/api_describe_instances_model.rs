/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 10:25:09
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-12-04 15:02:39
 * @Description: API describe instances model
 *
 * This module defines the `DescribeInstancesReq` and `DescribeInstancesResp` structures
 * and provides implementations for converting them to/from request and response formats.
 * The goal is to interface with the ECS (Elastic Compute Service) API for describing ECS instances.
 *
 * It includes implementations for two key parts:
 * 1. `DescribeInstancesReq`: A request struct that can be converted into a `HashMap` or a request body.
 * 2. `DescribeInstancesResp`: A response struct that processes the HTTP response, converts it into the
 *    struct, and handles error status codes.
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::ecs_instance;

// Implement the `ApiRequest` trait for the `DescribeInstancesReq` struct.
//
// This implementation allows the `DescribeInstancesReq` struct to be used in the process of
// making requests to the ECS API. Specifically, it provides two methods for converting
// the request object into a format suitable for an HTTP request:
// 1. `to_hashmap`: Converts the struct to a `HashMap<String, String>` for sending parameters as part of the query string.
// 2. `to_body`: Prepares the request body as a vector of bytes (`Vec<u8>`) to be sent in the HTTP request.
//
// The `ApiRequest` trait is part of the infrastructure that abstracts the request logic,
// allowing for different types of requests (e.g., describing instances, managing resources) to
// be structured and sent to the API consistently. In this case, `DescribeInstancesReq` is used
// to request information about ECS instances from the service.
impl request::ApiRequest for ecs_instance::DescribeInstancesReq {
    /// Converts the `DescribeInstancesReq` struct into a `HashMap<String, String>`.
    ///
    /// This function is used for preparing the request parameters in the form of a hashmap,
    /// which will be used when constructing the HTTP request to the ECS API.
    ///
    /// # Returns
    /// - A `HashMap<String, String>` containing the request parameters in key-value pairs.
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self) // Utilize helper function to convert the request to hashmap
    }

    /// Converts the `DescribeInstancesReq` struct into the request body in `Vec<u8>` format.
    ///
    /// This function is responsible for returning an empty vector as the body for the request.
    /// This might be used for requests that don't require a body or for future expansions if
    /// the body content is to be added.
    ///
    /// # Returns
    /// - An empty `Vec<u8>`, as this specific request does not contain a body.
    fn to_body(&self) -> Vec<u8> {
        let result = Vec::new(); // No body content in this case
        result
    }
}

// Implement the `ApiResponse` trait for the `DescribeInstancesResp` struct.
//
// This implementation is crucial for processing the response from the ECS API after a request
// is made. Specifically, it provides the functionality to:
// 1. Parse the response body into a `DescribeInstancesResp` struct.
// 2. Handle any potential error status codes returned by the API and update the response metadata accordingly.
//
// The `ApiResponse` trait abstracts the logic for handling API responses. It allows for:
// - Parsing the JSON response into a structured Rust object (in this case, `DescribeInstancesResp`).
// - Checking for error conditions, particularly for non-2xx status codes, and updating the response metadata to reflect error details.
//
// This ensures that the response is correctly processed and any error handling i
impl response::ApiResponse for ecs_instance::DescribeInstancesResp {
    /// Processes the HTTP response and populates the `DescribeInstancesResp` struct.
    ///
    /// This function is used for handling the response from the ECS API after a request is sent.
    /// It processes the HTTP status, parses the JSON response body, and handles error conditions.
    ///
    /// # Parameters
    /// - `http_response`: The HTTP response from the ECS API. This contains the status code and response body.
    ///
    /// # Returns
    /// - `Result<(), error::Error>`: Returns `Ok(())` if the parsing is successful, or an error if there is an issue.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Get HTTP status code to check the success of the request
        let http_status = http_response.status();

        // Parse the JSON response body into the `DescribeInstancesResp` struct
        let parsed_response: volcengine_sdk_protobuf::protobuf::ecs_instance::DescribeInstancesResp =
            http_response
                .json()
                .await
                .map_err(error::Error::ErrParseResponse)?; // Map any JSON parsing error to custom error type

        // Update the current struct with the parsed response
        *self = parsed_response;

        // Check if the response indicates an error (non-2xx status code)
        if !http_status.is_success() {
            // If response contains metadata, process it further
            if let Some(mut response_metadata) = self.response_metadata.take() {
                // Ensure `error` field exists in `response_metadata`
                let response_metadata_error = response_metadata.error.get_or_insert_with(
                    volcengine_sdk_protobuf::protobuf::ecs_instance::ResponseMetadataErr::default,
                );

                // Set the error code based on HTTP status code (e.g., 404, 500)
                response_metadata_error.code_n = Some(http_status.as_u16().into());

                // Update the `response_metadata` with the modified error information
                self.response_metadata = Some(response_metadata);
            }
        }

        // Return Ok if processing was successful
        Ok(())
    }
}
