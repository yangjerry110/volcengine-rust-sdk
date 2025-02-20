/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 10:36:18
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:32:31
 * @Description: API to stop an instance in ECS (Elastic Compute Service)
 */
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use crate::{service::ecs, volcengine::error::error};
use volcengine_sdk_protobuf::protobuf::ecs_instance;

/// The `ApiStopInstance` struct provides functionality to stop instances in ECS (Elastic Compute Service).
/// It encapsulates the logic for making API requests to the Volcengine ECS service to stop running instances.
pub struct ApiStopInstance;

/// Implementation of methods for the `ApiStopInstance` struct.
/// These methods handle the logic for stopping ECS instances, including constructing requests and processing responses.
impl ApiStopInstance {
    /// The `new_stop_instance` function creates a new API request to stop an ECS instance.
    /// This function is the primary entry point for initiating an instance stop request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiStopInstance`.
    /// - `ecs`: A reference to the `ecs::Ecs` struct, which contains the necessary ECS client information.
    /// - `request`: A `StopInstanceReq` struct containing the parameters to stop an ECS instance.
    ///
    /// # Returns:
    /// This function returns a `Result<ecs_instance::StopInstanceResp, error::Error>`.
    /// - On success, it returns a `StopInstanceResp` struct with the response from the ECS API.
    /// - On failure, it returns an `error::Error` indicating the failure reason.
    pub async fn new_stop_instance(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_instance::StopInstanceReq,
    ) -> Result<ecs_instance::StopInstanceResp, error::Error> {
        // Delegate the request creation and sending to the internal helper function.
        self.new_stop_instance_request(ecs, request).await
    }

    /// The `new_stop_instance_request` function constructs and sends the API request to stop an ECS instance.
    /// This function handles the internal details of request creation and response parsing.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiStopInstance`.
    /// - `ecs`: A reference to the `ecs::Ecs` struct, which contains ECS client information.
    /// - `request`: A `StopInstanceReq` struct with parameters for stopping an ECS instance.
    ///
    /// # Returns:
    /// This function returns a `Result<ecs_instance::StopInstanceResp, error::Error>`.
    /// - On success, it returns a `StopInstanceResp` containing information about the stopped instance.
    /// - On failure, it returns an `error::Error` indicating the error reason.
    async fn new_stop_instance_request(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_instance::StopInstanceReq,
    ) -> Result<ecs_instance::StopInstanceResp, error::Error> {
        // Define the request operation with necessary configurations.
        // The `operation_name` corresponds to the ECS "StopInstance" operation.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::EcsOperation(
                    operation_config::operation_name_ecs::OperationNameEcs::StopInstance,
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
            .with_operation(&request_operation) // Attach the operation definition (StopInstance)
            .build()? // Build the request
            .send(request) // Send the request to ECS with the provided `StopInstanceReq`
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp = volcengine_sdk_protobuf::protobuf::ecs_instance::StopInstanceResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
