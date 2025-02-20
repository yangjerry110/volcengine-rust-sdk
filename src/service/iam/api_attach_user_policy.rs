/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-06 10:56:34
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:33:37
 * @Description: API to attach a user policy in IAM (Identity and Access Management)
 */
use crate::service::iam;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::iam_policy;

/**
 * Struct for interacting with the IAM service to attach a user policy.
 * This struct encapsulates the logic for making API requests to attach a policy to an IAM user.
 */
pub struct ApiAttachUserPolicyIam;

/**
 * Implementation of methods for the ApiAttachUserPolicyIam struct.
 * These methods handle the logic for attaching a user policy to an IAM user, including constructing requests and processing responses.
 */
impl ApiAttachUserPolicyIam {
    /**
     * Creates a new API request to attach a user policy to an IAM user.
     * This method is the primary entry point for initiating the policy attachment request.
     *
     * # Parameters:
     * - `&self`: A reference to the current instance of `ApiAttachUserPolicyIam`.
     * - `iam`: A reference to the `iam::Iam` struct, which contains the necessary IAM client information.
     * - `request`: An `AttachUserPolicyReq` struct containing the parameters to attach a user policy.
     *
     * # Returns:
     * A `Result` containing an `AttachUserPolicyResp` on success or an `error::Error` on failure.
     */
    pub async fn new_attach_user_policy(
        &self,
        iam: &iam::Iam,
        request: iam_policy::AttachUserPolicyReq,
    ) -> Result<iam_policy::AttachUserPolicyResp, error::Error> {
        // Delegate the request creation and sending to the internal helper function.
        self.new_attach_user_policy_request(iam, request).await
    }

    /**
     * Internal function to create and send the request to attach a user policy to an IAM user.
     * This function constructs the request operation and sends it using the Volcengine request system.
     *
     * # Parameters:
     * - `&self`: A reference to the current instance of `ApiAttachUserPolicyIam`.
     * - `iam`: A reference to the `iam::Iam` struct, which contains the necessary IAM client information.
     * - `request`: An `AttachUserPolicyReq` struct containing the parameters to attach a user policy.
     *
     * # Returns:
     * A `Result` containing an `AttachUserPolicyResp` on success or an `error::Error` on failure.
     */
    async fn new_attach_user_policy_request(
        &self,
        iam: &iam::Iam,
        request: iam_policy::AttachUserPolicyReq,
    ) -> Result<iam_policy::AttachUserPolicyResp, error::Error> {
        // Define the request operation with necessary configurations.
        // The `operation_name` corresponds to the IAM "AttachUserPolicy" operation.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::IamOperation(
                    operation_config::operation_name_iam::OperationNameIam::AttachUserPolicy,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?; // Build the operation and return a Result

        // Set up the request using the client information from the `iam` struct.
        // The request is built with operation details and IAM client configuration.
        // After that, the request is sent using the `send` method and the response is processed.
        let response = request::Request::builder()
            .with_client_info(&iam.client.client_info) // Attach client information (e.g., credentials)
            .with_config(&iam.client.config) // Include configuration settings for the IAM client
            .with_handles(&iam.client.handles) // Use the appropriate IAM client handles for this operation
            .with_operation(&request_operation) // Attach the operation definition (AttachUserPolicy)
            .build()? // Build the request
            .send(request) // Send the request to IAM with the provided `AttachUserPolicyReq`
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp =
            volcengine_sdk_protobuf::protobuf::iam_policy::AttachUserPolicyResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
