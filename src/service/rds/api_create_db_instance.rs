/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:47:46
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:25:41
 * @Description: API for creating a database instance.
 */
use crate::service::rds;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::rds_instance;

/// A struct representing the API for creating a database instance in RDS.
/// This struct encapsulates the functionality required to send a request to the Volcengine RDS service
/// to create a new database instance.
pub struct ApiCreateDbInstanceRds;

/// Implementation of methods for the `ApiCreateDbInstanceRds` struct.
/// This implementation provides the necessary logic to construct and send a request to the Volcengine RDS service
/// to create a new database instance, as well as handle the response.
impl ApiCreateDbInstanceRds {
    /// Public method to create a new database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiCreateDbInstanceRds`.
    /// - `rds`: Reference to an `Rds` instance, which contains client information, configuration, and handles.
    /// - `request`: A `CreateDbInstanceReq` structure containing the details of the database instance to be created.
    ///
    /// # Returns
    /// - `Result<rds_instance::CreateDbInstanceResp, error::Error>`: On success, returns a `CreateDbInstanceResp` structure containing the response from the RDS service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    pub async fn new_create_db_instance(
        &self,
        rds: &rds::Rds,
        request: rds_instance::CreateDbInstanceReq,
    ) -> Result<rds_instance::CreateDbInstanceResp, error::Error> {
        // Delegate the request handling to the private method `new_create_db_instance_request`.
        self.new_create_db_instance_request(rds, request).await
    }

    /// Private method to handle the request to create a new database instance.
    ///
    /// This method constructs the request operation, builds the request, sends it to the Volcengine RDS service,
    /// and parses the response.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiCreateDbInstanceRds`.
    /// - `rds`: Reference to an `Rds` instance, which contains client information, configuration, and handles.
    /// - `request`: A `CreateDbInstanceReq` structure containing the details of the database instance to be created.
    ///
    /// # Returns
    /// - `Result<rds_instance::CreateDbInstanceResp, error::Error>`: On success, returns a `CreateDbInstanceResp` structure containing the response from the RDS service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_create_db_instance_request(
        &self,
        rds: &rds::Rds,
        request: rds_instance::CreateDbInstanceReq,
    ) -> Result<rds_instance::CreateDbInstanceResp, error::Error> {
        // Define the request operation with the specific operation name, HTTP method, and path.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::RdsOperation(
                    operation_config::operation_name_rds::OperationNameRds::CreateDBInstance,
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
            volcengine_sdk_protobuf::protobuf::rds_instance::CreateDbInstanceResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
