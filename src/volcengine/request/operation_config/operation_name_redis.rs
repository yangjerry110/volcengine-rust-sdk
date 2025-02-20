/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 10:51:34
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-17 17:25:32
 * @Description: operation name redis
 */

/// Enum representing various operation names for the Redis database service.
/// This enum provides a type-safe way to define and reference different operations
/// that can be performed on Redis resources. The `Debug` derive allows for easy debugging
/// by providing a default implementation of the `fmt::Debug` trait, enabling enum variants
/// to be printed in a human-readable format. The `Clone` derive allows creating copies
/// of the enum values.
#[derive(Debug, Clone)]
pub enum OperationNameRedis {
    /// Retrieves detailed information about a specific Redis database instance.
    /// This operation is used to fetch attributes such as instance configuration,
    /// status, and other metadata related to the instance.
    DescribeDBInstanceDetail,

    /// Creates a new Redis database instance.
    /// A Redis database instance is an isolated environment where Redis data is stored and managed.
    /// This operation is used to provision a new instance with specific configurations such as
    /// memory size, network settings, etc.
    CreateDBInstance,

    /// Retrieves information about all the Redis database instances in the service.
    /// This operation can be used to get details like instance names, statuses (e.g., running, stopped),
    /// associated configurations, and creation times.
    DescribeDBInstances,

    /// Modifies the allow list for a Redis database instance.
    /// The allow list is used to control which IP addresses or IP ranges can access the Redis instance.
    /// This operation can be used to add or remove IPs from the allow list, enhancing the security
    /// of the Redis service.
    ModifyAllowList,

    /// Increases the number of nodes in a Redis database instance.
    /// This operation is used to scale out the instance by adding more nodes to handle increased load.
    IncreaseDBInstanceNodeNumber,

    /// Decreases the number of nodes in a Redis database instance.
    /// This operation is used to scale in the instance by removing nodes when the load decreases.
    DecreaseDBInstanceNodeNumber,

    /// Modifies the shard capacity of a Redis database instance.
    /// This operation adjusts the memory or storage capacity allocated to each shard in the instance.
    ModifyDBInstanceShardCapacity,

    /// Modifies the number of shards in a Redis database instance.
    /// This operation is used to change the sharding configuration to optimize performance or storage.
    ModifyDBInstanceShardNumber,

    /// Enables sharded clustering for a Redis database instance.
    /// This operation configures the instance to use sharded clustering, which allows for horizontal scaling.
    EnableShardedCluster,
}

/// Implementation of the `ToString` trait for the `OperationNameRedis` enum.
/// This implementation enables converting an `OperationNameRedis` instance into a string representation.
/// It is useful when passing the operation name as a parameter in API calls or for logging purposes.
impl ToString for OperationNameRedis {
    /// Converts an `OperationNameRedis` instance into a string.
    ///
    /// # Returns
    /// - A `String` corresponding to the operation name. Each enum variant maps to a specific string
    ///   that represents the actual operation name used in the Redis API.
    fn to_string(&self) -> String {
        match self {
            OperationNameRedis::DescribeDBInstanceDetail => "DescribeDBInstanceDetail",
            OperationNameRedis::CreateDBInstance => "CreateDBInstance",
            OperationNameRedis::DescribeDBInstances => "DescribeDBInstances",
            OperationNameRedis::ModifyAllowList => "ModifyAllowList",
            OperationNameRedis::IncreaseDBInstanceNodeNumber => "IncreaseDBInstanceNodeNumber",
            OperationNameRedis::DecreaseDBInstanceNodeNumber => "DecreaseDBInstanceNodeNumber",
            OperationNameRedis::ModifyDBInstanceShardCapacity => "ModifyDBInstanceShardCapacity",
            OperationNameRedis::ModifyDBInstanceShardNumber => "ModifyDBInstanceShardNumber",
            OperationNameRedis::EnableShardedCluster => "EnableShardedCluster",
        }
        // Convert the string literal to a `String` type
        .to_string()
    }
}
