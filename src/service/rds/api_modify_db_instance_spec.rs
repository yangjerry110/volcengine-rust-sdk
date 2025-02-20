/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-19 11:16:43
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:29:11
 * @Description: API for modifying the specifications of a database instance.
 */
use crate::service::rds;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::rds_instance;

/// A struct representing the API for modifying the specifications of a database instance.
/// This struct encapsulates the functionality required to send a request to the Volcengine RDS service
/// to modify the specifications (e.g., CPU, memory, storage) of a specific database instance.
pub struct ApiModifyDBInstanceSpecRds;

/// Implementation of methods for the `ApiModifyDBInstanceSpecRds` struct.
/// This implementation provides the necessary logic to construct and send a request to the Volcengine RDS service
/// to modify the specifications of a database instance, as well as handle the response.
impl ApiModifyDBInstanceSpecRds {
    /// Public method to modify the specifications of a database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiModifyDBInstanceSpecRds`.
    /// - `rds`: Reference to an `Rds` instance, which contains client information, configuration, and handles.
    /// - `request`: A `ModifyDbInstanceSpecReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<rds_instance::ModifyDbInstanceSpecResp, error::Error>`: On success, returns a `ModifyDbInstanceSpecResp` structure containing the response from the RDS service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    pub async fn new_modify_db_instance_spec(
        &self,
        rds: &rds::Rds,
        request: rds_instance::ModifyDbInstanceSpecReq,
    ) -> Result<rds_instance::ModifyDbInstanceSpecResp, error::Error> {
        // Delegate the request handling to the private method `new_modify_db_instance_spec_request`.
        self.new_modify_db_instance_spec_request(rds, request).await
    }

    /// Private method to handle the request to modify the specifications of a database instance.
    ///
    /// This method constructs the request operation, builds the request, sends it to the Volcengine RDS service,
    /// and parses the response.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiModifyDBInstanceSpecRds`.
    /// - `rds`: Reference to an `Rds` instance, which contains client information, configuration, and handles.
    /// - `request`: A `ModifyDbInstanceSpecReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<rds_instance::ModifyDbInstanceSpecResp, error::Error>`: On success, returns a `ModifyDbInstanceSpecResp` structure containing the response from the RDS service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_modify_db_instance_spec_request(
        &self,
        rds: &rds::Rds,
        request: rds_instance::ModifyDbInstanceSpecReq,
    ) -> Result<rds_instance::ModifyDbInstanceSpecResp, error::Error> {
        // Define the request operation with the specific operation name, HTTP method, and path.
        // The operation name corresponds to the "ModifyDBInstanceSpec" action in the Volcengine RDS service.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::RdsOperation(
                    operation_config::operation_name_rds::OperationNameRds::ModifyDBInstanceSpec,
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
        let mut resp =
            volcengine_sdk_protobuf::protobuf::rds_instance::ModifyDbInstanceSpecResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
