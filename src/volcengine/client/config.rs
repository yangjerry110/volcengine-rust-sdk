/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-18 10:33:04
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 14:16:56
 * @Description: Client configuration for Volcengine API
 */
use crate::volcengine::config;
use crate::volcengine::request::handles;

/// Struct representing the client configuration for accessing Volcengine services.
///
/// This struct holds the configuration settings necessary to interact with various
/// Volcengine services, including authentication information, request handling,
/// endpoint URL, and region-specific details for signing requests.
///
/// The `Config` struct is crucial for managing connections to Volcengine APIs and
/// contains all required settings to authenticate and make requests to the desired service.
///
/// # Fields:
/// - `config`: Holds the detailed configuration required for authentication and other settings.
/// - `handles`: Manages the handles or sessions for handling requests to the Volcengine services.
/// - `endpoint`: The URL of the service endpoint for accessing the service.
/// - `signing_region`: The region to be used for signing API requests.
/// - `signing_name`: The name of the service to be used for signing the requests.
/// - `signing_name_derived`: A flag indicating whether the signing name is derived from metadata or explicitly modeled.
#[derive(Debug, Clone)]
pub struct Config {
    /// The configuration details for the client, including authentication and settings.
    pub config: config::Config,

    /// Handles associated with the client, likely for request handling or session management.
    pub handles: handles::Handles,

    /// The endpoint URL for the specific Volcengine service.
    pub endpoint: String,

    /// The region used for signing requests.
    pub signing_region: String,

    /// The service name used for signing requests.
    pub signing_name: String,

    /// A flag indicating whether the signing name was derived from metadata
    /// but was not explicitly modeled in the service configuration.
    pub signing_name_derived: bool,
}

/// Enum representing the available service names for the Volcengine API client.
///
/// This enum is used to define the different services that are accessible through
/// the Volcengine API, including services such as IAM, ECS, VPC, RDS, Redis, and CLB.
/// Each variant represents a distinct service that can be used by the API client
/// to make requests to the corresponding Volcengine API endpoint.
///
/// # Variants:
/// - `Iam`: The Identity and Access Management (IAM) service.
/// - `Ecs`: The Elastic Compute Service (ECS).
/// - `Vpc`: The Virtual Private Cloud (VPC) service.
/// - `Rds`: The Relational Database Service (RDS) for MySQL-based databases.
/// - `Redis`: The Redis service.
/// - `Clb`: The Cloud Load Balancer (CLB) service.
#[derive(Debug, Clone)]
pub enum ClientServiceName {
    Iam,   // Identity and Access Management (IAM) service
    Ecs,   // Elastic Compute Service (ECS) service
    Vpc,   // Virtual Private Cloud (VPC) service
    Rds,   // Relational Database Service (RDS) - specifically MySQL
    Redis, // Redis service
    Clb,   // CLB service
}

/**
 * Implementation block for the `ClientServiceName` enum.
 * This block contains methods that provide functionality
 * for working with the `ClientServiceName` enum.
 * @author: Jerry.Yang
 * @date: 2024-11-08 11:00:02
 * @return: None
 */
impl ClientServiceName {
    /// Converts the `ClientServiceName` enum variant to a string representation.
    ///
    /// This method returns a string slice (`&str`) that corresponds to the service name
    /// associated with the specific `ClientServiceName` variant. This is useful for
    /// easily converting the enum variant into a string that can be used for making
    /// API requests, logging, or other purposes where a string representation of the
    /// service name is required.
    ///
    /// # Returns
    /// Returns a `&str` representing the service name in lowercase.
    ///
    /// # Example
    /// ```rust
    /// let service_name = ClientServiceName::Iam.as_str();
    /// assert_eq!(service_name, "iam");
    /// ```
    pub fn as_str(&self) -> &str {
        match self {
            ClientServiceName::Iam => "iam",       // IAM service
            ClientServiceName::Ecs => "ecs",       // ECS service
            ClientServiceName::Vpc => "vpc",       // VPC service
            ClientServiceName::Rds => "rds_mysql", // MySQL-based RDS service
            ClientServiceName::Redis => "redis",   // Redis service
            ClientServiceName::Clb => "clb",       // CLB service
        }
    }
}
