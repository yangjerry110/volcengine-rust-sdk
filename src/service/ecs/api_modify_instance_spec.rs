/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 10:47:19
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:31:30
 * @Description: API to modify instance specifications in ECS (Elastic Compute Service)
 */
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use crate::{service::ecs, volcengine::error::error};
use volcengine_sdk_protobuf::protobuf::ecs_instance;

/// The `ApiModifyInstanceSpec` struct is used to interact with the Volcengine ECS service to modify
/// the specifications of an instance. It encapsulates the logic for constructing and sending
/// requests to update instance configurations.
pub struct ApiModifyInstanceSpec;

/// Implementation of methods for the `ApiModifyInstanceSpec` struct.
/// These methods handle the logic for modifying ECS instance specifications, including constructing
/// requests and processing responses.
impl ApiModifyInstanceSpec {
    /// The `new_modify_instance_spec` function creates a new API request to modify the specifications
    /// of an ECS instance. This function is the primary entry point for initiating an instance
    /// specification modification request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiModifyInstanceSpec`.
    /// - `ecs`: A reference to the `ecs::Ecs` struct, which contains the necessary ECS client information.
    /// - `request`: A `ModifyInstanceSpecReq` struct containing the parameters to modify the instance specifications.
    ///
    /// # Returns:
    /// This function returns a `Result<ecs_instance::ModifyInstanceSpecResp, error::Error>`.
    /// - On success, it returns a `ModifyInstanceSpecResp` struct with the response from the ECS API.
    /// - On failure, it returns an `error::Error` indicating the failure reason.
    pub async fn new_modify_instance_spec(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_instance::ModifyInstanceSpecReq,
    ) -> Result<ecs_instance::ModifyInstanceSpecResp, error::Error> {
        // Delegate the request creation and sending to the internal helper function.
        self.new_modify_instance_spec_request(ecs, request).await
    }

    /// The `new_modify_instance_spec_request` function constructs and sends the API request to modify
    /// the specifications of an ECS instance. This function handles the internal details of request
    /// creation and response parsing.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiModifyInstanceSpec`.
    /// - `ecs`: A reference to the `ecs::Ecs` struct, which contains ECS client information.
    /// - `request`: A `ModifyInstanceSpecReq` struct with parameters for modifying the instance specifications.
    ///
    /// # Returns:
    /// This function returns a `Result<ecs_instance::ModifyInstanceSpecResp, error::Error>`.
    /// - On success, it returns a `ModifyInstanceSpecResp` containing information about the modification result.
    /// - On failure, it returns an `error::Error` indicating the error reason.
    async fn new_modify_instance_spec_request(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_instance::ModifyInstanceSpecReq,
    ) -> Result<ecs_instance::ModifyInstanceSpecResp, error::Error> {
        // Define the request operation with necessary configurations.
        // The `operation_name` corresponds to the ECS "ModifyInstanceSpec" operation.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::EcsOperation(
                    operation_config::operation_name_ecs::OperationNameEcs::ModifyInstanceSpec,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            .build()?; // Build the operation and return a Result

        // Set up the request using the client information from the `ecs` struct.
        // The request is built with operation details and ECS client configuration.
        // After that, the request is sent using the `send` method and the response is processed.
        let response = request::Request::builder()
            .with_client_info(&ecs.client.client_info) // Attach client information (e.g., credentials)
            .with_config(&ecs.client.config) // Include configuration settings for the ECS client
            .with_handles(&ecs.client.handles) // Use the appropriate ECS client handles for this operation
            .with_operation(&request_operation) // Attach the operation definition (ModifyInstanceSpec)
            .build()? // Build the request
            .send(request) // Send the request to ECS with the provided `ModifyInstanceSpecReq`
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp =
            volcengine_sdk_protobuf::protobuf::ecs_instance::ModifyInstanceSpecResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
