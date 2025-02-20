/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-26 16:32:04
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:27:36
 * @Description: API for describing database instances.
 */
use crate::service::rds;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::rds_instance;

/// A struct representing the API for describing database instances.
/// This struct encapsulates the functionality required to send a request to the Volcengine RDS service
/// to retrieve information about database instances.
pub struct ApiDescribeDBInstancesRds;

/// Implementation of methods for the `ApiDescribeDBInstancesRds` struct.
/// This implementation provides the necessary logic to construct and send a request to the Volcengine RDS service
/// to describe database instances, as well as handle the response.
impl ApiDescribeDBInstancesRds {
    /// Public method to describe database instances.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiDescribeDBInstancesRds`.
    /// - `rds`: Reference to an `Rds` instance, which contains client information, configuration, and handles.
    /// - `request`: A `DescribeDbInstancesReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<rds_instance::DescribeDbInstancesResp, error::Error>`: On success, returns a `DescribeDbInstancesResp` structure containing the response from the RDS service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    pub async fn new_describe_db_instances(
        &self,
        rds: &rds::Rds,
        request: rds_instance::DescribeDbInstancesReq,
    ) -> Result<rds_instance::DescribeDbInstancesResp, error::Error> {
        // Delegate the request handling to the private method `new_describe_db_instances_request`.
        self.new_describe_db_instances_request(rds, request).await
    }

    /// Private method to handle the request to describe database instances.
    ///
    /// This method constructs the request operation, builds the request, sends it to the Volcengine RDS service,
    /// and parses the response.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiDescribeDBInstancesRds`.
    /// - `rds`: Reference to an `Rds` instance, which contains client information, configuration, and handles.
    /// - `request`: A `DescribeDbInstancesReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<rds_instance::DescribeDbInstancesResp, error::Error>`: On success, returns a `DescribeDbInstancesResp` structure containing the response from the RDS service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_describe_db_instances_request(
        &self,
        rds: &rds::Rds,
        request: rds_instance::DescribeDbInstancesReq,
    ) -> Result<rds_instance::DescribeDbInstancesResp, error::Error> {
        // Define the request operation with the specific operation name, HTTP method, and path.
        // The operation name corresponds to the "DescribeDBInstances" action in the Volcengine RDS service.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::RdsOperation(
                    operation_config::operation_name_rds::OperationNameRds::DescribeDBInstances,
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
            volcengine_sdk_protobuf::protobuf::rds_instance::DescribeDbInstancesResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
