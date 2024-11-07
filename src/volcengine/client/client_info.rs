/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 16:16:16
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-28 16:33:04
 * @Description: client_info
 */
use super::config;
use crate::volcengine::error::error;

#[derive(Debug, Clone)]
// The `ClientInfo` struct holds metadata about the service being used, such as the service name, API version, and signing region.
pub struct ClientInfo {
    pub service_name: config::ClientServiceName, // The name of the client service (e.g., IAM, S3)
    // pub service_id: config::ClientServiceId,
    // pub endpoint: String,
    pub api_version: String, // API version used for this service
    // pub signing_name: String,
    pub signing_region: String, // The region for signing requests
                                // pub json_version: String,
                                // pub target_prefix: String,
}

impl ClientInfo {
    // Provides a `builder` method to create a new `ClientInfoBuilder` for constructing `ClientInfo`.
    pub fn builder() -> ClientInfoBuilder {
        ClientInfoBuilder {
            service_name: None,
            // service_id: None,
            // endpoint: None,
            api_version: None,
            // signing_name: None,
            signing_region: None,
            // json_version: None,
            // target_prefix: None,
        }
    }
}

// impl ClientServiceId {
//     pub fn to_str(&self) -> &str {
//         match self {
//             ClientServiceId::Iam => "iam",
//             ClientServiceId::Empty => "",
//         }
//     }
// }

// The `ClientInfoBuilder` struct follows the builder pattern, allowing users to incrementally build a `ClientInfo` object.
pub struct ClientInfoBuilder {
    pub service_name: Option<config::ClientServiceName>, // Option type to represent optional fields until explicitly set
    // pub service_id: Option<config::ClientServiceId>,
    // pub endpoint: Option<String>,
    pub api_version: Option<String>, // API version, if set
    // pub signing_name: Option<String>,
    pub signing_region: Option<String>, // Signing region, if set
                                        // pub json_version: Option<String>,
                                        // pub target_prefix: Option<String>,
}

impl ClientInfoBuilder {
    // Sets the `service_name` in the builder and returns the updated builder.
    pub fn with_service_name(mut self, service_name: config::ClientServiceName) -> Self {
        self.service_name = Some(service_name);
        self
    }

    // pub fn with_service_id(mut self, service_id: config::ClientServiceId) -> Self {
    //     self.service_id = Some(service_id);
    //     self
    // }

    // pub fn with_endpoint(mut self, endpoint: &str) -> Self {
    //     self.endpoint = Some(endpoint.to_string());
    //     self
    // }

    // Sets the `api_version` in the builder and returns the updated builder.
    pub fn with_api_version(mut self, api_version: &str) -> Self {
        self.api_version = Some(api_version.to_string());
        self
    }

    // pub fn with_signing_name(mut self, signing_name: &str) -> Self {
    //     self.signing_name = Some(signing_name.to_string());
    //     self
    // }

    // Sets the `signing_region` in the builder and returns the updated builder.
    pub fn with_signing_region(mut self, signing_region: &str) -> Self {
        self.signing_region = Some(signing_region.to_string());
        self
    }

    // pub fn with_json_version(mut self,json_version : &str) -> Self {
    //     self.json_version = Some(json_version.to_string());
    //     self
    // }

    // pub fn with_target_prefix(mut self,target_prefix : &str) -> Self {
    //     self.target_prefix = Some(target_prefix.to_string());
    //     self
    // }

    // Builds the `ClientInfo` object after validating that all required fields are set.
    pub fn build(self) -> Result<ClientInfo, error::Error> {
        // Check if `service_name` is set
        if self.service_name.is_none() {
            return Err(error::Error::ErrUtilClientBuildClientInfoNo(
                "service_name".to_string(),
            ));
        }

        // // Check if `service_id` is set
        // if self.service_id.is_none() {
        //     return Err(error::Error::ErrUtilClientBuildClientInfoNo(
        //         "service_id".to_string(),
        //     ));
        // }

        // // Check if `endpoint` is set
        // if self.endpoint.is_none() {
        //     return Err(error::Error::ErrUtilClientBuildClientInfoNo(
        //         "endpoint".to_string(),
        //     ));
        // }

        // Check if `api_version` is set
        if self.api_version.is_none() {
            return Err(error::Error::ErrUtilClientBuildClientInfoNo(
                "api_version".to_string(),
            ));
        }

        // // Check if `signing_name` is set
        // if self.signing_name.is_none() {
        //     return Err(error::Error::ErrUtilClientBuildClientInfoNo(
        //         "signing_name".to_string(),
        //     ));
        // }

        // Check if `signing_region` is set
        if self.signing_region.is_none() {
            return Err(error::Error::ErrUtilClientBuildClientInfoNo(
                "signing_region".to_string(),
            ));
        }

        // Create and return a `ClientInfo` object after all validations are successful
        let client_info = ClientInfo {
            service_name: self.service_name.unwrap(),
            // service_id: self.service_id.unwrap(),
            // endpoint: self.endpoint.unwrap(),
            api_version: self.api_version.unwrap(),
            // signing_name: self.signing_name.unwrap(),
            signing_region: self.signing_region.unwrap(),
            // json_version: self.json_version.unwrap_or_default(),
            // target_prefix: self.target_prefix.unwrap_or_default(),
        };

        // return
        Ok(client_info)
    }
}
