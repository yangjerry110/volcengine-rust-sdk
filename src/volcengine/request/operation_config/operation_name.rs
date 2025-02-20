/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 14:39:42
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 16:48:05
 * @Description: operation name
 */
use crate::volcengine::request::operation_config::operation_name_clb;
use crate::volcengine::request::operation_config::operation_name_ecs;
use crate::volcengine::request::operation_config::operation_name_iam;
use crate::volcengine::request::operation_config::operation_name_rds;
use crate::volcengine::request::operation_config::operation_name_redis;
use crate::volcengine::request::operation_config::operation_name_vpc;

/// Enum representing a unified set of possible operation names across multiple services.
/// This enum encapsulates different operation names from various services such as IAM, ECS, VPC, RDS, Redis, and CLB.
/// It provides a type - safe way to represent and manage different operations in a single structure.
/// The `Debug` derive allows for easy debugging by providing a default implementation of the `fmt::Debug` trait,
/// which enables printing the enum variants in a readable format.
/// The `Clone` derive allows for creating copies of the enum values when needed.
#[derive(Debug, Clone)]
pub enum OperationName {
    /// Represents operations related to the Identity and Access Management (IAM) service.
    /// The inner value is of type `operation_name_iam::OperationNameIam`, which contains specific IAM operations.
    IamOperation(operation_name_iam::OperationNameIam),
    /// Represents operations related to the Elastic Compute Service (ECS).
    /// The inner value is of type `operation_name_ecs::OperationNameEcs`, which contains specific ECS operations.
    EcsOperation(operation_name_ecs::OperationNameEcs),
    /// Represents operations related to the Virtual Private Cloud (VPC) service.
    /// The inner value is of type `operation_name_vpc::OperationNameVpc`, which contains specific VPC operations.
    VpcOperation(operation_name_vpc::OperationNameVpc),
    /// Represents operations related to the Relational Database Service (RDS).
    /// The inner value is of type `operation_name_rds::OperationNameRds`, which contains specific RDS operations.
    RdsOperation(operation_name_rds::OperationNameRds),
    /// Represents operations related to the Redis database service.
    /// The inner value is of type `operation_name_redis::OperationNameRedis`, which contains specific Redis operations.
    RedisOperation(operation_name_redis::OperationNameRedis),
    /// Represents operations related to the Cloud Load Balancer (CLB) service.
    /// The inner value is of type `operation_name_clb::OperationNameClb`, which contains specific CLB operations.
    ClbOperation(operation_name_clb::OperationNameClb),
}

/// Implementation of the `ToString` trait for the `OperationName` enum.
/// This allows converting an instance of `OperationName` into a string representation.
/// It is useful when passing the operation name as a parameter in API calls or for logging purposes.
impl ToString for OperationName {
    /// Converts an `OperationName` instance into a string.
    ///
    /// Based on the variant of the `OperationName` enum, it calls the `to_string` method
    /// of the corresponding inner operation name enum (e.g., `OperationNameEcs`, `OperationNameIam`).
    ///
    /// # Returns
    /// - A `String` representing the operation name.
    fn to_string(&self) -> String {
        match self {
            // Convert the ECS operation name to a string
            OperationName::EcsOperation(operation_name_ecs) => operation_name_ecs.to_string(),
            // Convert the IAM operation name to a string
            OperationName::IamOperation(operation_name_iam) => operation_name_iam.to_string(),
            // Convert the VPC operation name to a string
            OperationName::VpcOperation(operation_name_vpc) => operation_name_vpc.to_string(),
            // Convert the RDS operation name to a string
            OperationName::RdsOperation(operation_name_rds) => operation_name_rds.to_string(),
            // Convert the Redis operation name to a string
            OperationName::RedisOperation(operation_name_redis) => operation_name_redis.to_string(),
            // Convert the CLB operation name to a string
            OperationName::ClbOperation(operation_name_clb) => operation_name_clb.to_string(),
        }
    }
}
