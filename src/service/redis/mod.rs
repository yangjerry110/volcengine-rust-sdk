/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 10:40:08
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 10:59:03
 * @Description: Module for managing Redis database instances using the Volcengine SDK.
 */
use crate::volcengine::client::client;
use crate::volcengine::error::error;
use crate::volcengine::session::session;
use std::future::Future;
use volcengine_sdk_protobuf::protobuf::{redis_allow, redis_instance};

// Import modules for various Redis operations
mod api_create_db_instance;
mod api_create_db_instance_model;
mod api_decrease_db_instance_node_number;
mod api_decrease_db_instance_node_number_model;
mod api_describe_db_instance_detail;
mod api_describe_db_instance_detail_model;
mod api_describe_db_instances;
mod api_describe_db_instances_models;
mod api_enable_sharded_cluster;
mod api_enable_sharded_cluster_model;
mod api_increase_db_instance_node_number;
mod api_increase_db_instance_node_number_model;
mod api_modify_allow_list;
mod api_modify_allow_list_model;
mod api_modify_db_instance_shard_capacity;
mod api_modify_db_instance_shard_capacity_model;
mod api_modify_db_instance_shard_number;
mod api_modify_db_instance_shard_number_model;
pub mod service_redis;
mod tests;

/// Defines the RedisService trait, providing methods for various Redis operations.
/// This trait encapsulates the functionality required to interact with the Volcengine Redis service.
pub trait RedisService {
    /// Creates a new Redis service instance from a given session.
    ///
    /// # Arguments
    /// - `session`: The session object containing the necessary configuration and credentials.
    ///
    /// # Returns
    /// - `Result<Redis, error::Error>`: On success, returns a new instance of the Redis struct.
    ///   On failure, returns an error indicating the cause of the failure.
    fn new_redis(session: session::Session) -> Result<Redis, error::Error>;

    /// Creates a new Redis database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current Redis service instance.
    /// - `request`: The request structure containing the parameters for creating a Redis database instance.
    ///
    /// # Returns
    /// - `impl Future<Output = Result<redis_instance::CreateDbInstanceResp, error::Error>>`: On success, returns a future that resolves to the response from the Redis service.
    ///   On failure, returns an error indicating the cause of the failure.
    fn new_create_db_instance(
        &self,
        request: redis_instance::RedisCreateDbInstanceReq,
    ) -> impl Future<Output = Result<redis_instance::RedisCreateDbInstanceResp, error::Error>>;

    /// Describes the details of a specific Redis database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current Redis service instance.
    /// - `request`: The request structure containing the parameters for describing a Redis database instance.
    ///
    /// # Returns
    /// - `impl Future<Output = Result<redis_instance::DescribeDbInstanceDetailResp, error::Error>>`: On success, returns a future that resolves to the response from the Redis service.
    ///   On failure, returns an error indicating the cause of the failure.
    fn new_describe_db_instance_detail(
        &self,
        request: redis_instance::RedisDescribeDbInstanceDetailReq,
    ) -> impl Future<Output = Result<redis_instance::RedisDescribeDbInstanceDetailResp, error::Error>>;

    /// Modifies the allow list of a Redis database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current Redis service instance.
    /// - `request`: The request structure containing the parameters for modifying the allow list.
    ///
    /// # Returns
    /// - `impl Future<Output = Result<redis_allow::ModifyAllowListResp, error::Error>>`: On success, returns a future that resolves to the response from the Redis service.
    ///   On failure, returns an error indicating the cause of the failure.
    fn new_modify_allow_list(
        &self,
        request: redis_allow::RedisModifyAllowListReq,
    ) -> impl Future<Output = Result<redis_allow::RedisModifyAllowListResp, error::Error>>;

    /// Increase the number of nodes in a Redis instance.
    ///
    /// This function adds nodes to a Redis instance to scale up its capacity or performance.
    /// It takes a request object containing necessary parameters such as the instance ID and the number of nodes to add.
    ///
    /// # Parameters
    /// - `request`: `redis_instance::IncreaseDbInstanceNodeNumberReq` - The request object with parameters for adding nodes.
    ///
    /// # Returns
    /// - `Ok`: `redis_instance::IncreaseDbInstanceNodeNumberResp` - The response indicating the result of the operation.
    /// - `Err`: `error::Error` - The error if the operation fails.
    ///   On failure, returns an error indicating the cause of the failure.
    fn new_increase_db_instance_node_number(
        &self,
        request: redis_instance::RedisIncreaseDbInstanceNodeNumberReq,
    ) -> impl Future<Output = Result<redis_instance::RedisIncreaseDbInstanceNodeNumberResp, error::Error>>;

    /// Decreases the node number of a Redis database instance.
    ///
    /// This function sends a request to the Volcengine Redis service to decrease the number of nodes
    /// in a Redis database instance. It returns a future that resolves to a result containing either
    /// the response from the Redis service or an error indicating the cause of the failure.
    ///
    /// # Arguments
    /// * `request` - A `DecreaseDbInstanceNodeNumberReq` structure containing the request parameters.
    ///
    /// # Returns
    /// * `impl Future<Output = Result<redis_instance::DecreaseDbInstanceNodeNumberResp, error::Error>>` -
    ///   A future that resolves to a result containing either the response from the Redis service or an error.
    fn new_decrease_db_instance_node_number(
        &self,
        request: redis_instance::RedisDecreaseDbInstanceNodeNumberReq,
    ) -> impl Future<Output = Result<redis_instance::RedisDecreaseDbInstanceNodeNumberResp, error::Error>>;

    /// Modifies the shard capacity of a Redis database instance.
    ///
    /// This function sends a request to the Volcengine Redis service to modify the shard capacity
    /// of a Redis database instance. It returns a future that resolves to a result containing either
    /// the response from the Redis service or an error indicating the cause of the failure.
    ///
    /// # Arguments
    /// * `request` - A `ModifyDbInstanceShardCapacityReq` structure containing the request parameters.
    ///
    /// # Returns
    /// * `impl Future<Output = Result<redis_instance::ModifyDbInstanceShardCapacityResp, error::Error>>` -
    ///   A future that resolves to a result containing either the response from the Redis service or an error.
    fn new_modify_db_instance_shard_capacity(
        &self,
        request: redis_instance::RedisModifyDbInstanceShardCapacityReq,
    ) -> impl Future<Output = Result<redis_instance::RedisModifyDbInstanceShardCapacityResp, error::Error>>;

    /// Modifies the shard number of a Redis database instance.
    ///
    /// This function sends a request to the Volcengine Redis service to modify the shard number
    /// of a Redis database instance. It returns a future that resolves to a result containing either
    /// the response from the Redis service or an error indicating the cause of the failure.
    ///
    /// # Arguments
    /// * `request` - A `ModifyDbInstanceShardNumberReq` structure containing the request parameters.
    ///
    /// # Returns
    /// * `impl Future<Output = Result<redis_instance::ModifyDbInstanceShardNumberResp, error::Error>>` -
    ///   A future that resolves to a result containing either the response from the Redis service or an error.
    fn new_modify_db_instance_shard_number(
        &self,
        request: redis_instance::RedisModifyDbInstanceShardNumberReq,
    ) -> impl Future<Output = Result<redis_instance::RedisModifyDbInstanceShardNumberResp, error::Error>>;

    /// Enables sharded cluster mode for a Redis database instance.
    ///
    /// This function sends a request to the Volcengine Redis service to enable sharded cluster mode
    /// for a Redis database instance. It returns a future that resolves to a result containing either
    /// the response from the Redis service or an error indicating the cause of the failure.
    ///
    /// # Arguments
    /// * `request` - A `EnableShardedClusterReq` structure containing the request parameters.
    ///
    /// # Returns
    /// * `impl Future<Output = Result<redis_instance::EnableShardedClusterResp, error::Error>>` -
    ///   A future that resolves to a result containing either the response from the Redis service or an error.
    fn new_enable_sharded_cluster(
        &self,
        request: redis_instance::RedisEnableShardedClusterReq,
    ) -> impl Future<Output = Result<redis_instance::RedisEnableShardedClusterResp, error::Error>>;

    /// Public method to initiate a request for describing the details of a Redis database instance.
    ///
    /// This method returns a future that, when awaited, sends a request to the Volcengine Redis service
    /// to retrieve detailed information about Redis database instances.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of the struct.
    /// - `request`: A `RedisDescribeDbInstancesReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `impl Future<Output = Result<redis_instance::RedisDescribeDbInstancesResp, error::Error>>`:
    ///   A future that resolves to either a `RedisDescribeDbInstancesResp` structure containing the response from the Redis service,
    ///   or an `error::Error` indicating the cause of the failure.
    fn new_describe_db_instances(
        &self,
        request: redis_instance::RedisDescribeDbInstancesReq,
    ) -> impl Future<Output = Result<redis_instance::RedisDescribeDbInstancesResp, error::Error>>;
}

/// Represents the Redis service, encapsulating the client information required to interact with the Volcengine Redis service.
#[derive(Debug, Clone)]
pub struct Redis {
    /// The client used to make requests to the Volcengine Redis service.
    client: client::Client,
}
