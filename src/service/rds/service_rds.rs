/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:36:29
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 11:38:48
 * @Description: Implementation of the RDS service, providing methods for managing RDS instances, databases, endpoints, and accounts.
 */
use crate::service::rds::api_create_db_account;
use crate::service::rds::api_create_db_database;
use crate::service::rds::api_create_db_endpoint;
use crate::service::rds::api_create_db_instance;
use crate::service::rds::api_describe_db_accounts;
use crate::service::rds::api_describe_db_database;
use crate::service::rds::api_describe_db_instance_detail;
use crate::service::rds::api_describe_db_instances;
use crate::service::rds::api_modify_allow_list;
use crate::service::rds::api_modify_db_endpoint;
use crate::service::rds::api_modify_db_instance_spec;
use crate::service::rds::{Rds, RdsService};
use crate::volcengine::client::client;
use crate::volcengine::client::client_info;
use crate::volcengine::client::config as client_config;
use crate::volcengine::common;
use crate::volcengine::error::error;
use crate::volcengine::request::handles;
use crate::volcengine::session::session;

/// Implementation of the RdsService trait for the Rds struct.
/// This implementation provides the necessary logic to interact with the Volcengine RDS service,
/// including creating instances, databases, endpoints, and accounts, as well as modifying and describing them.
impl RdsService for Rds {
    /// Creates a new RDS service instance from a given session.
    ///
    /// # Arguments
    /// - `session`: The session object containing the necessary configuration and credentials.
    ///
    /// # Returns
    /// - `Result<Self, error::Error>`: On success, returns a new instance of the Rds struct.
    ///   On failure, returns an error indicating the cause of the failure.
    fn new_rds(session: session::Session) -> Result<Self, error::Error> {
        // Create a new client configuration for the RDS service.
        let client_config = session.new_client_config(client_config::ClientServiceName::Rds);

        // Build the client information with the required parameters.
        let client_info = client_info::ClientInfo::builder()
            .with_service_name(client_config::ClientServiceName::Rds)
            .with_api_version(common::COMMON_VERSION_2022_01_01)
            .with_signing_region(&client_config.signing_region)
            .build()?;

        // Initialize the request handles.
        let request_handles = handles::Handles {};

        // Build the client with the provided information.
        let client = client::Client::builder()
            .with_client_info(&client_info)
            .with_config(&client_config)
            .with_handles(&request_handles)
            .build()?;

        // Return the new RDS service instance.
        Ok(Rds { client: client })
    }

    /// Creates a new database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current RDS service instance.
    /// - `request`: The request structure containing the parameters for creating a database instance.
    ///
    /// # Returns
    /// - `Result<volcengine_sdk_protobuf::protobuf::rds_instance::CreateDbInstanceResp, error::Error>`: On success, returns the response from the RDS service.
    ///   On failure, returns an error indicating the cause of the failure.
    async fn new_create_db_instance(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_instance::CreateDbInstanceReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::rds_instance::CreateDbInstanceResp, error::Error>
    {
        api_create_db_instance::ApiCreateDbInstanceRds
            .new_create_db_instance(self, request)
            .await
    }

    /// Describes the details of a specific database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current RDS service instance.
    /// - `request`: The request structure containing the parameters for describing a database instance.
    ///
    /// # Returns
    /// - `Result<volcengine_sdk_protobuf::protobuf::rds_instance::DescribeDbInstanceDetailResp, error::Error>`: On success, returns the response from the RDS service.
    ///   On failure, returns an error indicating the cause of the failure.
    async fn new_describe_db_instance_detail(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_instance::DescribeDbInstanceDetailReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::rds_instance::DescribeDbInstanceDetailResp,
        error::Error,
    > {
        api_describe_db_instance_detail::ApiDescribeDbInstanceDetailRds
            .new_describe_db_instance_detail(self, request)
            .await
    }

    /// Creates a new database endpoint.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current RDS service instance.
    /// - `request`: The request structure containing the parameters for creating a database endpoint.
    ///
    /// # Returns
    /// - `Result<volcengine_sdk_protobuf::protobuf::rds_endpoint::CreateDbEndpointResp, error::Error>`: On success, returns the response from the RDS service.
    ///   On failure, returns an error indicating the cause of the failure.
    async fn new_create_db_endpoint(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_endpoint::CreateDbEndpointReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::rds_endpoint::CreateDbEndpointResp, error::Error>
    {
        api_create_db_endpoint::ApiCreateDbEndpointRds
            .new_create_db_endpoint(self, request)
            .await
    }

    /// Creates a new database account.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current RDS service instance.
    /// - `request`: The request structure containing the parameters for creating a database account.
    ///
    /// # Returns
    /// - `Result<volcengine_sdk_protobuf::protobuf::rds_account::CreateDbAccountResp, error::Error>`: On success, returns the response from the RDS service.
    ///   On failure, returns an error indicating the cause of the failure.
    async fn new_create_db_account(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_account::CreateDbAccountReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::rds_account::CreateDbAccountResp, error::Error>
    {
        api_create_db_account::ApiCreateDbAccountRds
            .new_create_db_account(self, request)
            .await
    }

    /// Creates a new database.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current RDS service instance.
    /// - `request`: The request structure containing the parameters for creating a database.
    ///
    /// # Returns
    /// - `Result<volcengine_sdk_protobuf::protobuf::rds_database::CreateDatabaseResp, error::Error>`: On success, returns the response from the RDS service.
    ///   On failure, returns an error indicating the cause of the failure.
    async fn new_create_db_database(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_database::CreateDatabaseReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::rds_database::CreateDatabaseResp, error::Error>
    {
        api_create_db_database::ApiCreateDbDatabaseRds
            .new_create_db_database(self, request)
            .await
    }

    /// Modifies the allow list of a database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current RDS service instance.
    /// - `request`: The request structure containing the parameters for modifying the allow list.
    ///
    /// # Returns
    /// - `Result<volcengine_sdk_protobuf::protobuf::rds_allow::ModifyAllowListResp, error::Error>`: On success, returns the response from the RDS service.
    ///   On failure, returns an error indicating the cause of the failure.
    async fn new_modify_allow_list(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_allow::ModifyAllowListReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::rds_allow::ModifyAllowListResp, error::Error>
    {
        api_modify_allow_list::ApiModifyAllowListRds
            .new_modify_allow_list(self, request)
            .await
    }

    /// Modifies the specifications of a database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current RDS service instance.
    /// - `request`: The request structure containing the parameters for modifying the instance specifications.
    ///
    /// # Returns
    /// - `Result<volcengine_sdk_protobuf::protobuf::rds_instance::ModifyDbInstanceSpecResp, error::Error>`: On success, returns the response from the RDS service.
    ///   On failure, returns an error indicating the cause of the failure.
    async fn new_modify_db_instance_spec(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_instance::ModifyDbInstanceSpecReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::rds_instance::ModifyDbInstanceSpecResp,
        error::Error,
    > {
        api_modify_db_instance_spec::ApiModifyDBInstanceSpecRds
            .new_modify_db_instance_spec(self, request)
            .await
    }

    /// Describes the databases of a specific database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current RDS service instance.
    /// - `request`: The request structure containing the parameters for describing databases.
    ///
    /// # Returns
    /// - `Result<volcengine_sdk_protobuf::protobuf::rds_database::DescribeDatabasesResp, error::Error>`: On success, returns the response from the RDS service.
    ///   On failure, returns an error indicating the cause of the failure.
    async fn new_describe_db_databases(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_database::DescribeDatabasesReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::rds_database::DescribeDatabasesResp, error::Error>
    {
        api_describe_db_database::ApiDescribeDbDatabasesRds
            .new_describe_db_databases(self, request)
            .await
    }

    /// Describes the accounts of a specific database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current RDS service instance.
    /// - `request`: The request structure containing the parameters for describing accounts.
    ///
    /// # Returns
    /// - `Result<volcengine_sdk_protobuf::protobuf::rds_account::DescribeDbAccountsResp, error::Error>`: On success, returns the response from the RDS service.
    ///   On failure, returns an error indicating the cause of the failure.
    async fn new_describe_db_accounts(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_account::DescribeDbAccountsReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::rds_account::DescribeDbAccountsResp, error::Error>
    {
        api_describe_db_accounts::ApiDescribeDBAccountsRds
            .new_describe_db_accounts(self, request)
            .await
    }

    /// Modifies a database endpoint.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current RDS service instance.
    /// - `request`: The request structure containing the parameters for modifying a database endpoint.
    ///
    /// # Returns
    /// - `Result<volcengine_sdk_protobuf::protobuf::rds_endpoint::ModifyDbEndpointResp, error::Error>`: On success, returns the response from the RDS service.
    ///   On failure, returns an error indicating the cause of the failure.
    async fn new_modify_db_endpoint(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_endpoint::ModifyDbEndpointReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::rds_endpoint::ModifyDbEndpointResp, error::Error>
    {
        api_modify_db_endpoint::ApiModifyDBEndpointRds
            .new_modify_db_endpoint(self, request)
            .await
    }

    /// Describes the database instances.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current RDS service instance.
    /// - `request`: The request structure containing the parameters for describing database instances.
    ///
    /// # Returns
    /// - `Result<volcengine_sdk_protobuf::protobuf::rds_instance::DescribeDbInstancesResp, error::Error>`: On success, returns the response from the RDS service.
    ///   On failure, returns an error indicating the cause of the failure.
    async fn new_describe_db_instances(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_instance::DescribeDbInstancesReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::rds_instance::DescribeDbInstancesResp,
        error::Error,
    > {
        api_describe_db_instances::ApiDescribeDBInstancesRds
            .new_describe_db_instances(self, request)
            .await
    }
}
