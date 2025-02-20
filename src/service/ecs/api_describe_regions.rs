/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 11:08:30
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:29:27
 * @Description: API to describe regions in ECS (Elastic Compute Service)
 */
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use crate::{service::ecs, volcengine::error::error};
use volcengine_sdk_protobuf::protobuf::ecs_zone;

/// `ApiDescribeRegionsEcs` is a struct that provides functionality to describe regions in ECS (Elastic Compute Service).
/// It encapsulates the logic for making API requests to the Volcengine ECS service to retrieve information about available regions.
///
/// # Usage Example
/// ```rust
/// let ecs = ecs::Ecs::new();
/// let req = ecs_zone::DescribeRegionsReq {
///     // Set request parameters here
/// };
/// let api = ApiDescribeRegionsEcs;
/// match api.new_describe_regions(&ecs, req).await {
///     Ok(response) => println!("Regions description: {:?}", response),
///     Err(err) => eprintln!("Error: {}", err),
/// };
/// ```
pub struct ApiDescribeRegionsEcs;

/// Implementation of methods for the `ApiDescribeRegionsEcs` struct.
/// These methods handle the logic for describing regions in ECS, including constructing requests and processing responses.
impl ApiDescribeRegionsEcs {
    /// Creates a new API request to describe ECS regions.
    /// This method is the main entry point for interacting with the ECS DescribeRegions API.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiDescribeRegionsEcs`.
    /// - `ecs`: A reference to the `ecs::Ecs` struct, which provides the necessary client information for ECS operations.
    /// - `request`: A `DescribeRegionsReq` struct containing the parameters to describe ECS regions.
    ///
    /// # Returns:
    /// This function returns a `Result<ecs_zone::DescribeRegionsResp, error::Error>`.
    /// - On success, it returns a `DescribeRegionsResp` struct with the response from the ECS API.
    /// - On failure, it returns an `error::Error` indicating the failure reason.
    pub async fn new_describe_regions(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_zone::DescribeRegionsReq,
    ) -> Result<ecs_zone::DescribeRegionsResp, error::Error> {
        // Calls the internal helper function to initiate the ECS describe regions request.
        self.new_describe_regions_request(ecs, request).await
    }

    /// Internal function to create and send the request to describe ECS regions.
    /// This function constructs the request operation and sends it using the Volcengine request system.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiDescribeRegionsEcs`.
    /// - `ecs`: A reference to the `ecs::Ecs` struct, which contains ECS client information.
    /// - `request`: A `DescribeRegionsReq` struct with parameters for describing ECS regions.
    ///
    /// # Returns:
    /// This function returns a `Result<ecs_zone::DescribeRegionsResp, error::Error>`.
    /// - On success, it returns a `DescribeRegionsResp` containing information about the described regions.
    /// - On failure, it returns an `error::Error` indicating the error reason.
    async fn new_describe_regions_request(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_zone::DescribeRegionsReq,
    ) -> Result<ecs_zone::DescribeRegionsResp, error::Error> {
        // Define the request operation with necessary configurations.
        // The `operation_name` corresponds to the ECS "DescribeRegions" operation.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::EcsOperation(
                    operation_config::operation_name_ecs::OperationNameEcs::DescribeRegions,
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
            .with_operation(&request_operation) // Attach the operation definition (DescribeRegions)
            .build()? // Build the request
            .send(request) // Send the request to ECS with the provided `DescribeRegionsReq`
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp = volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeRegionsResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
