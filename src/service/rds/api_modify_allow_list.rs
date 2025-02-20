/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 14:39:42
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:28:22
 * @Description: API for modifying the allow list of an RDS instance.
 */
use crate::service::rds;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::rds_allow;

/// A struct representing the API for modifying the allow list of an RDS instance.
/// This struct encapsulates the functionality required to send a request to the Volcengine RDS service
/// to modify the allow list of a specific database instance.
pub struct ApiModifyAllowListRds;

/// Implementation of methods for the `ApiModifyAllowListRds` struct.
/// This implementation provides the necessary logic to construct and send a request to the Volcengine RDS service
/// to modify the allow list of a database instance, as well as handle the response.
impl ApiModifyAllowListRds {
    /// Public method to modify the allow list of a database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiModifyAllowListRds`.
    /// - `rds`: Reference to an `Rds` instance, which contains client information, configuration, and handles.
    /// - `request`: A `ModifyAllowListReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<rds_allow::ModifyAllowListResp, error::Error>`: On success, returns a `ModifyAllowListResp` structure containing the response from the RDS service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    pub async fn new_modify_allow_list(
        &self,
        rds: &rds::Rds,
        request: rds_allow::ModifyAllowListReq,
    ) -> Result<rds_allow::ModifyAllowListResp, error::Error> {
        // Delegate the request handling to the private method `new_modify_allow_list_request`.
        self.new_modify_allow_list_request(rds, request).await
    }

    /// Private method to handle the request to modify the allow list of a database instance.
    ///
    /// This method constructs the request operation, builds the request, sends it to the Volcengine RDS service,
    /// and parses the response.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiModifyAllowListRds`.
    /// - `rds`: Reference to an `Rds` instance, which contains client information, configuration, and handles.
    /// - `request`: A `ModifyAllowListReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<rds_allow::ModifyAllowListResp, error::Error>`: On success, returns a `ModifyAllowListResp` structure containing the response from the RDS service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_modify_allow_list_request(
        &self,
        rds: &rds::Rds,
        request: rds_allow::ModifyAllowListReq,
    ) -> Result<rds_allow::ModifyAllowListResp, error::Error> {
        // Define the request operation with the specific operation name, HTTP method, and path.
        // The operation name corresponds to the "ModifyAllowList" action in the Volcengine RDS service.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::RdsOperation(
                    operation_config::operation_name_rds::OperationNameRds::ModifyAllowList,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::POST,
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
        let mut resp = volcengine_sdk_protobuf::protobuf::rds_allow::ModifyAllowListResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
