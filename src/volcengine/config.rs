/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-16 17:41:31
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 14:08:39
 * @Description: Utility configuration for Volcengine.
 */

use crate::volcengine::credentials::credentials; // Importing the Credentials struct
use crate::volcengine::error::error; // Importing the custom Error type

/// Represents the configuration settings required for connecting to Volcengine services.
///
/// The `Config` struct holds essential parameters such as the service region, endpoint,
/// SSL settings, and authentication credentials. It is designed to be used with a builder
/// pattern (`ConfigBuilder`) to allow flexible and structured configuration.
///
/// # Fields
/// - `region` (`String`): Specifies the geographical region for the service.
/// - `endpoint` (`String`): The base URL of the API service. If manually set, this overrides
///   the default endpoint.
/// - `disable_ssl` (`bool`): Determines whether SSL/TLS is disabled.
///   - `true`: SSL is disabled (not recommended for production).
///   - `false`: SSL is enabled for secure communication.
/// - `credentials` (`credentials::Credentials`): Stores authentication credentials,
///   including access keys and secret keys, required for making API requests.
///
/// # Example
/// ```rust
/// let config = Config {
///     region: "us-west-2".to_string(),
///     endpoint: "https://api.volcengine.com".to_string(),
///     disable_ssl: false,
///     credentials: credentials::Credentials::new("access_key", "secret_key"),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    pub region: String,    // The geographical region where the service is deployed.
    pub endpoint: String,  // The API endpoint for the service.
    pub disable_ssl: bool, // Indicates whether SSL/TLS is disabled.
    pub credentials: credentials::Credentials, // Authentication credentials for accessing the service.
}

/// Implementation of the `Config` struct, providing utility methods  
/// for managing application configuration settings.  
///  
/// This implementation includes a builder method to facilitate  
/// constructing a `Config` instance step-by-step using the `ConfigBuilder`.  
///  
/// ## Key Functionalities  
/// - `builder()`: Returns a new `ConfigBuilder` instance with default values,  
///   allowing users to set configuration properties before building a `Config` object.  
///  
/// The `Config` struct encapsulates essential configuration parameters such as  
/// service region, endpoint URL, SSL settings, and authentication credentials.  
/// This implementation ensures that configuration objects are created in  
/// a structured and validated manner.  
impl Config {
    /// Creates a new `ConfigBuilder` instance to facilitate the construction of a `Config` object.
    ///
    /// This method returns a `ConfigBuilder` with default values, allowing users to configure
    /// the settings step by step before building a complete `Config` instance.
    ///
    /// # Example
    /// ```rust
    /// let builder = Config::builder();
    /// ```
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default() // Returns a new ConfigBuilder with default values.
    }
}

/// A builder for constructing a `Config` instance in a structured manner.
///
/// The `ConfigBuilder` struct allows users to set configuration parameters individually,
/// ensuring required fields are provided before building the final `Config` instance.
///
/// # Fields
/// - `region` (`Option<String>`): Optional region value.
/// - `endpoint` (`Option<String>`): Optional endpoint value.
/// - `disable_ssl` (`Option<bool>`): Optional SSL flag.
/// - `credentials` (`Option<credentials::Credentials>`): Optional authentication credentials.
pub struct ConfigBuilder {
    region: Option<String>,                        // Optional region value.
    endpoint: Option<String>,                      // Optional endpoint value.
    disable_ssl: Option<bool>,                     // Optional SSL flag.
    credentials: Option<credentials::Credentials>, // Optional credentials.
}

/// Provides a default implementation for `ConfigBuilder`,  
/// initializing all fields to `None`.  
///  
/// This implementation ensures that when a `ConfigBuilder` instance is created  
/// without any parameters, it starts with no predefined values. The user can  
/// then set specific fields as needed before calling `build()`.  
///  
/// ## Default Values  
/// - `region`: `None` (must be provided if required for the configuration)  
/// - `endpoint`: `None` (must be provided if required for the configuration)  
/// - `disable_ssl`: `None` (defaults to `false` if not explicitly set)  
/// - `credentials`: `None` (mandatory; `build()` will return an error if not provided)  
///  
/// Using the builder pattern with a default constructor allows flexibility  
/// while enforcing validation rules in the `build()` method.  
impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            region: None,      // Default region is None.
            endpoint: None,    // Default endpoint is None.
            disable_ssl: None, // Default SSL flag is None.
            credentials: None, // Default credentials are None.
        }
    }
}

/// Implementation of the `ConfigBuilder` struct, which provides a fluent interface  
/// for constructing a `Config` instance step-by-step.  
///  
/// The builder pattern allows for greater flexibility when creating configuration objects  
/// by enabling method chaining to set individual parameters. This approach ensures  
/// that required fields are validated before the final `Config` instance is created.  
///  
/// ## Example Usage  
/// ```rust  
/// use crate::volcengine::config::Config;  
///  
/// let config = Config::builder()  
///     .with_region("us-east-1")  
///     .with_endpoint("https://example.com")  
///     .with_disable_ssl(true)  
///     .with_credentials(credentials)  
///     .build()  
///     .expect("Failed to build config");  
/// ```  
///  
/// Each setter method in `ConfigBuilder` accepts an argument, modifies the corresponding  
/// field, and returns `self` to allow method chaining. The `build` method performs final  
/// validation and returns a `Config` instance or an error if mandatory fields are missing.  
impl ConfigBuilder {
    /// Sets the region for the configuration.
    ///
    /// # Arguments
    /// - `region` (`&str`): The service region to be set.
    ///
    /// # Returns
    /// - `Self`: The updated `ConfigBuilder` instance with the specified region.
    pub fn with_region(mut self, region: &str) -> Self {
        self.region = Some(region.to_string()); // Store the region as an `Option<String>`.
        self
    }

    /// Sets the endpoint URL for the configuration.
    ///
    /// # Arguments
    /// - `endpoint` (`&str`): The service endpoint to be set.
    ///
    /// # Returns
    /// - `Self`: The updated `ConfigBuilder` instance with the specified endpoint.
    pub fn with_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = Some(endpoint.to_string()); // Store the endpoint as an `Option<String>`.
        self
    }

    /// Enables or disables SSL in the configuration.
    ///
    /// # Arguments
    /// - `disable_ssl` (`bool`): Set to `true` to disable SSL, or `false` to enable it.
    ///
    /// # Returns
    /// - `Self`: The updated `ConfigBuilder` instance with the specified SSL setting.
    pub fn with_disable_ssl(mut self, disable_ssl: bool) -> Self {
        self.disable_ssl = Some(disable_ssl); // Store the SSL flag as an `Option<bool>`.
        self
    }

    /// Sets the credentials for the configuration.
    ///
    /// # Arguments
    /// - `credentials` (`credentials::Credentials`): Authentication credentials.
    ///
    /// # Returns
    /// - `Self`: The updated `ConfigBuilder` instance with the specified credentials.
    pub fn with_credentials(mut self, credentials: credentials::Credentials) -> Self {
        self.credentials = Some(credentials); // Store the credentials as an `Option<Credentials>`.
        self
    }

    /// Builds the final `Config` object.
    ///
    /// This method ensures that all required fields (such as credentials) are provided before
    /// constructing the `Config` instance. If any required field is missing, an error is returned.
    ///
    /// # Returns
    /// - `Ok(Config)`: The successfully built `Config` instance.
    /// - `Err(error::Error)`: Returns an error if required fields (e.g., credentials) are missing.
    ///
    /// # Example
    /// ```rust
    /// let config = Config::builder()
    ///     .with_region("us-west-2")
    ///     .with_endpoint("https://api.volcengine.com")
    ///     .with_disable_ssl(false)
    ///     .with_credentials(credentials::Credentials::new("access_key", "secret_key"))
    ///     .build();
    ///
    /// match config {
    ///     Ok(cfg) => println!("Configuration built successfully!"),
    ///     Err(err) => println!("Failed to build configuration: {:?}", err),
    /// }
    /// ```
    pub fn build(self) -> Result<Config, error::Error> {
        // Ensure that credentials are provided; return an error if missing.
        if self.credentials.is_none() {
            return Err(error::Error::ErrUtilConfigBuildConfigNoCredentials);
        }

        // Construct and return a `Config` instance, using default values where necessary.
        Ok(Config {
            region: self.region.unwrap_or_default(), // Defaults to an empty string if region is not set.
            endpoint: self.endpoint.unwrap_or_default(), // Defaults to an empty string if endpoint is not set.
            disable_ssl: self.disable_ssl.unwrap_or_default(), // Defaults to `false` if not set.
            credentials: self.credentials.unwrap(), // Credentials are required and safely unwrapped.
        })
    }
}
