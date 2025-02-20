/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 15:46:30
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:20:59
 * @Description: api update login profile
 */
use crate::service::iam::Iam;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::iam_user;

/// Represents an API for updating the login profile in the IAM (Identity and Access Management) service.
/// This struct provides methods to perform operations related to updating the login profile of a user.
pub struct ApiUpdateLoginProfileIam;

/// Implementation of methods for the `ApiUpdateLoginProfileIam` struct.
/// These methods are used to create and send requests for updating the login profile in the IAM service.
impl ApiUpdateLoginProfileIam {
    /// Creates a new API call to update the login profile.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `ApiUpdateLoginProfileIam` instance.
    /// - `iam`: A reference to the `Iam` service instance.
    /// - `request`: A `UpdateLoginProfileReq` struct containing the request details for updating the login profile.
    ///
    /// # Returns
    /// - On success, returns a `UpdateLoginProfileResp` struct containing the response details.
    /// - On error, returns an `Error` struct indicating the reason for the failure.
    pub async fn new_update_login_profile(
        &self,
        iam: &Iam,
        request: iam_user::UpdateLoginProfileReq,
    ) -> Result<iam_user::UpdateLoginProfileResp, error::Error> {
        self.new_update_login_profile_request(iam, request).await
    }

    /// Creates and sends a new API request to update the login profile.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `ApiUpdateLoginProfileIam` instance.
    /// - `iam`: A reference to the `Iam` service instance.
    /// - `request`: A `UpdateLoginProfileReq` struct containing the request details for updating the login profile.
    ///
    /// # Returns
    /// - On success, returns a `UpdateLoginProfileResp` struct containing the response details.
    /// - On error, returns an `Error` struct indicating the reason for the failure.
    async fn new_update_login_profile_request(
        &self,
        iam: &Iam,
        request: iam_user::UpdateLoginProfileReq,
    ) -> Result<iam_user::UpdateLoginProfileResp, error::Error> {
        // Define the request operation
        // Configure the operation name, HTTP method, and HTTP path for the request
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::IamOperation(
                    operation_config::operation_name_iam::OperationNameIam::UpdateLoginProfile,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?;

        // Build and send the request
        // Construct the request with client information, configuration, handles, and operation details
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
        let mut resp =
            volcengine_sdk_protobuf::protobuf::iam_user::UpdateLoginProfileResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
