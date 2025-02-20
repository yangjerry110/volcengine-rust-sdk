/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-18 10:49:00
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 11:42:12
 * @Description: Utility for constructing service URLs
 */
use crate::volcengine::client::config;
use crate::volcengine::endpoint::endpoint;

/// Represents a URL structure for constructing an endpoint URL based on the service name and region.
///
/// This struct is designed to hold information related to constructing the endpoint URL for a
/// given service in a specified region. It encapsulates the `service_name` and `region`
/// that can be used together to dynamically build the full URL to access the service.
///
/// # Fields:
/// - `service_name`: The name of the service for which the URL is being constructed. It typically
///   corresponds to a service identifier or a name used to distinguish between different services.
/// - `region`: The region where the service is located. This allows the endpoint to be region-specific,
///   which is useful when services are deployed in different geographical regions for performance or
///   compliance reasons.
///
/// This struct is primarily used for generating endpoint URLs that include the appropriate service and
/// region, ensuring that requests are routed to the correct service instance in the correct region.
///
/// # Example:
/// ```rust
/// let url = Url {
///     service_name: config::ClientServiceName::SomeService,
///     region: "us-west-2".to_string(),
/// };
/// ```
#[allow(dead_code)]
pub struct Url {
    /// The service name that the URL is associated with.
    pub service_name: config::ClientServiceName,

    /// The region for which the endpoint is needed.
    pub region: String,
}

/// Provides methods for constructing and retrieving endpoint URLs based on the service name and region.
///
/// The `Url` struct is designed to encapsulate the logic of building URLs for various services within a specific region.
/// The service name (`service_name`) determines which API service the URL will target (e.g., IAM, ECS, VPC, etc.), and
/// the `region` field is intended for future use in selecting regional endpoints (though it is not yet actively used in the
/// URL construction). This implementation currently maps service names to specific endpoints, allowing clients to
/// easily retrieve the full URL for a given service.
///
/// The `get_endpoint` method is used to construct the URL for the desired service based on its name. While the region
/// is kept in the struct for future extensibility, it currently does not modify the endpoint URL.
///
/// # Methods
/// - `get_endpoint`: Returns the endpoint URL for the specified service, based on the `service_name`. The URL is
///   selected from a predefined set of endpoints. The region is not yet incorporated into the URL construction process,
///   but it is stored in the struct for future use.
///
/// # Example
/// ```rust
/// let url = Url {
///     service_name: config::ClientServiceName::Ecs,
///     region: "us-east-1".to_string(),
/// };
/// let endpoint = url.get_endpoint();
/// println!("The ECS endpoint is: {}", endpoint);
/// ```
impl Url {
    /// Gets the endpoint URL for the specified service.
    ///
    /// This function selects the appropriate endpoint based on the `service_name` and returns it as a string.
    /// The region field is currently not used, but it is part of the struct for future extension to handle
    /// region-specific endpoints.
    ///
    /// # Returns
    /// A `String` representing the endpoint URL for the service, selected from a predefined set of endpoints.
    pub fn get_endpoint(&self) -> String {
        // Match the service name to determine the appropriate endpoint.
        let url_config_endpoint = match self.service_name {
            config::ClientServiceName::Iam => endpoint::Endpoint::IamEndpoint,
            config::ClientServiceName::Ecs => endpoint::Endpoint::EcsEndpoint,
            config::ClientServiceName::Vpc => endpoint::Endpoint::VpcEndpoint,
            config::ClientServiceName::Rds => endpoint::Endpoint::RdsEndpoint,
            config::ClientServiceName::Redis => endpoint::Endpoint::RedisEndpoint,
            config::ClientServiceName::Clb => endpoint::Endpoint::ClbEndpoint,
        };

        // Region-related configuration is not yet implemented, so this is left blank for future expansion.

        // Return the selected endpoint URL as a string.
        url_config_endpoint.get_endpoint().to_string()
    }
}
