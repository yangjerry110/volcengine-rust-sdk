/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 16:33:22
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 11:07:30
 * @Description: Operation
 *
 * This code defines an `Operation` struct that represents an HTTP operation
 * (e.g., API requests). It uses a builder pattern for constructing the operation
 * with various attributes such as `name`, `http_method`, and `http_path`.
 * The code also defines related enums and helper methods.
 */
use crate::volcengine::error::error;
use crate::volcengine::request::operation_config;

/// Represents an HTTP operation with a name, HTTP method, and HTTP path.
#[derive(Debug, Clone)]
pub struct Operation {
    // The name of the operation (e.g., GetUser, CreateUser).
    pub name: operation_config::operation_name::OperationName,
    // The HTTP method used for the operation (e.g., GET, POST).
    pub http_method: operation_config::operation_http_method::OperationHttpMethod,
    // The HTTP path for the operation (e.g., "/user").
    pub http_path: operation_config::operation_http_path::OperationHttpPath,
}

impl Operation {
    /// Provides a builder to construct an `Operation` instance step by step.
    ///
    /// This method returns a new `OperationBuilder` object. The builder pattern allows for
    /// gradual construction of the `Operation` object, with flexibility to set each field
    /// individually and validate them before finalizing the construction.
    ///
    /// # Returns
    /// A new `OperationBuilder` to build the `Operation` object.
    pub fn builder() -> OperationBuilder {
        // Initialize an empty builder with no fields set.
        OperationBuilder {
            name: None,
            http_method: None,
            http_path: None,
        }
    }
}

/// Builder struct used to gradually build an `Operation` instance.
pub struct OperationBuilder {
    // Optional name of the operation (e.g., "GetUser").
    pub name: Option<operation_config::operation_name::OperationName>,
    // Optional HTTP method for the operation (e.g., GET, POST).
    pub http_method: Option<operation_config::operation_http_method::OperationHttpMethod>,
    // Optional HTTP path for the operation (e.g., "/user").
    pub http_path: Option<operation_config::operation_http_path::OperationHttpPath>,
}

impl OperationBuilder {
    /// Sets the operation name in the builder.
    ///
    /// This method allows you to set the `name` field of the `Operation` object that will
    /// be constructed. The operation name typically corresponds to the action or API endpoint
    /// (e.g., "GetUser", "CreateUser").
    ///
    /// # Arguments
    /// * `operation_name` - The name of the operation (e.g., "GetUser").
    ///
    /// # Returns
    /// The builder with the updated operation name.
    pub fn with_operation_name(
        mut self,
        operation_name: operation_config::operation_name::OperationName,
    ) -> Self {
        self.name = Some(operation_name);
        self
    }

    /// Sets the HTTP method in the builder.
    ///
    /// This method allows you to set the `http_method` field of the `Operation` object that will
    /// be constructed. The HTTP method determines whether the request will use GET, POST, PUT, etc.
    ///
    /// # Arguments
    /// * `operation_http_method` - The HTTP method for the operation (e.g., GET, POST).
    ///
    /// # Returns
    /// The builder with the updated HTTP method.
    pub fn with_operation_http_method(
        mut self,
        operation_http_method: operation_config::operation_http_method::OperationHttpMethod,
    ) -> Self {
        self.http_method = Some(operation_http_method);
        self
    }

    /// Sets the HTTP path in the builder.
    ///
    /// This method allows you to set the `http_path` field of the `Operation` object that will
    /// be constructed. The HTTP path defines the specific API endpoint or route for the operation
    /// (e.g., "/user", "/product").
    ///
    /// # Arguments
    /// * `operation_http_path` - The HTTP path for the operation (e.g., "/user").
    ///
    /// # Returns
    /// The builder with the updated HTTP path.
    pub fn with_operation_http_path(
        mut self,
        operation_http_path: operation_config::operation_http_path::OperationHttpPath,
    ) -> Self {
        self.http_path = Some(operation_http_path);
        self
    }

    /// Finalizes the builder and creates an `Operation` instance.
    ///
    /// This method validates the required fields and constructs the `Operation` object.
    /// If any required fields are missing, it returns an error. Otherwise, it returns the
    /// fully constructed `Operation`.
    ///
    /// # Returns
    /// * `Ok(Operation)` - A fully constructed `Operation` if all fields are provided.
    /// * `Err(Error)` - An error if any required fields are missing.
    pub fn build(self) -> Result<Operation, error::Error> {
        // Validate that the operation name is provided.
        if self.name.is_none() {
            return Err(error::Error::ErrUtilRequestBuildOperationNo(
                "operation_name".to_string(),
            ));
        }

        // Validate that the HTTP method is provided.
        if self.http_method.is_none() {
            return Err(error::Error::ErrUtilRequestBuildOperationNo(
                "operation_http_method".to_string(),
            ));
        }

        // The HTTP path is optional and defaults to a default value if not provided, so it's not strictly validated.
        // If not provided, the operation will still be built with a default path.
        //
        // Validate HTTP path if needed:
        // if self.http_path.is_none() {
        //     return Err(error::Error::ErrUtilRequestBuildOperationNo(
        //         "operation_http_path".to_string(),
        //     ));
        // }

        // Return a new Operation instance after validating all required fields.
        Ok(Operation {
            name: self.name.unwrap(),
            http_method: self.http_method.unwrap(),
            // Use the provided http_path, or default to an empty path if not specified.
            http_path: self.http_path.unwrap_or_default(),
        })
    }
}
