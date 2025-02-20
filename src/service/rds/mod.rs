/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:36:11
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 11:36:03
 * @Description: Module for managing RDS (Relational Database Service) instances, databases, endpoints, and accounts using Volcengine SDK.
 */
use crate::volcengine::client::client;
use crate::volcengine::error::error;
use crate::volcengine::session::session;
use std::future::Future;
use volcengine_sdk_protobuf::protobuf::rds_account;
use volcengine_sdk_protobuf::protobuf::rds_allow;
use volcengine_sdk_protobuf::protobuf::rds_database;
use volcengine_sdk_protobuf::protobuf::rds_endpoint;
use volcengine_sdk_protobuf::protobuf::rds_instance;

// Import modules for various RDS operations
mod api_create_db_account;
mod api_create_db_account_model;
mod api_create_db_database;
mod api_create_db_database_model;
mod api_create_db_endpoint;
mod api_create_db_endpoint_model;
mod api_create_db_instance;
mod api_create_db_instance_model;
mod api_describe_db_accounts;
mod api_describe_db_accounts_model;
mod api_describe_db_database;
mod api_describe_db_databases_model;
mod api_describe_db_instance_detail;
mod api_describe_db_instance_detail_model;
mod api_describe_db_instances;
mod api_describe_db_instances_models;
mod api_modify_allow_list;
mod api_modify_allow_list_model;
mod api_modify_db_endpoint;
mod api_modify_db_endpoint_model;
mod api_modify_db_instance_spec;
mod api_modify_db_instance_spec_model;
pub mod service_rds;
mod tests;

/**
 * Defines the RDS service interface, providing methods for various RDS operations.
 * This trait encapsulates the functionality required to interact with the Volcengine RDS service.
 * @author: Jerry.Yang
 * @date: 2024-11-05 14:27:29
 */
pub trait RdsService {
    /// Creates a new RDS service instance from a given session.
    /// This method initializes the RDS service with the provided session information.
    fn new_rds(session: session::Session) -> Result<Rds, error::Error>;

    /// Creates a new database instance.
    /// This method sends a request to the Volcengine RDS service to create a new database instance.
    fn new_create_db_instance(
        &self,
        request: rds_instance::CreateDbInstanceReq,
    ) -> impl Future<Output = Result<rds_instance::CreateDbInstanceResp, error::Error>>;

    /// Describes the details of a specific database instance.
    /// This method retrieves detailed information about a specific database instance from the Volcengine RDS service.
    fn new_describe_db_instance_detail(
        &self,
        request: rds_instance::DescribeDbInstanceDetailReq,
    ) -> impl Future<Output = Result<rds_instance::DescribeDbInstanceDetailResp, error::Error>>;

    /// Creates a new database endpoint.
    /// This method sends a request to the Volcengine RDS service to create a new database endpoint.
    fn new_create_db_endpoint(
        &self,
        request: rds_endpoint::CreateDbEndpointReq,
    ) -> impl Future<Output = Result<rds_endpoint::CreateDbEndpointResp, error::Error>>;

    /// Creates a new database account.
    /// This method sends a request to the Volcengine RDS service to create a new database account.
    fn new_create_db_account(
        &self,
        request: rds_account::CreateDbAccountReq,
    ) -> impl Future<Output = Result<rds_account::CreateDbAccountResp, error::Error>>;

    /// Creates a new database.
    /// This method sends a request to the Volcengine RDS service to create a new database.
    fn new_create_db_database(
        &self,
        request: rds_database::CreateDatabaseReq,
    ) -> impl Future<Output = Result<rds_database::CreateDatabaseResp, error::Error>>;

    /// Modifies the allow list of a database instance.
    /// This method sends a request to the Volcengine RDS service to modify the allow list of a database instance.
    fn new_modify_allow_list(
        &self,
        request: rds_allow::ModifyAllowListReq,
    ) -> impl Future<Output = Result<rds_allow::ModifyAllowListResp, error::Error>>;

    /// Modifies the specifications of a database instance.
    /// This method sends a request to the Volcengine RDS service to modify the specifications (e.g., CPU, memory) of a database instance.
    fn new_modify_db_instance_spec(
        &self,
        request: rds_instance::ModifyDbInstanceSpecReq,
    ) -> impl Future<Output = Result<rds_instance::ModifyDbInstanceSpecResp, error::Error>>;

    /// Describes the databases of a database instance.
    /// This method retrieves information about the databases of a specific database instance from the Volcengine RDS service.
    fn new_describe_db_databases(
        &self,
        request: rds_database::DescribeDatabasesReq,
    ) -> impl Future<Output = Result<rds_database::DescribeDatabasesResp, error::Error>>;

    /// Describes the accounts of a database instance.
    /// This method retrieves information about the accounts of a specific database instance from the Volcengine RDS service.
    fn new_describe_db_accounts(
        &self,
        request: rds_account::DescribeDbAccountsReq,
    ) -> impl Future<Output = Result<rds_account::DescribeDbAccountsResp, error::Error>>;

    /// Modifies a database endpoint.
    /// This method sends a request to the Volcengine RDS service to modify a database endpoint.
    fn new_modify_db_endpoint(
        &self,
        request: rds_endpoint::ModifyDbEndpointReq,
    ) -> impl Future<Output = Result<rds_endpoint::ModifyDbEndpointResp, error::Error>>;

    /// Describes the database instances.
    /// This method retrieves information about the database instances from the Volcengine RDS service.
    fn new_describe_db_instances(
        &self,
        request: rds_instance::DescribeDbInstancesReq,
    ) -> impl Future<Output = Result<rds_instance::DescribeDbInstancesResp, error::Error>>;
}

/**
 * Represents the RDS service, encapsulating the client information required to interact with the Volcengine RDS service.
 * @author: Jerry.Yang
 * @date: 2024-11-05 14:27:20
 */
#[derive(Debug, Clone)]
pub struct Rds {
    /// The client used to make requests to the Volcengine RDS service.
    client: client::Client,
}
