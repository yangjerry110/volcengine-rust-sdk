/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 14:39:42
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:34:09
 * @Description: api create login profile
 */
use crate::service::iam::Iam;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::iam_user;

/// Struct for creating a login profile in IAM.
/// This struct encapsulates the logic for making API requests to create a login profile for an IAM user.
pub struct ApiCreateLoginProfileIam;

/// Implementation of methods for the ApiCreateLoginProfileIam struct.
/// These methods handle the logic for creating a login profile, including constructing requests and processing responses.
impl ApiCreateLoginProfileIam {
    /// Creates a new API request to create a login profile for an IAM user.
    /// This method is the primary entry point for initiating the login profile creation request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiCreateLoginProfileIam`.
    /// - `iam`: A reference to the `Iam` struct, which contains the necessary IAM client information.
    /// - `request`: A `CreateLoginProfileReq` struct containing the parameters to create a login profile.
    ///
    /// # Returns:
    /// A `Result` containing a `CreateLoginProfileResp` on success or an `error::Error` on failure.
    pub async fn new_create_login_profile(
        &self,
        iam: &Iam,
        request: iam_user::CreateLoginProfileReq,
    ) -> Result<iam_user::CreateLoginProfileResp, error::Error> {
        // Delegates the request creation and sending to the internal helper function.
        self.new_create_login_profile_request(iam, request).await
    }

    /// Internal function to create and send the request to create a login profile for an IAM user.
    /// This function constructs the request operation and sends it using the Volcengine request system.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiCreateLoginProfileIam`.
    /// - `iam`: A reference to the `Iam` struct, which contains the necessary IAM client information.
    /// - `request`: A `CreateLoginProfileReq` struct containing the parameters to create a login profile.
    ///
    /// # Returns:
    /// A `Result` containing a `CreateLoginProfileResp` on success or an `error::Error` on failure.
    async fn new_create_login_profile_request(
        &self,
        iam: &Iam,
        request: iam_user::CreateLoginProfileReq,
    ) -> Result<iam_user::CreateLoginProfileResp, error::Error> {
        // Define the request operation with necessary configurations.
        // The `operation_name` corresponds to the IAM "CreateLoginProfile" operation.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::IamOperation(
                    operation_config::operation_name_iam::OperationNameIam::CreateLoginProfile,
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
            .with_operation(&request_operation) // Attach the operation definition (CreateLoginProfile)
            .build()? // Build the request
            .send(request) // Send the request to IAM with the provided `CreateLoginProfileReq`
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp =
            volcengine_sdk_protobuf::protobuf::iam_user::CreateLoginProfileResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
