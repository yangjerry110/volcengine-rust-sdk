/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 17:33:27
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 14:35:15
 * @Description: API for describing VPCs.
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::vpc_vpc;

/// Implementation of the `ApiRequest` trait for the `DescribeVpcsReq` structure.
/// This implementation provides the necessary methods to convert the request into a format suitable for sending over HTTP.
impl request::ApiRequest for vpc_vpc::DescribeVpcsReq {
    /// Converts the request into a `HashMap` of headers.
    /// This method formats the request parameters into a `HashMap` that can be used as query parameters in the HTTP request.
    /// The `Request::format_request_to_hashmap` method is used to handle the conversion.
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self)
    }

    /// Serializes the request into a byte vector to be sent as the request body.
    /// For this specific request, no request body is required, so an empty byte vector is returned.
    fn to_body(&self) -> Vec<u8> {
        Vec::new()
    }
}

/// Implementation of the `ApiResponse` trait for the `DescribeVpcsResp` structure.
/// This implementation provides the necessary methods to parse the HTTP response into a structured response object.
impl response::ApiResponse for vpc_vpc::DescribeVpcsResp {
    /// Deserializes the HTTP response into a structured response object.
    /// This method takes an `http_response` object, parses its JSON body, and updates the current response object with the parsed data.
    ///
    /// # Arguments
    /// * `http_response` - The HTTP response object received from the server.
    ///
    /// # Returns
    /// * `Result<(), error::Error>` - Returns `Ok(())` if the response is successfully parsed, or an error if parsing fails.
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // Convert the response body to a text string.
        let txt_response = http_response
            .text()
            .await
            .map_err(|e| error::Error::ErrParseResponse(e))?;

        // Sanitize the response text by replacing `null` values with empty arrays.
        // This is necessary because the response may contain `null` values that need to be handled properly.
        let sanitized_response = txt_response
            .replace("\"SecondaryCidrBlocks\":null", "\"SecondaryCidrBlocks\":[]") // Replace `null` with an empty array
            .replace("\"DnsServers\":null", "\"DnsServers\":[]");

        // Parse the sanitized JSON response body into the expected structure.
        let parsed_response: volcengine_sdk_protobuf::protobuf::vpc_vpc::DescribeVpcsResp =
            serde_json::from_str(&sanitized_response).map_err(|e| error::Error::ErrParseJson(e))?;

        // Update the current response object with the parsed data.
        *self = parsed_response;

        // Return successfully if no errors occurred.
        Ok(())
    }
}
