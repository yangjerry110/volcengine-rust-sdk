/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 10:25:22
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:30:03
 * @Description: API to describe instances in ECS (Elastic Compute Service)
 */
use crate::service::ecs;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::ecs_instance;

/// `ApiDescribeInstances` is a struct that provides functionality to describe instances in ECS (Elastic Compute Service).
/// It encapsulates the logic for making API requests to the Volcengine ECS service to retrieve information about instances.
pub struct ApiDescribeInstances;

/// Implementation of core functionality for the `ApiDescribeInstances` struct.
///
/// This implementation provides methods to interact with Volcengine's ECS (Elastic Compute Service)
/// DescribeInstances API. It handles request construction, execution, and response parsing.
impl ApiDescribeInstances {
    /// Creates a new API request to describe ECS instances.
    ///
    /// This method handles the description of ECS instances by using the `DescribeInstancesReq` request.
    /// It is the primary entry point for interacting with the ECS "DescribeInstances" API.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiDescribeInstances`.
    /// - `ecs`: A reference to the `ecs::Ecs` struct, which contains the necessary ECS client information.
    /// - `request`: A `DescribeInstancesReq` struct containing the parameters to describe ECS instances.
    ///
    /// # Returns:
    /// This function returns a `Result<ecs_instance::DescribeInstancesResp, error::Error>`.
    /// - On success, it returns a `DescribeInstancesResp` struct with the response from the ECS API.
    /// - On failure, it returns an `error::Error` indicating the failure reason.
    pub async fn new_describe_instances(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_instance::DescribeInstancesReq,
    ) -> Result<ecs_instance::DescribeInstancesResp, error::Error> {
        // Calls the internal helper function to initiate the ECS describe instances request.
        self.new_describe_instances_request(ecs, request).await
    }

    /// Internal function to create and send the request to describe ECS instances.
    ///
    /// This function constructs the request operation and sends it using the Volcengine request system.
    /// It ensures that the necessary operation configurations are applied and the request is properly executed.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiDescribeInstances`.
    /// - `ecs`: A reference to the `ecs::Ecs` struct, which contains ECS client information.
    /// - `request`: A `DescribeInstancesReq` struct with parameters for describing ECS instances.
    ///
    /// # Returns:
    /// This function returns a `Result<ecs_instance::DescribeInstancesResp, error::Error>`.
    /// - On success, it returns a `DescribeInstancesResp` containing information about the described instances.
    /// - On failure, it returns an `error::Error` indicating the error reason.
    async fn new_describe_instances_request(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_instance::DescribeInstancesReq,
    ) -> Result<ecs_instance::DescribeInstancesResp, error::Error> {
        // Define the request operation with necessary configurations.
        // The `operation_name` corresponds to the ECS "DescribeInstances" operation.
        let request_operation = operation::Operation::builder()
            // Set the operation name to "DescribeInstances" which corresponds to the ECS API for describing instances.
            .with_operation_name(
                operation_config::operation_name::OperationName::EcsOperation(
                    operation_config::operation_name_ecs::OperationNameEcs::DescribeInstances,
                ),
            )
            // Specify the HTTP method for this operation. In this case, it's a GET request.
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            // Set the HTTP path for the operation. Here, the default path is used.
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            // Build the operation object, ensuring all configurations are properly set.
            .build()?;

        // Set up the request using the client information from the `ecs` struct.
        // The request is built with operation details and ECS client configuration.
        // After that, the request is sent using the `send` method and the response is processed.
        let response = request::Request::builder()
            // Attach client information (e.g., credentials) from the ECS client.
            .with_client_info(&ecs.client.client_info)
            // Include configuration settings for the ECS client.
            .with_config(&ecs.client.config)
            // Use the appropriate ECS client handles for this operation.
            .with_handles(&ecs.client.handles)
            // Attach the operation definition (DescribeInstances) to the request.
            .with_operation(&request_operation)
            // Build the request.
            .build()?
            // Send the request to ECS with the provided `DescribeInstancesReq`.
            .send(request)
            // Await the response asynchronously.
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for ECS instance descriptions.
        let mut resp =
            volcengine_sdk_protobuf::protobuf::ecs_instance::DescribeInstancesResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
