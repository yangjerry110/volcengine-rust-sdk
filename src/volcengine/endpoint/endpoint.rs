/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-18 11:31:37
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 14:12:17
 * @Description: Endpoint and utilities for URL scheme handling
 */
/// Enum representing different service endpoints in the Volcengine API.  
///  
/// This enum defines various service-specific endpoints that are used to  
/// determine the correct API URL for making requests. Each variant corresponds  
/// to a specific Volcengine service and provides a logical mapping to  
/// the appropriate endpoint.  
///  
/// This enum is typically used in conjunction with the `Url` struct  
/// to generate fully qualified endpoint URLs dynamically based on  
/// the selected service.
#[derive(Debug, Clone)]
pub enum Endpoint {
    IamEndpoint,   // IAM (Identity and Access Management) endpoint
    EcsEndpoint,   // ECS (Elastic Compute Service) endpoint
    VpcEndpoint,   // VPC (Virtual Private Cloud) endpoint
    RdsEndpoint,   // RDS (Relational Database Service) endpoint
    RedisEndpoint, // Redis service endpoint
    ClbEndpoint,   // Clb service endpoint
}

/// Represents the resolved endpoint details for a specific service in Volcengine.
///
/// This struct encapsulates information about the service's endpoint,  
/// including its URL, signing details, and whether certain attributes were  
/// derived dynamically. It is primarily used in API request signing and  
/// endpoint resolution to ensure requests are correctly authenticated  
/// and routed to the appropriate service.
///
/// ## Fields
/// - `url`  
///   The fully qualified endpoint URL for the service.  
///   This URL is used to send API requests.
///
/// - `signing_region`  
///   The region identifier used for signing API requests.  
///   Some services require regional-specific signing to authenticate requests.
///
/// - `signing_name`  
///   The name of the service as required for request signing.  
///   This ensures that API requests are authenticated correctly.
///
/// - `signing_name_derived`  
///   A boolean flag indicating whether the signing name was automatically  
///   derived from metadata, rather than being explicitly defined in the  
///   service's API specification. This is useful for handling cases  
///   where a service does not explicitly define its signing name.
///
/// - *Planned but commented out:* `signing_method`  
///   Initially intended to specify the cryptographic method used for  
///   signing requests (e.g., HMAC-SHA256). However, it is currently  
///   not included in the struct.
///
/// ## Usage Example
/// ```rust
/// let endpoint = ResolvedEndpoint {
///     url: "https://ecs.volcengineapi.com".to_string(),
///     signing_region: "cn-beijing".to_string(),
///     signing_name: "ecs".to_string(),
///     signing_name_derived: false,
/// };
/// println!("Resolved endpoint: {}", endpoint.url);
/// ```
#[derive(Default)]
/// Struct for holding the resolved endpoint details for a service.
pub struct ResolvedEndpoint {
    /// The endpoint URL for the service.
    pub url: String,

    /// The region used for signing requests to the service.
    pub signing_region: String,

    /// The service name used for signing requests.
    pub signing_name: String,

    /// A flag indicating if the signing name was derived from metadata,
    /// but was not explicitly modeled in the service definition.
    pub signing_name_derived: bool,
    // The signing method was planned to be here (currently commented out).
    // pub signing_method: String,
}

/// Implementation of the `Endpoint` enum, providing methods to interact with service endpoints.
///
/// This implementation defines functionality for retrieving the base URL associated  
/// with each Volcengine service endpoint. These endpoints are used to send API requests  
/// to the corresponding services, such as IAM, ECS, VPC, RDS, Redis, and CLB.
///
/// # Usage
/// The `Endpoint` enum represents different services, and calling `get_endpoint()`  
/// on an instance returns the appropriate service URL.
///
/// ## Example
/// ```rust
/// let iam_service_url = Endpoint::IamEndpoint.get_endpoint();
/// println!("IAM Service URL: {}", iam_service_url); // Output: "iam.volcengineapi.com"
/// ```
///
/// # Shared Endpoints
/// Some services, such as `EcsEndpoint`, `VpcEndpoint`, and `ClbEndpoint`,  
/// use the same base URL (`"open.volcengineapi.com"`). This is common for services  
/// that share infrastructure.
///
/// # Methods
/// - `get_endpoint()`: Returns the corresponding base URL for the selected service.
///
impl Endpoint {
    /// Retrieves the endpoint URL associated with a specific Volcengine service.
    ///
    /// This function returns the base URL corresponding to each service.  
    /// The returned endpoint is used to send API requests for the given service.  
    /// Some services share the same endpoint (e.g., `EcsEndpoint`, `VpcEndpoint`, and `ClbEndpoint`  
    /// all use `"open.volcengineapi.com"`).
    ///
    /// # Returns
    /// A string slice (`&str`) containing the endpoint URL for the selected service.
    ///
    /// ## Example
    /// ```rust
    /// let ecs_endpoint = Endpoint::EcsEndpoint.get_endpoint();
    /// println!("ECS Service URL: {}", ecs_endpoint); // Output: "open.volcengineapi.com"
    /// ```
    pub fn get_endpoint(&self) -> &str {
        match self {
            // Identity and Access Management (IAM) service endpoint
            Endpoint::IamEndpoint => "iam.volcengineapi.com",

            // Elastic Compute Service (ECS) endpoint
            Endpoint::EcsEndpoint => "open.volcengineapi.com",

            // Virtual Private Cloud (VPC) endpoint
            // Uses the same endpoint as ECS
            Endpoint::VpcEndpoint => "open.volcengineapi.com",

            // Relational Database Service (RDS) endpoint
            Endpoint::RdsEndpoint => "rds.volcengineapi.com",

            // Redis service endpoint
            Endpoint::RedisEndpoint => "redis.volcengineapi.com",

            // Cloud Load Balancer (CLB) service endpoint
            // Uses the same endpoint as ECS and VPC
            Endpoint::ClbEndpoint => "open.volcengineapi.com",
        }
    }
}

/// Adds the appropriate scheme (HTTP or HTTPS) to the endpoint URL.
///
/// # Parameters
/// - `endpoint`: The endpoint URL to which the scheme should be added.
/// - `disable_ssl`: A boolean flag indicating whether to disable SSL (use HTTP instead of HTTPS).
///
/// # Returns
/// A `String` representing the full endpoint URL with the scheme added.
pub fn add_scheme(endpoint: &str, disable_ssl: bool) -> String {
    // If the endpoint already starts with "http://" or "https://", return it as is.
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        return endpoint.to_string();
    }

    // If SSL is disabled, force HTTPS as the scheme.
    if disable_ssl {
        return format!("https://{}", endpoint);
    }

    // Default case, add HTTP scheme.
    format!("http://{}", endpoint)
}
