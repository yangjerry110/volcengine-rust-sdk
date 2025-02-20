/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:47:46
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:24:42
 * @Description: API endpoint to create a database endpoint.
 */
use crate::service::rds;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::rds_endpoint;

/// `ApiCreateDbEndpointRds` is a struct designed to encapsulate the functionality
/// for creating a database endpoint in RDS (Relational Database Service).
/// This struct provides methods to interact with the Volcengine RDS service,
/// specifically for creating database endpoints.
pub struct ApiCreateDbEndpointRds;

/// Implementation of methods for the `ApiCreateDbEndpointRds` struct.
/// This `impl` block contains the logic for creating a database endpoint,
/// including defining the request operation, building the request,
/// sending it to the service, and parsing the response.
impl ApiCreateDbEndpointRds {
    /// Public method to create a new database endpoint.
    /// This method is intended to be called by external clients
    /// to initiate the creation of a database endpoint.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance.
    /// - `rds`: Reference to an RDS instance (contains client configuration and handles).
    /// - `request`: A Protobuf request structure for creating a database endpoint.
    ///
    /// # Returns
    /// Returns a Protobuf response structure if successful, or an error.
    pub async fn new_create_db_endpoint(
        &self,
        rds: &rds::Rds,
        request: rds_endpoint::CreateDbEndpointReq,
    ) -> Result<rds_endpoint::CreateDbEndpointResp, error::Error> {
        // Delegate the request handling to the private method
        self.new_create_db_endpoint_request(rds, request).await
    }

    /// Private method to handle the actual request to create a database endpoint.
    /// This method is responsible for defining the request operation,
    /// building the request, sending it to the service, and parsing the response.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance.
    /// - `rds`: Reference to an RDS instance (contains client configuration and handles).
    /// - `request`: A Protobuf request structure for creating a database endpoint.
    ///
    /// # Returns
    /// Returns a Protobuf response structure if successful, or an error.
    async fn new_create_db_endpoint_request(
        &self,
        rds: &rds::Rds,
        request: rds_endpoint::CreateDbEndpointReq,
    ) -> Result<rds_endpoint::CreateDbEndpointResp, error::Error> {
        // Define the request operation with specific HTTP method, path, and operation name
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::RdsOperation(
                    operation_config::operation_name_rds::OperationNameRds::CreateDBEndpoint,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::POST,
            )
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()
            .expect("Failed to build request operation"); // Ensure the operation is valid

        // Build the Volcengine request using client information, configuration, handles, and the defined operation
        let response = request::Request::builder()
            .with_client_info(&rds.client.client_info) // Set client information
            .with_config(&rds.client.config) // Set configuration
            .with_handles(&rds.client.handles) // Set handles
            .with_operation(&request_operation) // Set the operation
            .build()?
            .send(request)
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp =
            volcengine_sdk_protobuf::protobuf::rds_endpoint::CreateDbEndpointResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
