/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 17:33:27
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 16:01:59
 * @Description: API for modifying the shard number of a Redis database instance.
 * This module provides the necessary implementations to handle requests and responses for modifying the shard number
 * of a Redis database instance in the Volcengine environment. It includes traits implementations for converting requests
 * to HTTP-compatible formats and parsing HTTP responses into structured data.
 */
use crate::volcengine::error::error;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::redis_instance;

/// Implementation of the `ApiRequest` trait for the `ModifyDbInstanceShardNumberReq` structure.
/// This implementation allows the `ModifyDbInstanceShardNumberReq` to be converted into a format
/// that can be sent as an HTTP request. It provides methods to generate headers and serialize the request body.
impl request::ApiRequest for redis_instance::RedisModifyDbInstanceShardNumberReq {
    /// Converts the request into a `HashMap` of headers and query parameters.
    /// This method is responsible for formatting the request's data into a `HashMap`
    /// where keys are parameter names and values are their corresponding values.
    /// These parameters will be used as query parameters in the HTTP request.
    /// In this implementation, an empty `HashMap` is returned, which might need to be adjusted
    /// based on the actual requirements of the API.
    ///
    /// # Returns
    /// * `HashMap<String, String>` - A map containing the request parameters as query parameters.
    fn to_hashmap(&self) -> HashMap<String, String> {
        // Return an empty HashMap. This can be modified to include necessary query parameters.
        HashMap::new()
    }

    /// Serializes the request into a byte vector to be sent as the request body.
    /// This method takes the current request object and serializes it into a JSON byte vector.
    /// The serialized data will be used as the body of the HTTP request.
    /// If the serialization process fails, it will panic because the `unwrap` method is used.
    ///
    /// # Returns
    /// * `Vec<u8>` - A byte vector representing the serialized request body.
    fn to_body(&self) -> Vec<u8> {
        // Serialize the request object into a JSON byte vector.
        serde_json::to_vec(self).unwrap()
    }
}

/// Implementation of the `ApiResponse` trait for the `ModifyDbInstanceShardNumberResp` structure.
/// This implementation enables the `ModifyDbInstanceShardNumberResp` to parse an HTTP response
/// into a structured response object. It provides a method to deserialize the JSON body of the HTTP response
/// and update the current response object with the parsed data.
impl response::ApiResponse for redis_instance::RedisModifyDbInstanceShardNumberResp {
    /// Deserializes the HTTP response into a structured response object.
    /// This asynchronous method takes an `http_response` object, which represents the HTTP response received from the server.
    /// It attempts to parse the JSON body of the response into a `ModifyDbInstanceShardNumberResp` object.
    /// If the parsing is successful, it updates the current response object with the parsed data.
    ///
    /// # Arguments
    /// * `http_response` - The HTTP response object received from the server.
    ///
    /// # Returns
    /// * `Result<(), error::Error>` - Returns `Ok(())` if the response is successfully parsed,
    /// or an `error::Error` if parsing fails. The error type `ErrParseResponse` is used to indicate
    /// a failure during the JSON parsing process.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Get the HTTP status code from the response.
        let http_status = http_response.status();

        // Parse the JSON response body into the expected structure.
        let parsed_response: volcengine_sdk_protobuf::protobuf::redis_instance::RedisModifyDbInstanceShardNumberResp =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?;

        // Update the current response object with the parsed data.
        *self = parsed_response;

        // Check if the HTTP status code indicates an error.
        if !http_status.is_success() {
            // Check if `response_metadata` exists in the response.
            if let Some(mut response_metadata) = self.response_metadata.take() {
                // Ensure the `error` field exists in `response_metadata`.
                let response_metadata_error = response_metadata.error.get_or_insert_with(
                    volcengine_sdk_protobuf::protobuf::redis_instance::ResponseMetadataErr::default,
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
