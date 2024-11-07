/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-16 17:41:31
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-28 15:08:52
 * @Description: Utility configuration for Volcengine.
 */

use crate::volcengine::credentials::credentials; // Importing the Credentials struct
use crate::volcengine::error::error; // Importing the custom Error type

/// Represents the configuration settings for the application.
#[derive(Debug, Clone)]
pub struct Config {
    pub region: String,                        // The region for the service
    pub endpoint: String,                      // The endpoint URL for the service
    pub disable_ssl: bool,                     // Flag to enable or disable SSL
    pub credentials: credentials::Credentials, // Credentials for authentication
}

impl Config {
    /// Creates a new instance of ConfigBuilder for building Config objects.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default() // Returns a new ConfigBuilder with default values
    }
}

/// Builder struct for constructing a Config instance step-by-step.
pub struct ConfigBuilder {
    region: Option<String>,                        // Optional region value
    endpoint: Option<String>,                      // Optional endpoint value
    disable_ssl: Option<bool>,                     // Optional SSL flag
    credentials: Option<credentials::Credentials>, // Optional credentials
}

/// Default implementation for ConfigBuilder.
impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            region: None,      // Default region is None
            endpoint: None,    // Default endpoint is None
            disable_ssl: None, // Default SSL flag is None
            credentials: None, // Default credentials is None
        }
    }
}

impl ConfigBuilder {
    /// Sets the region for the configuration.
    pub fn with_region(mut self, region: &str) -> Self {
        self.region = Some(region.to_string()); // Store the region as an Option
        self
    }

    /// Sets the endpoint for the configuration.
    pub fn with_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = Some(endpoint.to_string()); // Store the endpoint as an Option
        self
    }

    /// Enables or disables SSL in the configuration.
    pub fn with_disable_ssl(mut self, disable_ssl: bool) -> Self {
        self.disable_ssl = Some(disable_ssl); // Store the SSL flag as an Option
        self
    }

    /// Sets the credentials for the configuration.
    pub fn with_credentials(mut self, credentials: credentials::Credentials) -> Self {
        self.credentials = Some(credentials); // Store the credentials as an Option
        self
    }

    /// Builds the final Config object.
    /// Returns an error if required fields are not provided.
    pub fn build(self) -> Result<Config, error::Error> {
        // Check if credentials are provided; return an error if not
        if self.credentials.is_none() {
            return Err(error::Error::ErrUtilConfigBuildConfigNoCredentials);
        }

        // Create and return a Config instance, using default values for optional fields if necessary
        Ok(Config {
            region: self.region.unwrap_or_default(), // Default to empty string if region is not set
            endpoint: self.endpoint.unwrap_or_default(), // Default to empty string if endpoint is not set
            disable_ssl: self.disable_ssl.unwrap_or_default(), // Default to false if SSL flag is not set
            credentials: self.credentials.unwrap(), // Unwrap credentials since they are required
        })
    }
}
