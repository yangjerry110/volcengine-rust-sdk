/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:47:46
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:23:35
 * @Description: api create db account
 */
use crate::service::rds;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::rds_account;

/// Represents an API client for creating a database account in the RDS service.
/// This struct provides methods to send requests for creating a new database account.
pub struct ApiCreateDbAccountRds;

/// Implementation of the methods for the `ApiCreateDbAccountRds` struct.
impl ApiCreateDbAccountRds {
    /// Initiates a new API call to create a database account.
    ///
    /// This method serves as a high - level wrapper around the actual request - sending process.
    /// It calls `new_create_db_account_request` to perform the underlying operations.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current `ApiCreateDbAccountRds` instance.
    /// - `rds`: A reference to the `Rds` service instance, which contains client configuration
    ///          such as authentication details and endpoint information.
    /// - `request`: A `CreateDbAccountReq` struct that holds the necessary data for creating a database account,
    ///              like account name, password, and other relevant settings.
    ///
    /// # Returns
    /// - On success, it returns a `CreateDbAccountResp` struct containing the response data from the RDS service,
    ///   which may include details about the newly created account.
    /// - On error, it returns an `Error` struct indicating the reason for the failure,
    ///   such as network issues, authentication errors, or invalid request parameters.
    pub async fn new_create_db_account(
        &self,
        rds: &rds::Rds,
        request: rds_account::CreateDbAccountReq,
    ) -> Result<rds_account::CreateDbAccountResp, error::Error> {
        // Delegate the actual request - sending to the `new_create_db_account_request` method
        self.new_create_db_account_request(rds, request).await
    }

    /// Creates and sends a new API request to create a database account.
    ///
    /// This method constructs the request operation, builds the Volcengine request,
    /// sends the request to the RDS service, and parses the response.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current `ApiCreateDbAccountRds` instance.
    /// - `rds`: A reference to the `Rds` service instance, which contains client configuration
    ///          such as authentication details and endpoint information.
    /// - `request`: A `CreateDbAccountReq` struct that holds the necessary data for creating a database account,
    ///              like account name, password, and other relevant settings.
    ///
    /// # Returns
    /// - On success, it returns a `CreateDbAccountResp` struct containing the response data from the RDS service,
    ///   which may include details about the newly created account.
    /// - On error, it returns an `Error` struct indicating the reason for the failure,
    ///   such as network issues, authentication errors, or invalid request parameters.
    async fn new_create_db_account_request(
        &self,
        rds: &rds::Rds,
        request: rds_account::CreateDbAccountReq,
    ) -> Result<rds_account::CreateDbAccountResp, error::Error> {
        // Define the request operation
        // Set the operation name, HTTP method, and HTTP path for the API request
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                // Specify the RDS operation type as creating a database account
                operation_config::operation_name::OperationName::RdsOperation(
                    operation_config::operation_name_rds::OperationNameRds::CreateDBAccount,
                ),
            )
            .with_operation_http_method(
                // Use the POST HTTP method for creating a new resource
                operation_config::operation_http_method::OperationHttpMethod::POST,
            )
            .with_operation_http_path(
                // Use the default HTTP path for this operation
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?;

        // Build and send the request
        // Construct the Volcengine request using the client information, configuration,
        // handlers, and the defined request operation. Then send the `CreateDbAccountReq`
        // request and parse the response as a `CreateDbAccountResp` struct.
        let response = request::Request::builder()
            .with_client_info(&rds.client.client_info)
            .with_config(&rds.client.config)
            .with_handles(&rds.client.handles)
            .with_operation(&request_operation)
            .build()?
            .send(request)
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp =
            volcengine_sdk_protobuf::protobuf::rds_account::CreateDbAccountResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
