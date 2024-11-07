/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 16:05:11
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-28 16:30:07
 * @Description: Session management for Volcengine clients.
 */

use crate::volcengine::client::config as client_config; // Importing client configuration
use crate::volcengine::config; // Importing the main configuration
use crate::volcengine::endpoint::endpoint; // Importing endpoint handling
use crate::volcengine::error::error; // Importing custom error types
use crate::volcengine::request::handles; // Importing request handling
use crate::volcengine::util::url; // Importing URL utilities

/// Represents a session for interacting with Volcengine services.
#[derive(Debug, Clone)]
pub struct Session {
    config: config::Config,    // Configuration for the session
    handles: handles::Handles, // Handles for managing requests
}

impl Session {
    /// Creates a new `SessionBuilder` for constructing `Session` instances.
    pub fn builder() -> SessionBuilder {
        SessionBuilder {
            config: None,
            handles: None,
        } // Initialize builder with None values
    }

    /// Creates a new `client_config::Config` for the specified service name.
    pub fn new_client_config(
        &self,
        client_service_name: client_config::ClientServiceName, // The name of the client service
    ) -> client_config::Config {
        // Initialize client_config::Config explicitly without using Default
        let mut client_config = client_config::Config {
            config: self.config.clone(),   // Clone the session's config
            endpoint: String::new(),       // Initialize endpoint as an empty string
            signing_region: String::new(), // Initialize signing region as empty
            signing_name: String::new(),   // Initialize signing name as empty
            signing_name_derived: false,   // Initialize derived signing name flag
            handles: self.handles.clone(), // Clone the session's handles
        };

        // Set endpoint if not provided
        if client_config.config.endpoint.is_empty() {
            // Generate endpoint URL based on service name and region
            let endpoint_url = url::Url {
                service_name: client_service_name,  // Service name
                region: self.config.region.clone(), // Region from session config
            }
            .get_endpoint();
            client_config.config.endpoint = endpoint_url.to_string(); // Assign generated endpoint
        }

        // Resolve the endpoint and signing information
        let resolved_endpoint = self.resolve_endpoint(&client_config.config.endpoint);
        client_config.endpoint = resolved_endpoint.url; // Set the resolved URL
        client_config.signing_region = resolved_endpoint.signing_region; // Set signing region
        client_config.signing_name = resolved_endpoint.signing_name; // Set signing name
        client_config.signing_name_derived = resolved_endpoint.signing_name_derived; // Set derived signing name flag

        client_config // Return the configured client
    }

    /// Resolves the endpoint URL and signing details for the given endpoint.
    fn resolve_endpoint(&self, endpoint: &str) -> endpoint::ResolvedEndpoint {
        // Initialize a new ResolvedEndpoint with empty values
        let mut resolved_endpoint = endpoint::ResolvedEndpoint {
            url: String::new(),            // URL placeholder
            signing_region: String::new(), // Signing region placeholder
            signing_name: String::new(),   // Signing name placeholder
            signing_name_derived: false,   // Derived signing name flag placeholder
        };

        // If the endpoint is not empty, resolve it
        if !endpoint.is_empty() {
            let endpoint_url = endpoint::add_scheme(endpoint, self.config.disable_ssl); // Add scheme to URL
            resolved_endpoint.url = endpoint_url; // Set resolved URL
            resolved_endpoint.signing_region = self.config.region.clone(); // Set signing region from session config
        }

        resolved_endpoint // Return the resolved endpoint
    }
}

/// Builder struct for creating a Session instance step-by-step.
pub struct SessionBuilder {
    config: Option<config::Config>, // Optional configuration for the session
    handles: Option<handles::Handles>, // Optional handles for requests
}

impl SessionBuilder {
    /// Sets the configuration for the session builder.
    pub fn with_config(mut self, config: config::Config) -> Self {
        self.config = Some(config); // Store the provided config
        self
    }

    /// Sets the handles for the session builder.
    pub fn with_handles(mut self, handles: handles::Handles) -> Self {
        self.handles = Some(handles); // Store the provided handles
        self
    }

    /// Builds the final Session object.
    /// Returns an error if required fields are not provided.
    pub fn build(self) -> Result<Session, error::Error> {
        // Check if config is provided; return an error if not
        if self.config.is_none() {
            return Err(error::Error::ErrUtilSessionBuildSessionNoConfig); // Error for missing config
        }

        // Create and return a Session instance, using default handles if not provided
        Ok(Session {
            config: self.config.unwrap(), // Unwrap config (guaranteed to be Some)
            handles: handles::Handles {}, // Use default handles if None
        })
    }
}
