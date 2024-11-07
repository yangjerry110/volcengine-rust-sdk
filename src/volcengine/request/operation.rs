/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 16:33:22
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-30 11:18:22
 * @Description: Operation
 *
 * This code defines an `Operation` struct that represents an HTTP operation
 * (e.g., API requests). It uses a builder pattern for constructing the operation
 * with various attributes such as `name`, `http_method`, and `http_path`.
 * The code also defines related enums and helper methods.
 */
use super::operation_config;
use crate::volcengine::error::error;

// Operation struct, representing an API operation with its name, HTTP method, and HTTP path
#[derive(Debug, Clone)]
pub struct Operation {
    pub name: operation_config::operation_name::OperationName, // Name of the operation (e.g., GetUser, CreateUser)
    pub http_method: operation_config::operation_http_method::OperationHttpMethod, // HTTP method used for the operation (e.g., GET, POST)
    pub http_path: operation_config::operation_http_path::OperationHttpPath, // HTTP path for the operation (e.g., default path "/")
}

// Implementation of the `Operation` struct
impl Operation {
    // Provides a builder for constructing an `Operation` instance step by step
    pub fn builder() -> OperationBuilder {
        OperationBuilder {
            name: None,
            http_method: None,
            http_path: None,
        }
    }
}

// Builder struct used to gradually build an `Operation` instance
pub struct OperationBuilder {
    pub name: Option<operation_config::operation_name::OperationName>, // Optional name of the operation
    pub http_method: Option<operation_config::operation_http_method::OperationHttpMethod>, // Optional HTTP method
    pub http_path: Option<operation_config::operation_http_path::OperationHttpPath>, // Optional HTTP path
}

// Implementation of `OperationBuilder` for building an `Operation` instance with provided values
impl OperationBuilder {
    // Set the operation name in the builder
    pub fn with_operation_name(
        mut self,
        operation_name: operation_config::operation_name::OperationName,
    ) -> Self {
        self.name = Some(operation_name);
        self
    }

    // Set the HTTP method in the builder
    pub fn with_operation_http_method(
        mut self,
        operation_http_method: operation_config::operation_http_method::OperationHttpMethod,
    ) -> Self {
        self.http_method = Some(operation_http_method);
        self
    }

    // Set the HTTP path in the builder
    pub fn with_operation_http_path(
        mut self,
        operation_http_path: operation_config::operation_http_path::OperationHttpPath,
    ) -> Self {
        self.http_path = Some(operation_http_path);
        self
    }

    // Final build method to create an `Operation` instance; returns error if any field is missing
    pub fn build(self) -> Result<Operation, error::Error> {
        // Validate operation name
        if self.name.is_none() {
            return Err(error::Error::ErrUtilRequestBuildOperationNo(
                "operation_name".to_string(),
            ));
        }

        // Validate HTTP method
        if self.http_method.is_none() {
            return Err(error::Error::ErrUtilRequestBuildOperationNo(
                "operation_http_method".to_string(),
            ));
        }

        // Validate HTTP path
        if self.http_path.is_none() {
            return Err(error::Error::ErrUtilRequestBuildOperationNo(
                "operation_http_path".to_string(),
            ));
        }

        // Return a new Operation instance after validation
        Ok(Operation {
            name: self.name.unwrap(),
            http_method: self.http_method.unwrap(),
            http_path: self.http_path.unwrap(),
        })
    }
}
