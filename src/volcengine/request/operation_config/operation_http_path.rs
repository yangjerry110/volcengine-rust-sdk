/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-30 10:50:53
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 16:36:49
 * @Description: operation http path
 */

/// Enum representing possible HTTP paths that can be used in API operations.
/// This enum provides a type - safe way to specify the HTTP path for an API request.
/// Currently, it only includes the `Default` path, which maps to the root path ("/").
/// The `Debug` derive allows for easy debugging by providing a default implementation
/// of the `fmt::Debug` trait, which enables printing the enum variants in a readable format.
/// The `Clone` derive allows for creating copies of the enum values when needed.
#[derive(Debug, Clone)]
pub enum OperationHttpPath {
    /// Represents the default HTTP path, which is the root path ("/").
    /// This is typically used when no specific path is required for an API request.
    Default,
}

/// Implementation of the `ToString` trait for the `OperationHttpPath` enum.
/// This allows converting an instance of `OperationHttpPath` into a string representation.
impl ToString for OperationHttpPath {
    /// Converts an `OperationHttpPath` instance into a string.
    ///
    /// # Returns
    /// - A `String` representing the HTTP path. For the `Default` variant, it returns "/".
    fn to_string(&self) -> String {
        match self {
            // Map the `Default` variant to the root path ("/")
            OperationHttpPath::Default => "/",
        }
        // Convert the string literal to a `String` type
        .to_string()
    }
}

/// Implementation of the `Default` trait for the `OperationHttpPath` enum.
/// This provides a default value for the `OperationHttpPath` type.
impl Default for OperationHttpPath {
    /// Returns the default value for `OperationHttpPath`.
    ///
    /// # Returns
    /// - An instance of `OperationHttpPath::Default`, which represents the root path ("/").
    fn default() -> Self {
        OperationHttpPath::Default
    }
}
