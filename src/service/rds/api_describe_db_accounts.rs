/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-19 16:01:02
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:26:11
 * @Description: API for describing database accounts.
 */
use crate::service::rds;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::rds_account;

/// A struct representing the API for describing database accounts.
/// This struct encapsulates the functionality required to send a request to the Volcengine RDS service
/// to retrieve information about database accounts.
pub struct ApiDescribeDBAccountsRds;

/// Implementation of methods for the `ApiDescribeDBAccountsRds` struct.
/// This implementation provides the necessary logic to construct and send a request to the Volcengine RDS service
/// to describe database accounts, as well as handle the response.
impl ApiDescribeDBAccountsRds {
    /// Public method to describe database accounts.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiDescribeDBAccountsRds`.
    /// - `rds`: Reference to an `Rds` instance, which contains client information, configuration, and handles.
    /// - `request`: A `DescribeDbAccountsReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<rds_account::DescribeDbAccountsResp, error::Error>`: On success, returns a `DescribeDbAccountsResp` structure containing the response from the RDS service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    pub async fn new_describe_db_accounts(
        &self,
        rds: &rds::Rds,
        request: rds_account::DescribeDbAccountsReq,
    ) -> Result<rds_account::DescribeDbAccountsResp, error::Error> {
        // Delegate the request handling to the private method `new_describe_db_accounts_request`.
        self.new_describe_db_accounts_request(rds, request).await
    }

    /// Private method to handle the request to describe database accounts.
    ///
    /// This method constructs the request operation, builds the request, sends it to the Volcengine RDS service,
    /// and parses the response.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiDescribeDBAccountsRds`.
    /// - `rds`: Reference to an `Rds` instance, which contains client information, configuration, and handles.
    /// - `request`: A `DescribeDbAccountsReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<rds_account::DescribeDbAccountsResp, error::Error>`: On success, returns a `DescribeDbAccountsResp` structure containing the response from the RDS service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_describe_db_accounts_request(
        &self,
        rds: &rds::Rds,
        request: rds_account::DescribeDbAccountsReq,
    ) -> Result<rds_account::DescribeDbAccountsResp, error::Error> {
        // Define the request operation with the specific operation name, HTTP method, and path.
        // The operation name corresponds to the "DescribeDBAccounts" action in the Volcengine RDS service.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::RdsOperation(
                    operation_config::operation_name_rds::OperationNameRds::DescribeDBAccounts,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?;

        // Build the Volcengine request using the client information, configuration, handles, and the defined operation.
        // The request is then sent to the RDS service, and the response is awaited.
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
            volcengine_sdk_protobuf::protobuf::rds_account::DescribeDbAccountsResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
