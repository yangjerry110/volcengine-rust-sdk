/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-28 17:16:53
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:32:03
 * @Description: API to run instances in ECS (Elastic Compute Service)
 */
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use crate::{service::ecs, volcengine::error::error};
use volcengine_sdk_protobuf::protobuf::ecs_instance;

/// The `ApiRunInstance` struct provides functionality to run instances in ECS (Elastic Compute Service).
/// It encapsulates the logic for making API requests to the Volcengine ECS service to create and start new instances.
pub struct ApiRunInstance;

/// Implementation of methods for the `ApiRunInstance` struct.
/// These methods handle the logic for creating and starting new ECS instances, including constructing requests and processing responses.
impl ApiRunInstance {
    /// The `new_run_instances` function creates a new API request to run instances in ECS.
    /// This function is the primary entry point for initiating an instance creation and start request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiRunInstance`.
    /// - `ecs`: A reference to the `ecs::Ecs` struct, which contains the necessary ECS client information.
    /// - `request`: A `RunInstancesReq` struct containing the parameters to create and start new instances.
    ///
    /// # Returns:
    /// This function returns a `Result<ecs_instance::RunInstancesResp, error::Error>`.
    /// - On success, it returns a `RunInstancesResp` struct with the response from the ECS API.
    /// - On failure, it returns an `error::Error` indicating the failure reason.
    pub async fn new_run_instances(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_instance::RunInstancesReq,
    ) -> Result<ecs_instance::RunInstancesResp, error::Error> {
        // Delegate the request creation and sending to the internal helper function.
        self.new_run_instance_request(ecs, request).await
    }

    /// The `new_run_instance_request` function constructs and sends the API request to run instances in ECS.
    /// This function handles the internal details of request creation and response parsing.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiRunInstance`.
    /// - `ecs`: A reference to the `ecs::Ecs` struct, which contains ECS client information.
    /// - `request`: A `RunInstancesReq` struct with parameters for creating and starting new instances.
    ///
    /// # Returns:
    /// This function returns a `Result<ecs_instance::RunInstancesResp, error::Error>`.
    /// - On success, it returns a `RunInstancesResp` containing information about the created instances.
    /// - On failure, it returns an `error::Error` indicating the error reason.
    async fn new_run_instance_request(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_instance::RunInstancesReq,
    ) -> Result<ecs_instance::RunInstancesResp, error::Error> {
        // Define the request operation with necessary configurations.
        // The `operation_name` corresponds to the ECS "RunInstances" operation.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::EcsOperation(
                    operation_config::operation_name_ecs::OperationNameEcs::RunInstances,
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
            .with_operation(&request_operation) // Attach the operation definition (RunInstances)
            .build()? // Build the request
            .send(request) // Send the request to ECS with the provided `RunInstancesReq`
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp = volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
