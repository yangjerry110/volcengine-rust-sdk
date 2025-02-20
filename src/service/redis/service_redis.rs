/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 14:39:42
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:03:29
 * @Description: Implementation of the Redis service, providing methods for managing Redis database instances.
 */
use crate::service::redis::api_create_db_instance;
use crate::service::redis::api_decrease_db_instance_node_number;
use crate::service::redis::api_describe_db_instance_detail;
use crate::service::redis::api_describe_db_instances;
use crate::service::redis::api_enable_sharded_cluster;
use crate::service::redis::api_increase_db_instance_node_number;
use crate::service::redis::api_modify_allow_list;
use crate::service::redis::api_modify_db_instance_shard_capacity;
use crate::service::redis::api_modify_db_instance_shard_number;
use crate::service::redis::{Redis, RedisService};
use crate::volcengine::client::client;
use crate::volcengine::client::client_info;
use crate::volcengine::client::config as client_config;
use crate::volcengine::common;
use crate::volcengine::error::error;
use crate::volcengine::request::handles;
use crate::volcengine::session::session;

/// Implementation of RedisService trait for Redis struct.
/// Provides core functionality for interacting with Volcengine Redis service,
/// including instance lifecycle management and security configurations.
impl RedisService for Redis {
    /// Initializes a new Redis service client from a session configuration.
    ///
    /// # Parameters
    /// - `session`: Authenticated session containing credentials and regional configuration
    ///
    /// # Workflow
    /// 1. Creates service-specific client configuration
    /// 2. Constructs client information (service metadata)
    /// 3. Initializes network handles
    /// 4. Builds complete client instance
    ///
    /// # Returns
    /// - `Ok(Redis)`: Configured Redis client ready for API operations
    /// - `Err(error::Error)`: Configuration errors during client setup
    fn new_redis(session: session::Session) -> Result<Self, error::Error> {
        // Initialize Redis-specific client configuration from session
        let client_config = session.new_client_config(client_config::ClientServiceName::Redis);

        // Build service metadata for API requests
        let client_info = client_info::ClientInfo::builder()
            .with_service_name(client_config::ClientServiceName::Redis)
            .with_api_version(common::COMMON_VERSION_2020_12_07)
            .with_signing_region(&client_config.signing_region)
            .build()?;

        // Initialize network connection pool and TLS configuration
        let request_handles = handles::Handles {};

        // Construct complete client instance
        let client = client::Client::builder()
            .with_client_info(&client_info) // Service metadata
            .with_config(&client_config) // Connection parameters
            .with_handles(&request_handles) // Network resources
            .build()?; // Validate configuration

        Ok(Redis { client: client })
    }

    /// Creates a new Redis database instance with specified configuration.
    ///
    /// # Parameters
    /// - `request`: CreateDbInstanceReq containing:
    ///   - Instance specifications (memory, version, etc.)
    ///   - Network configuration (VPC, subnet)
    ///   - Authentication parameters
    ///
    /// # Returns
    /// - `Ok(CreateDbInstanceResp)`: Contains created instance ID and status
    /// - `Err(error::Error)`: Creation failures (quota exceeded, invalid params)
    async fn new_create_db_instance(
        &self,
        request: volcengine_sdk_protobuf::protobuf::redis_instance::RedisCreateDbInstanceReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::redis_instance::RedisCreateDbInstanceResp,
        error::Error,
    > {
        // Delegate to API implementation module
        api_create_db_instance::ApiCreateDBInstanceRedis
            .new_create_db_instance(self, request)
            .await
    }

    /// Retrieves detailed configuration and status of a Redis instance.
    ///
    /// # Parameters
    /// - `request`: DescribeDbInstanceDetailReq with target instance ID
    ///
    /// # Returns
    /// - `Ok(DescribeDbInstanceDetailResp)`: Detailed instance metadata including:
    ///   - Current state (running/creating/error)
    ///   - Connection endpoints
    ///   - Resource utilization metrics
    ///   - Configuration parameters
    async fn new_describe_db_instance_detail(
        &self,
        request: volcengine_sdk_protobuf::protobuf::redis_instance::RedisDescribeDbInstanceDetailReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::redis_instance::RedisDescribeDbInstanceDetailResp,
        error::Error,
    > {
        api_describe_db_instance_detail::ApiDescribeDBInstanceDetailRedis
            .new_describe_db_instance(self, request)
            .await
    }

    /// Updates network access rules (allow list) for a Redis instance.
    ///
    /// # Security Note
    /// - Modifies IP whitelist rules controlling instance access
    /// - Supports CIDR ranges and individual IP addresses
    ///
    /// # Parameters
    /// - `request`: ModifyAllowListReq containing:
    ///   - Allow list ID to modify
    ///   - New IP/CIDR rules
    ///   - Optional rule description
    ///
    /// # Returns
    /// - `Ok(ModifyAllowListResp)`: Updated allow list version and status
    async fn new_modify_allow_list(
        &self,
        request: volcengine_sdk_protobuf::protobuf::redis_allow::RedisModifyAllowListReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::redis_allow::RedisModifyAllowListResp,
        error::Error,
    > {
        api_modify_allow_list::ApiModifyAllowListRedis
            .new_modify_allow_list(self, request)
            .await
    }

    /// Increases the node number of a Redis database instance.
    ///
    /// This function sends a request to the Volcengine Redis service to increase the number of nodes
    /// in a Redis database instance. It delegates the actual request handling to the
    /// `ApiIncreaseDbInstanceNodeNumber` struct.
    ///
    /// # Arguments
    /// * `request` - A `IncreaseDbInstanceNodeNumberReq` structure containing the request parameters.
    ///
    /// # Returns
    /// * `Result<IncreaseDbInstanceNodeNumberResp, error::Error>` - On success, returns a `IncreaseDbInstanceNodeNumberResp` structure containing the response from the Redis service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_increase_db_instance_node_number(
        &self,
        request: volcengine_sdk_protobuf::protobuf::redis_instance::RedisIncreaseDbInstanceNodeNumberReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::redis_instance::RedisIncreaseDbInstanceNodeNumberResp,
        error::Error,
    > {
        api_increase_db_instance_node_number::ApiIncreaseDbInstanceNodeNumber
            .new_increase_db_instance_node_number(self, request)
            .await
    }

    /// Decreases the node number of a Redis database instance.
    ///
    /// This function sends a request to the Volcengine Redis service to decrease the number of nodes
    /// in a Redis database instance. It delegates the actual request handling to the
    /// `ApiDecreaseDBInstanceNodeNumber` struct.
    ///
    /// # Arguments
    /// * `request` - A `DecreaseDbInstanceNodeNumberReq` structure containing the request parameters.
    ///
    /// # Returns
    /// * `Result<DecreaseDbInstanceNodeNumberResp, error::Error>` - On success, returns a `DecreaseDbInstanceNodeNumberResp` structure containing the response from the Redis service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_decrease_db_instance_node_number(
        &self,
        request: volcengine_sdk_protobuf::protobuf::redis_instance::RedisDecreaseDbInstanceNodeNumberReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::redis_instance::RedisDecreaseDbInstanceNodeNumberResp,
        error::Error,
    > {
        api_decrease_db_instance_node_number::ApiDecreaseDBInstanceNodeNumber
            .new_decrease_db_instance_node_number(self, request)
            .await
    }

    /// Modifies the shard capacity of a Redis database instance.
    ///
    /// This function sends a request to the Volcengine Redis service to modify the shard capacity
    /// of a Redis database instance. It delegates the actual request handling to the
    /// `ApiModifyDBInstanceShardCapacity` struct.
    ///
    /// # Arguments
    /// * `request` - A `ModifyDbInstanceShardCapacityReq` structure containing the request parameters.
    ///
    /// # Returns
    /// * `Result<ModifyDbInstanceShardCapacityResp, error::Error>` - On success, returns a `ModifyDbInstanceShardCapacityResp` structure containing the response from the Redis service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_modify_db_instance_shard_capacity(
        &self,
        request: volcengine_sdk_protobuf::protobuf::redis_instance::RedisModifyDbInstanceShardCapacityReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::redis_instance::RedisModifyDbInstanceShardCapacityResp,
        error::Error,
    > {
        api_modify_db_instance_shard_capacity::ApiModifyDBInstanceShardCapacity
            .new_modify_db_instance_shard_capacity(self, request)
            .await
    }

    /// Modifies the shard number of a Redis database instance.
    ///
    /// This function sends a request to the Volcengine Redis service to modify the shard number
    /// of a Redis database instance. It delegates the actual request handling to the
    /// `ApiModifyDBInstanceShardNumber` struct.
    ///
    /// # Arguments
    /// * `request` - A `ModifyDbInstanceShardNumberReq` structure containing the request parameters.
    ///
    /// # Returns
    /// * `Result<ModifyDbInstanceShardNumberResp, error::Error>` - On success, returns a `ModifyDbInstanceShardNumberResp` structure containing the response from the Redis service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_modify_db_instance_shard_number(
        &self,
        request: volcengine_sdk_protobuf::protobuf::redis_instance::RedisModifyDbInstanceShardNumberReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::redis_instance::RedisModifyDbInstanceShardNumberResp,
        error::Error,
    > {
        api_modify_db_instance_shard_number::ApiModifyDBInstanceShardNumber
            .new_modify_db_instance_shard_number(self, request)
            .await
    }

    /// Enables sharded cluster mode for a Redis database instance.
    ///
    /// This function sends a request to the Volcengine Redis service to enable sharded cluster mode
    /// for a Redis database instance. It delegates the actual request handling to the
    /// `ApiEnableShardedCluster` struct.
    ///
    /// # Arguments
    /// * `request` - A `EnableShardedClusterReq` structure containing the request parameters.
    ///
    /// # Returns
    /// * `Result<EnableShardedClusterResp, error::Error>` - On success, returns a `EnableShardedClusterResp` structure containing the response from the Redis service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_enable_sharded_cluster(
        &self,
        request: volcengine_sdk_protobuf::protobuf::redis_instance::RedisEnableShardedClusterReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::redis_instance::RedisEnableShardedClusterResp,
        error::Error,
    > {
        api_enable_sharded_cluster::ApiEnableShardedCluster
            .new_enable_sharded_cluster(self, request)
            .await
    }

    /// Public method to describe the details of a Redis database instance.
    ///
    /// This method acts as a wrapper to call the `new_describe_db_instances` method of the `ApiDescribeDBInstancesRedis` struct.
    /// It is used to send a request to the Volcengine Redis service to retrieve detailed information about Redis database instances.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of the struct.
    /// - `request`: A `RedisDescribeDbInstancesReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<RedisDescribeDbInstancesResp, error::Error>`: On success, returns a `RedisDescribeDbInstancesResp` structure containing the response from the Redis service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_describe_db_instances(
        &self,
        request: volcengine_sdk_protobuf::protobuf::redis_instance::RedisDescribeDbInstancesReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::redis_instance::RedisDescribeDbInstancesResp,
        error::Error,
    > {
        api_describe_db_instances::ApiDescribeDBInstancesRedis
            .new_describe_db_instances(self, request)
            .await
    }
}
