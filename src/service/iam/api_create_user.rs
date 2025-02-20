/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 14:39:42
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:35:44
 * @Description: api create user
 */
use crate::service::iam::Iam;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;

/// Struct for creating a user in IAM.
/// This struct encapsulates the logic for making API requests to create a new user.
pub struct ApiCreateUserIam;

/// Implementation of methods for the ApiCreateUserIam struct.
/// These methods handle the logic for creating a user, including constructing requests and processing responses.
impl ApiCreateUserIam {
    /// Creates a new API request to create a user in IAM.
    /// This method is the primary entry point for initiating the user creation request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiCreateUserIam`.
    /// - `iam`: A reference to the `Iam` struct, which contains the necessary IAM client information.
    /// - `request`: A `CreateUserReq` struct containing the parameters to create a user.
    ///
    /// # Returns:
    /// A `Result` containing a `CreateUserResp` on success or an `error::Error` on failure.
    pub async fn new_create_user(
        &self,
        iam: &Iam,
        request: volcengine_sdk_protobuf::protobuf::iam_user::CreateUserReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::CreateUserResp, error::Error> {
        // Delegates the request creation and sending to the internal helper function.
        self.new_create_user_request(iam, request).await
    }

    /// Internal function to create and send the request to create a user in IAM.
    /// This function constructs the request operation and sends it using the Volcengine request system.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiCreateUserIam`.
    /// - `iam`: A reference to the `Iam` struct, which contains the necessary IAM client information.
    /// - `request`: A `CreateUserReq` struct containing the parameters to create a user.
    ///
    /// # Returns:
    /// A `Result` containing a `CreateUserResp` on success or an `error::Error` on failure.
    async fn new_create_user_request(
        &self,
        iam: &Iam,
        request: volcengine_sdk_protobuf::protobuf::iam_user::CreateUserReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::CreateUserResp, error::Error> {
        // Define the request operation with necessary configurations.
        // The `operation_name` corresponds to the IAM "CreateUser" operation.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::IamOperation(
                    operation_config::operation_name_iam::OperationNameIam::CreateUser,
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
            .with_operation(&request_operation) // Attach the operation definition (CreateUser)
            .build()? // Build the request
            .send(request) // Send the request to IAM with the provided `CreateUserReq`
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp = volcengine_sdk_protobuf::protobuf::iam_user::CreateUserResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
