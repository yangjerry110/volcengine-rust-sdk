/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 14:55:04
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:23:00
 * @Description: api update user
 */
use crate::service::iam::Iam;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::iam_user;

/// Represents an API for updating user information in the IAM (Identity and Access Management) service.
/// This struct provides methods to perform operations related to updating user details.
pub struct ApiUpdateUserIam;

/// Implementation of methods for the `ApiUpdateUserIam` struct.
/// These methods are used to create and send requests for updating user information in the IAM service.
impl ApiUpdateUserIam {
    /// Initiates a new API call to update user information.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current `ApiUpdateUserIam` instance.
    /// - `iam`: A reference to the `Iam` service instance, which contains client - related information.
    /// - `request`: A `UpdateUserReq` struct that holds the request details for updating user information.
    ///
    /// # Returns
    /// - On success, returns an `UpdateUserResp` struct containing the response details.
    /// - On error, returns an `Error` struct indicating the reason for the failure.
    pub async fn new_update_user(
        &self,
        iam: &Iam,
        request: iam_user::UpdateUserReq,
    ) -> Result<iam_user::UpdateUserResp, error::Error> {
        self.new_update_user_request(iam, request).await
    }

    /// Creates and sends a new API request to update user information.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current `ApiUpdateUserIam` instance.
    /// - `iam`: A reference to the `Iam` service instance, which contains client - related information.
    /// - `request`: A `UpdateUserReq` struct that holds the request details for updating user information.
    ///
    /// # Returns
    /// - On success, returns an `UpdateUserResp` struct containing the response details.
    /// - On error, returns an `Error` struct indicating the reason for the failure.
    async fn new_update_user_request(
        &self,
        iam: &Iam,
        request: iam_user::UpdateUserReq,
    ) -> Result<iam_user::UpdateUserResp, error::Error> {
        // Define the request operation
        // Set the operation name, HTTP method, and HTTP path for the API request
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::IamOperation(
                    operation_config::operation_name_iam::OperationNameIam::UpdateUser,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?;

        // Construct and send the Volcengine request
        // Build the request with client information, configuration, handlers, and the defined operation
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
        let mut resp = volcengine_sdk_protobuf::protobuf::iam_user::UpdateUserResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
