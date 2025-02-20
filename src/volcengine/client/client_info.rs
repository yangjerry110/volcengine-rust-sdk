/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 16:16:16
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 14:28:58
 * @Description: client_info
 */
use crate::volcengine::client::config;
use crate::volcengine::error::error;

#[derive(Debug, Clone)]
// The `ClientInfo` struct holds metadata about a cloud service, including the service's name,
// API version, and the signing region required for requests.
pub struct ClientInfo {
    pub service_name: config::ClientServiceName, // The name of the service (e.g., IAM, S3)
    // pub service_id: config::ClientServiceId, // A unique identifier for the service (optional)
    // pub endpoint: String, // The endpoint URL for the service (optional)
    pub api_version: String, // The API version used for the service (e.g., "v1", "v2")
    // pub signing_name: String, // The name used for signing requests (optional)
    // The region used for signing requests, typically the AWS region
    // pub json_version: String, // The version of the JSON protocol used (optional)
    // pub target_prefix: String, // The target prefix for the request (optional)
    pub signing_region: String,
}

// The `ClientInfo` struct provides a way to represent metadata required for making authenticated
// API requests to cloud services. This implementation provides a way to configure and manage information
// about the service being interacted with, such as its name, API version, and region, which are needed
// to correctly sign and send requests to a cloud service endpoint.
//
// The `impl ClientInfo` block provides an implementation for the methods related to creating and managing
// `ClientInfo` instances. One key method is `builder()`, which returns a `ClientInfoBuilder` that allows
// clients to incrementally construct a `ClientInfo` object using the builder pattern. The builder pattern
// is useful because it allows for flexible construction of the object, providing an easy way to set only
// the necessary fields while leaving others as optional.
//
// The `ClientInfo` struct is designed for use when interacting with a cloud service API and needs
// metadata for signing requests. The builder pattern ensures that the object can be properly configured
// before being used to make requests.
impl ClientInfo {
    // Provides a method to create a new `ClientInfoBuilder`, which follows the builder pattern
    // for constructing `ClientInfo` with required fields.
    pub fn builder() -> ClientInfoBuilder {
        ClientInfoBuilder {
            service_name: None, // The service name, which must be set later
            // service_id: None, // Optional service identifier
            // endpoint: None, // Optional endpoint URL
            api_version: None, // The API version, which must be set later
            // signing_name: None, // Optional signing name
            // The signing region, which must be set later
            // json_version: None, // Optional JSON version
            // target_prefix: None, // Optional target prefix
            signing_region: None,
        }
    }
}

// The `ClientInfoBuilder` struct implements the builder pattern, allowing clients to incrementally
// set up fields to construct a valid `ClientInfo` object.
pub struct ClientInfoBuilder {
    pub service_name: Option<config::ClientServiceName>, // Option type allows for optional fields
    // pub service_id: Option<config::ClientServiceId>, // Optional service ID
    // pub endpoint: Option<String>, // Optional endpoint URL
    pub api_version: Option<String>, // The API version, if provided
    // pub signing_name: Option<String>, // Optional signing name
    // The signing region, if provided
    // pub json_version: Option<String>, // Optional JSON version
    // pub target_prefix: Option<String>, // Optional target prefix
    pub signing_region: Option<String>,
}

// The `ClientInfoBuilder` struct implements the builder pattern for creating instances of the
// `ClientInfo` struct. The builder pattern is useful for constructing an object step by step, allowing
// optional fields to be set and ensuring that the final object is valid before being built.
//
// `ClientInfoBuilder` starts with all fields as `None` and provides a set of methods to progressively
// assign values to the `ClientInfo` struct. Once all required fields are set, the `build` method can be
// called to generate a fully constructed `ClientInfo` instance. If any required fields are missing,
// an error will be returned.
//
// This pattern is designed to make it easier to create `ClientInfo` objects with varying configurations,
// especially in cases where some fields are optional or have default values.
impl ClientInfoBuilder {
    // Sets the `service_name` field for the builder.
    // The `service_name` field holds the name of the cloud service (e.g., IAM, S3)
    // that the client will interact with.
    //
    // # Arguments:
    // - `service_name`: The name of the service as a `ClientServiceName`.
    //
    // # Returns:
    // Returns the builder instance (`Self`) to allow for method chaining.
    pub fn with_service_name(mut self, service_name: config::ClientServiceName) -> Self {
        self.service_name = Some(service_name); // Set the `service_name` in the builder.
        self // Return the builder instance for method chaining.
    }

    // // Sets the `service_id` field (commented out for now).
    // pub fn with_service_id(mut self, service_id: config::ClientServiceId) -> Self {
    //     self.service_id = Some(service_id);
    //     self
    // }

    // // Sets the `endpoint` field (commented out for now).
    // pub fn with_endpoint(mut self, endpoint: &str) -> Self {
    //     self.endpoint = Some(endpoint.to_string());
    //     self
    // }

    // Sets the `api_version` field for the builder.
    // The `api_version` field holds the version of the API that the client will use.
    //
    // # Arguments:
    // - `api_version`: A string slice (`&str`) representing the version of the API (e.g., "v1", "v2").
    //
    // # Returns:
    // Returns the builder instance (`Self`) to allow for method chaining.
    pub fn with_api_version(mut self, api_version: &str) -> Self {
        self.api_version = Some(api_version.to_string()); // Set the `api_version` in the builder.
        self // Return the builder instance for method chaining.
    }

    // // Sets the `signing_name` field (commented out for now).
    // pub fn with_signing_name(mut self, signing_name: &str) -> Self {
    //     self.signing_name = Some(signing_name.to_string());
    //     self
    // }

    // Sets the `signing_region` field for the builder.
    // The `signing_region` field specifies the region used for signing requests,
    // typically the AWS region or a similar regional identifier.
    //
    // # Arguments:
    // - `signing_region`: A string slice (`&str`) representing the region used for signing requests.
    //
    // # Returns:
    // Returns the builder instance (`Self`) to allow for method chaining.
    pub fn with_signing_region(mut self, signing_region: &str) -> Self {
        self.signing_region = Some(signing_region.to_string()); // Set the `signing_region` in the builder.
        self // Return the builder instance for method chaining.
    }

    // // Sets the `json_version` field (commented out for now).
    // pub fn with_json_version(mut self,json_version : &str) -> Self {
    //     self.json_version = Some(json_version.to_string());
    //     self
    // }

    // // Sets the `target_prefix` field (commented out for now).
    // pub fn with_target_prefix(mut self,target_prefix : &str) -> Self {
    //     self.target_prefix = Some(target_prefix.to_string());
    //     self
    // }

    // Finalizes the builder and creates a `ClientInfo` object.
    // Returns an error if any required fields are missing or not set.
    //
    // The `build` function validates that all necessary fields are provided before constructing
    // the `ClientInfo` object. If any required field is missing, an error is returned.
    //
    // # Returns:
    // - `Result<ClientInfo, error::Error>`: On success, returns a `ClientInfo` object. On failure,
    //   returns an error indicating which required field is missing.
    pub fn build(self) -> Result<ClientInfo, error::Error> {
        // Ensure `service_name` is provided, otherwise return an error
        if self.service_name.is_none() {
            return Err(error::Error::ErrUtilClientBuildClientInfoNo(
                "service_name".to_string(),
            ));
        }

        // // Ensure `service_id` is provided (if this field is uncommented)
        // if self.service_id.is_none() {
        //     return Err(error::Error::ErrUtilClientBuildClientInfoNo(
        //         "service_id".to_string(),
        //     ));
        // }

        // // Ensure `endpoint` is provided (if this field is uncommented)
        // if self.endpoint.is_none() {
        //     return Err(error::Error::ErrUtilClientBuildClientInfoNo(
        //         "endpoint".to_string(),
        //     ));
        // }

        // Ensure `api_version` is provided, otherwise return an error
        if self.api_version.is_none() {
            return Err(error::Error::ErrUtilClientBuildClientInfoNo(
                "api_version".to_string(),
            ));
        }

        // // Ensure `signing_name` is provided (if this field is uncommented)
        // if self.signing_name.is_none() {
        //     return Err(error::Error::ErrUtilClientBuildClientInfoNo(
        //         "signing_name".to_string(),
        //     ));
        // }

        // Ensure `signing_region` is provided, otherwise return an error
        if self.signing_region.is_none() {
            return Err(error::Error::ErrUtilClientBuildClientInfoNo(
                "signing_region".to_string(),
            ));
        }

        // Construct and return the `ClientInfo` object, filling in the fields.
        let client_info = ClientInfo {
            service_name: self.service_name.unwrap(), // Unwrap to get the value of service_name
            // service_id: self.service_id.unwrap(), // If service_id is used, uncomment this line
            // endpoint: self.endpoint.unwrap(), // If endpoint is used, uncomment this line
            api_version: self.api_version.unwrap(), // Unwrap to get the value of api_version
            // signing_name: self.signing_name.unwrap(), // If signing_name is used, uncomment this line
            // Unwrap to get the value of signing_region
            // json_version: self.json_version.unwrap_or_default(), // If json_version is used, uncomment this line
            // target_prefix: self.target_prefix.unwrap_or_default(), // If target_prefix is used, uncomment this line
            signing_region: self.signing_region.unwrap(),
        };

        // Return the constructed `ClientInfo` object inside a Result.
        Ok(client_info)
    }
}
