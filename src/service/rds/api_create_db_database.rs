/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:47:46
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:24:07
 * @Description: api create db account
 */
use crate::service::rds;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::rds_database;

/// Represents an API client for creating a database in the RDS service.
/// This struct provides methods to send requests for creating a new database.
pub struct ApiCreateDbDatabaseRds;

/// Implementation of methods for the `ApiCreateDbDatabaseRds` struct.
/// These methods are used to initiate and execute requests for creating a database in the RDS service.
impl ApiCreateDbDatabaseRds {
    /// Initiates a new API call to create a database.
    ///
    /// This method acts as a high - level wrapper for the actual request - sending process.
    /// It calls the `new_create_db_database_request` method to perform the underlying operations.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current `ApiCreateDbDatabaseRds` instance.
    /// - `rds`: A reference to the `Rds` service instance, which contains client - related information
    ///          such as authentication details, endpoints, and configuration settings.
    /// - `request`: A `CreateDatabaseReq` struct that holds the necessary data for creating a database,
    ///              such as the database name, character set, and other relevant parameters.
    ///
    /// # Returns
    /// - On success, it returns a `CreateDatabaseResp` struct containing the response data from the RDS service,
    ///   which may include details about the newly created database, like its ID or status.
    /// - On error, it returns an `Error` struct indicating the reason for the failure,
    ///   such as network issues, authentication errors, or invalid request parameters.
    pub async fn new_create_db_database(
        &self,
        rds: &rds::Rds,
        request: rds_database::CreateDatabaseReq,
    ) -> Result<rds_database::CreateDatabaseResp, error::Error> {
        // Delegate the actual request - sending to the underlying method
        self.new_create_db_database_request(rds, request).await
    }

    /// Creates and sends a new API request to create a database.
    ///
    /// This method constructs the request operation, builds the Volcengine request,
    /// sends the request to the RDS service, and parses the response.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current `ApiCreateDbDatabaseRds` instance.
    /// - `rds`: A reference to the `Rds` service instance, which contains client - related information
    ///          such as authentication details, endpoints, and configuration settings.
    /// - `request`: A `CreateDatabaseReq` struct that holds the necessary data for creating a database,
    ///              such as the database name, character set, and other relevant parameters.
    ///
    /// # Returns
    /// - On success, it returns a `CreateDatabaseResp` struct containing the response data from the RDS service,
    ///   which may include details about the newly created database, like its ID or status.
    /// - On error, it returns an `Error` struct indicating the reason for the failure,
    ///   such as network issues, authentication errors, or invalid request parameters.
    async fn new_create_db_database_request(
        &self,
        rds: &rds::Rds,
        request: rds_database::CreateDatabaseReq,
    ) -> Result<rds_database::CreateDatabaseResp, error::Error> {
        // Define the request operation
        // Set the operation name, HTTP method, and HTTP path for the API request
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                // Specify the RDS operation as creating a database
                operation_config::operation_name::OperationName::RdsOperation(
                    operation_config::operation_name_rds::OperationNameRds::CreateDatabase,
                ),
            )
            .with_operation_http_method(
                // Use the POST method for creating a new resource
                operation_config::operation_http_method::OperationHttpMethod::POST,
            )
            .with_operation_http_path(
                // Use the default HTTP path for this operation
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?;

        // Build and send the request
        // Construct the Volcengine request using the client information, configuration,
        // handlers, and the defined operation. Then send the `CreateDatabaseReq` request
        // and parse the response as a `CreateDatabaseResp` struct.
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
            volcengine_sdk_protobuf::protobuf::rds_database::CreateDatabaseResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
