/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 14:39:42
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:39:59
 * @Description: api get security config
 */
use crate::service::iam::Iam;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::iam_user;

/// Represents an API for getting the security configuration in the IAM (Identity and Access Management) service.
/// This struct provides methods to perform operations related to retrieving the security configuration.
pub struct ApiGetSecurityConfigIam;

/// Implementation of methods for the `ApiGetSecurityConfigIam` struct.
/// These methods are used to create and send requests for getting the security configuration in the IAM service.
impl ApiGetSecurityConfigIam {
    /// Creates a new API call to get the security configuration.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `ApiGetSecurityConfigIam` instance.
    /// - `iam`: A reference to the `Iam` service instance.
    /// - `request`: A `GetSecurityConfigReq` struct containing the request details for getting the security configuration.
    ///
    /// # Returns
    /// - On success, returns a `GetSecurityConfigResp` struct containing the response details.
    /// - On error, returns an `Error` struct indicating the reason for the failure.
    pub async fn new_get_security_config(
        &self,
        iam: &Iam,
        request: iam_user::GetSecurityConfigReq,
    ) -> Result<iam_user::GetSecurityConfigResp, error::Error> {
        self.new_get_security_config_request(iam, request).await
    }

    /// Creates and sends a new API request to get the security configuration.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `ApiGetSecurityConfigIam` instance.
    /// - `iam`: A reference to the `Iam` service instance.
    /// - `request`: A `GetSecurityConfigReq` struct containing the request details for getting the security configuration.
    ///
    /// # Returns
    /// - On success, returns a `GetSecurityConfigResp` struct containing the response details.
    /// - On error, returns an `Error` struct indicating the reason for the failure.
    async fn new_get_security_config_request(
        &self,
        iam: &Iam,
        request: iam_user::GetSecurityConfigReq,
    ) -> Result<iam_user::GetSecurityConfigResp, error::Error> {
        // Define the request operation
        // Set the operation name, HTTP method, and HTTP path for the request
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::IamOperation(
                    operation_config::operation_name_iam::OperationNameIam::GetSecurityConfig,
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
        let mut resp =
            volcengine_sdk_protobuf::protobuf::iam_user::GetSecurityConfigResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
