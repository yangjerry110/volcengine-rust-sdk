/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 10:25:09
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 14:51:06
 * @Description: API to describe ECS images model
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::ecs_image;

/// Implement the `ApiRequest` trait for `DescribeImagesReq` struct.
///
/// This implementation provides methods to convert the request object to a hashmap
/// and to generate the body of the request as a byte vector. The `DescribeImagesReq`
/// struct is used to describe the images in the ECS (Elastic Compute Service) context.
///
/// # Methods
/// - `to_hashmap`: Converts the request struct into a `HashMap<String, String>`.
/// - `to_body`: Converts the request into a body in byte format (currently returns an empty vector).
impl request::ApiRequest for ecs_image::DescribeImagesReq {
    /// Converts the `DescribeImagesReq` struct into a `HashMap<String, String>`.
    ///
    /// This method uses the `Request::format_request_to_hashmap` utility to format
    /// the request struct into a hashmap for API request construction.
    ///
    /// # Returns
    /// - `HashMap<String, String>`: A hashmap representing the request data, which can be used
    ///   for building the query string for an API request.
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self)
    }

    /// Converts the `DescribeImagesReq` struct into the body of the request as a byte vector.
    ///
    /// This method currently returns an empty vector but can be customized to convert the struct
    /// into a body in the future (e.g., for POST requests).
    ///
    /// # Returns
    /// - `Vec<u8>`: The byte vector representing the body of the request. In this case, it's just an empty vector.
    fn to_body(&self) -> Vec<u8> {
        // The body is currently empty, but this method can be expanded to serialize
        // the request struct into a byte vector (e.g., using `serde_json`).
        Vec::new()
    }
}

/// Implement the `ApiResponse` trait for `DescribeImagesResp` struct.
///
/// This implementation provides a method to process the HTTP response from an API call
/// and convert it into the corresponding `DescribeImagesResp` struct. The response
/// typically contains a list of ECS images, and this struct will hold that information.
///
/// # Methods
/// - `to_struct`: Parses the HTTP response (JSON) and deserializes it into the `DescribeImagesResp` struct.
impl response::ApiResponse for ecs_image::DescribeImagesResp {
    /// Converts the HTTP response into the `DescribeImagesResp` struct.
    ///
    /// This method processes the HTTP response (which is expected to be in JSON format),
    /// deserializes it, and sets the current struct (`self`) with the parsed data.
    ///
    /// The method uses `reqwest::Response::json` to parse the JSON body into the
    /// `DescribeImagesResp` struct and handles any parsing errors by mapping them
    /// to the `error::Error::ErrParseResponse` variant.
    ///
    /// # Arguments
    /// - `http_response`: The HTTP response object received from the API.
    ///
    /// # Returns
    /// - `Ok(())`: If the parsing is successful, the method sets `self` to the parsed data.
    /// - `Err(error::Error)`: If there's a parsing error, the method returns an error.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Parse the JSON response body into `DescribeImagesResp` struct.
        let parsed_response: volcengine_sdk_protobuf::protobuf::ecs_image::DescribeImagesResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?;

        // Set the current struct (`self`) with the parsed response data.
        *self = parsed_response;

        // Return `Ok(())` to indicate successful processing.
        Ok(())
    }
}
