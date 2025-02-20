/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:51:35
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 16:44:36
 * @Description: operation name rds
 */

/// Enum representing various operation names for the Relational Database Service (RDS).
/// This enum provides a type - safe way to define and reference different operations
/// that can be performed on RDS resources. The `Debug` derive allows for easy debugging
/// by providing a default implementation of the `fmt::Debug` trait, enabling enum variants
/// to be printed in a human - readable format. The `Clone` derive allows creating copies
/// of the enum values.
#[derive(Debug, Clone)]
pub enum OperationNameRds {
    /// Creates a new database instance in the RDS service.
    /// A database instance is a running database environment with specific configurations
    /// such as storage size, computing resources, etc.
    CreateDBInstance,
    /// Retrieves detailed information about a specific database instance in the RDS service.
    /// This includes information like instance status, configuration settings, and associated metadata.
    DescribeDBInstanceDetail,
    /// Creates a new database endpoint in the RDS service.
    /// A database endpoint is used to connect to a database instance, and it can be used
    /// for various types of connections (e.g., read - only, read - write).
    CreateDBEndpoint,
    /// Creates a new database account in the RDS service.
    /// A database account is used to authenticate and access a database instance,
    /// and it can have different levels of permissions.
    CreateDBAccount,
    /// Creates a new database within a database instance in the RDS service.
    /// A database is a container for tables, views, and other database objects.
    CreateDatabase,
    /// Modifies the allow list for a database instance in the RDS service.
    /// The allow list is used to control which IP addresses or IP ranges can access the database instance.
    ModifyAllowList,
    /// Modifies the specification (e.g., storage size, computing resources) of a database instance
    /// in the RDS service. This can be used to scale up or down the resources allocated to the instance.
    ModifyDBInstanceSpec,
    /// Retrieves information about all the databases within a database instance in the RDS service.
    /// This can include details like database names, character sets, and creation times.
    DescribeDatabases,
    /// Retrieves information about all the database accounts associated with a database instance
    /// in the RDS service. This includes details like account names, permissions, and creation times.
    DescribeDBAccounts,
    /// Modifies an existing database endpoint in the RDS service.
    /// This can involve changing the endpoint's configuration, such as its connection type or security settings.
    ModifyDBEndpoint,
    /// Retrieves information about all the database instances in the RDS service.
    /// This can include details like instance names, statuses, and associated configurations.
    DescribeDBInstances,
}

/// Implementation of the `ToString` trait for the `OperationNameRds` enum.
/// This implementation enables converting an `OperationNameRds` instance into a string representation.
/// It is useful when passing the operation name as a parameter in API calls or for logging purposes.
impl ToString for OperationNameRds {
    /// Converts an `OperationNameRds` instance into a string.
    ///
    /// # Returns
    /// - A `String` corresponding to the operation name. Each enum variant maps to a specific string
    ///   that represents the actual operation name used in the RDS API.
    fn to_string(&self) -> String {
        match self {
            OperationNameRds::CreateDBInstance => "CreateDBInstance",
            OperationNameRds::DescribeDBInstanceDetail => "DescribeDBInstanceDetail",
            OperationNameRds::CreateDBEndpoint => "CreateDBEndpoint",
            OperationNameRds::CreateDBAccount => "CreateDBAccount",
            OperationNameRds::CreateDatabase => "CreateDatabase",
            OperationNameRds::ModifyAllowList => "ModifyAllowList",
            OperationNameRds::ModifyDBInstanceSpec => "ModifyDBInstanceSpec",
            OperationNameRds::DescribeDatabases => "DescribeDatabases",
            OperationNameRds::DescribeDBAccounts => "DescribeDBAccounts",
            OperationNameRds::ModifyDBEndpoint => "ModifyDBEndpoint",
            OperationNameRds::DescribeDBInstances => "DescribeDBInstances",
        }
        // Convert the string literal to a `String` type
        .to_string()
    }
}
