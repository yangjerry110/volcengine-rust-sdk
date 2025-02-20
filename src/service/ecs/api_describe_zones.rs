/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 11:08:30
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:30:53
 * @Description: API to describe zones in ECS (Elastic Compute Service)
 */
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use crate::{service::ecs, volcengine::error::error};
use volcengine_sdk_protobuf::protobuf::ecs_zone;

/// The `ApiDescribeZonesEcs` struct is used to interact with the Volcengine ECS service to describe
/// available zones. It encapsulates the logic for constructing and sending requests to retrieve zone information.
pub struct ApiDescribeZonesEcs;

/// Implementation of methods for the `ApiDescribeZonesEcs` struct.
/// These methods handle the logic for describing ECS zones, including constructing requests and processing responses.
impl ApiDescribeZonesEcs {
    /// The `new_describe_zones` function creates a new API request to describe ECS zones.
    /// This function is the primary entry point for initiating a zone description request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiDescribeZonesEcs`.
    /// - `ecs`: A reference to the `ecs::Ecs` struct, which contains the necessary ECS client information.
    /// - `request`: A `DescribeZonesReq` struct containing the parameters to describe ECS zones.
    ///
    /// # Returns:
    /// This function returns a `Result<ecs_zone::DescribeZonesResp, error::Error>`.
    /// - On success, it returns a `DescribeZonesResp` struct with the response from the ECS API.
    /// - On failure, it returns an `error::Error` indicating the failure reason.
    pub async fn new_describe_zones(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_zone::DescribeZonesReq,
    ) -> Result<ecs_zone::DescribeZonesResp, error::Error> {
        // Delegate the request creation and sending to the internal helper function.
        self.new_describe_zones_request(ecs, request).await
    }

    /// The `new_describe_zones_request` function constructs and sends the API request to describe ECS zones.
    /// This function handles the internal details of request creation and response parsing.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiDescribeZonesEcs`.
    /// - `ecs`: A reference to the `ecs::Ecs` struct, which contains ECS client information.
    /// - `request`: A `DescribeZonesReq` struct with parameters for describing ECS zones.
    ///
    /// # Returns:
    /// This function returns a `Result<ecs_zone::DescribeZonesResp, error::Error>`.
    /// - On success, it returns a `DescribeZonesResp` containing information about the described zones.
    /// - On failure, it returns an `error::Error` indicating the error reason.
    async fn new_describe_zones_request(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_zone::DescribeZonesReq,
    ) -> Result<ecs_zone::DescribeZonesResp, error::Error> {
        // Define the request operation with necessary configurations.
        // The `operation_name` corresponds to the ECS "DescribeZones" operation.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::EcsOperation(
                    operation_config::operation_name_ecs::OperationNameEcs::DescribeZones,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?; // Build the operation and return a Result

        // Set up the request using the client information from the `ecs` struct.
        // The request is built with operation details and ECS client configuration.
        // After that, the request is sent using the `send` method and the response is processed.
        let response = request::Request::builder()
            .with_client_info(&ecs.client.client_info) // Attach client information (e.g., credentials)
            .with_config(&ecs.client.config) // Include configuration settings for the ECS client
            .with_handles(&ecs.client.handles) // Use the appropriate ECS client handles for this operation
            .with_operation(&request_operation) // Attach the operation definition (DescribeZones)
            .build()? // Build the request
            .send(request) // Send the request to ECS with the provided `DescribeZonesReq`
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp = volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeZonesResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
