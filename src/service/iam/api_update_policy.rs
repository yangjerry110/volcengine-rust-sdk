/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-06 10:56:34
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:22:15
 * @Description: create policy
 */
use crate::service::iam;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::iam_policy;

/// Represents an API for updating a policy in the IAM (Identity and Access Management) service.
/// This struct provides methods to perform operations related to updating policies.
pub struct ApiUpdatePolicyIam;

/// Implementation of methods for the `ApiUpdatePolicyIam` struct.
/// These methods are used to create and send requests for updating policies in the IAM service.
impl ApiUpdatePolicyIam {
    /// Creates a new API call to update a policy.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `ApiUpdatePolicyIam` instance.
    /// - `iam`: A reference to the `Iam` service instance.
    /// - `request`: An `UpdatePolicyReq` struct containing the request details for updating a policy.
    ///
    /// # Returns
    /// - On success, returns an `UpdatePolicyResp` struct containing the response details.
    /// - On error, returns an `Error` struct indicating the reason for the failure.
    pub async fn new_update_policy(
        &self,
        iam: &iam::Iam,
        request: iam_policy::UpdatePolicyReq,
    ) -> Result<iam_policy::UpdatePolicyResp, error::Error> {
        self.new_update_policy_request(iam, request).await
    }

    /// Creates and sends a new API request to update a policy.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `ApiUpdatePolicyIam` instance.
    /// - `iam`: A reference to the `Iam` service instance.
    /// - `request`: An `UpdatePolicyReq` struct containing the request details for updating a policy.
    ///
    /// # Returns
    /// - On success, returns an `UpdatePolicyResp` struct containing the response details.
    /// - On error, returns an `Error` struct indicating the reason for the failure.
    async fn new_update_policy_request(
        &self,
        iam: &iam::Iam,
        request: iam_policy::UpdatePolicyReq,
    ) -> Result<iam_policy::UpdatePolicyResp, error::Error> {
        // Define the request operation
        // Set the operation name, HTTP method, and HTTP path for the request
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::IamOperation(
                    operation_config::operation_name_iam::OperationNameIam::UpdatePolicy,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?;

        // Set up the request and send it
        // Build the request with client information, configuration, handles, and operation details
        // Then send the request, parse the JSON response, and handle any errors that occur during the process
        let response = request::Request::builder()
            .with_client_info(&iam.client.client_info)
            .with_config(&iam.client.config)
            .with_handles(&iam.client.handles)
            .with_operation(&request_operation)
            .build()?
            .send(request)
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp = volcengine_sdk_protobuf::protobuf::iam_policy::UpdatePolicyResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
