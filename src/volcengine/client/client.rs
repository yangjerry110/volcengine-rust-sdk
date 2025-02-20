/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 15:07:52
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 14:18:01
 * @Description: Defines the `Client` and its builder `ClientBuilder` to encapsulate client configuration,
 * client information, and request handlers for interacting with the Volcengine API.
 */
use super::client_info;
use crate::volcengine::client::config;
use crate::volcengine::error::error;
use crate::volcengine::request::handles;

// `Client` struct that holds the client information, configuration, and request handler.
//
// This struct is the main client object used to interact with Volcengine services. It contains all the
// necessary data to authenticate, configure, and send requests to the Volcengine API.
//
// # Fields:
// - `client_info`: Holds the client information such as identification details, credentials, etc.
// - `config`: Contains the configuration data that defines the client's behavior and connection settings.
// - `handles`: Manages the request handlers responsible for handling communication with the API.
#[derive(Debug, Clone)]
pub struct Client {
    pub client_info: client_info::ClientInfo, // Client information required for authentication and operations
    pub config: config::Config,               // Configuration data to set up the client
    pub handles: handles::Handles, // Request handler responsible for managing API requests
}

// `Client` implementation providing a `builder` function for constructing `ClientBuilder`
// The `builder` method allows for a more flexible and readable way to construct a `Client` by chaining method calls
impl Client {
    // Creates a new `ClientBuilder` instance, which is used to incrementally set up the `Client` fields
    // before calling `build` to create the final `Client` instance.
    //
    // # Returns:
    // Returns a new `ClientBuilder` object that allows for configuring and building a `Client` object.
    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            client_info: None, // Optional client information, defaults to None
            config: None,      // Optional configuration, defaults to None
            handles: None,     // Optional request handler, defaults to None
        }
    }
}

// `ClientBuilder` struct to incrementally build a `Client` object.
//
// The `ClientBuilder` struct provides a builder pattern to facilitate the creation of a `Client` object.
// Each method allows setting a specific field of the `Client` and returns the builder for method chaining.
//
// # Fields:
// - `client_info`: Optional client information for the API client, may be set using `with_client_info`.
// - `config`: Optional configuration for the client, set via `with_config`.
// - `handles`: Optional request handler, set using `with_handles`.
pub struct ClientBuilder {
    pub client_info: Option<client_info::ClientInfo>, // Optional client information
    pub config: Option<config::Config>,               // Optional configuration
    pub handles: Option<handles::Handles>,            // Optional request handler
}

// `ClientBuilder` implementation to add fields to the `Client` struct and build it with validation.
//
// The `ClientBuilder` provides methods for setting each field of the `Client`. These methods are chainable,
// allowing for a fluent API. The `build` method performs validation to ensure all required fields are set before
// returning the final `Client` instance.
//
// # Methods:
// - `with_client_info`: Sets the `client_info` field in the builder.
// - `with_config`: Sets the `config` field in the builder.
// - `with_handles`: Sets the `handles` field in the builder.
// - `build`: Finalizes the builder, ensuring that all required fields are set, and returns the constructed `Client` instance.
impl ClientBuilder {
    // Sets the `client_info` field for the builder.
    // The `client_info` represents the information about the client, such as authentication details.
    //
    // # Arguments:
    // - `client_info`: A reference to a `ClientInfo` struct that holds the client's information.
    //
    // # Returns:
    // Returns the builder instance (`Self`) to allow for method chaining.
    pub fn with_client_info(mut self, client_info: &client_info::ClientInfo) -> Self {
        self.client_info = Some(client_info.clone());
        self
    }

    // Sets the `config` field for the builder.
    // The `config` field holds the configuration data needed for the client, including connection settings.
    //
    // # Arguments:
    // - `config`: A reference to a `Config` struct that contains the configuration for the client.
    //
    // # Returns:
    // Returns the builder instance (`Self`) to allow for method chaining.
    pub fn with_config(mut self, config: &config::Config) -> Self {
        self.config = Some(config.clone());
        self
    }

    // Sets the `handles` field for the builder.
    // The `handles` field manages the request handlers that facilitate the communication with the Volcengine API.
    //
    // # Arguments:
    // - `handles`: A reference to a `Handles` struct that contains request handlers.
    //
    // # Returns:
    // Returns the builder instance (`Self`) to allow for method chaining.
    pub fn with_handles(mut self, handles: &handles::Handles) -> Self {
        self.handles = Some(handles.clone());
        self
    }

    // Builds the `Client` instance from the builder, performing necessary validation on the required fields.
    // If any of the required fields (`client_info`, `config`, `handles`) are missing, an error is returned.
    //
    // # Returns:
    // - `Ok(Client)`: If all required fields are set, returns the constructed `Client`.
    // - `Err(error::Error)`: If any required field is missing, returns an error indicating which field is missing.
    pub fn build(self) -> Result<Client, error::Error> {
        // Validate that `client_info` is set
        if self.client_info.is_none() {
            return Err(error::Error::ErrUtilClientBuildClientNo(
                "client_info".to_string(),
            ));
        }

        // Validate that `config` is set
        if self.config.is_none() {
            return Err(error::Error::ErrUtilClientBuildClientNo(
                "config".to_string(),
            ));
        }

        // Validate that `handles` is set
        if self.handles.is_none() {
            return Err(error::Error::ErrUtilClientBuildClientNo(
                "handles".to_string(),
            ));
        }

        // Return the fully constructed `Client` object with the set fields
        Ok(Client {
            client_info: self.client_info.unwrap(),
            config: self.config.unwrap(),
            handles: self.handles.unwrap(),
        })
    }
}
