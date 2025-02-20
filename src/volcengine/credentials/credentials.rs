/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 10:34:24
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 14:15:55
 * @Description: credentials
 */
/// Represents the authentication credentials required to access Volcengine services.
///
/// This struct holds the necessary credentials used for authenticating API requests  
/// to Volcengine. These credentials are typically provided by the service and must  
/// be securely managed to prevent unauthorized access.
///
/// # Fields
/// - `access_key_id` (`String`): The access key ID associated with the user or application.
///   This key acts as a unique identifier for authentication.
/// - `secret_access_key` (`String`): The secret access key associated with the access key ID.
///   This key must be kept secure, as it is used to sign API requests.
///
/// # Security Considerations
/// - The `secret_access_key` should **never** be exposed in logs, environment variables,  
///   or source code repositories.
/// - Always use secure storage mechanisms, such as environment variables, secrets management  
///   tools, or encrypted configuration files, to store credentials.
/// - Rotate credentials periodically to minimize security risks.
///
/// # Example Usage
/// ```rust
/// let creds = Credentials {
///     access_key_id: "your-access-key-id".to_string(),
///     secret_access_key: "your-secret-access-key".to_string(),
/// };
///
/// println!("Access Key ID: {}", creds.access_key_id);
/// ```
///
/// This struct is often used in conjunction with authentication mechanisms when making  
/// API requests to Volcengine services.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// The access key ID for authentication.
    pub access_key_id: String,

    /// The secret access key for authentication (must be kept secure).
    pub secret_access_key: String,
}

/// Represents a set of credentials used for authenticating API requests.
///
/// This struct holds the access key ID and secret access key required for signing
/// requests and authenticating with the API service. These credentials are used
/// to prove the identity of the user or application making the requests and to
/// ensure that the requests are authorized.
///
/// The `Credentials` struct provides a simple, efficient way to manage the credentials
/// needed for accessing protected resources in the Volcengine API, and it is typically
/// used in API client configurations where authentication is necessary.
///
/// # Example
/// ```rust
/// let creds = Credentials::new("your-access-key-id", "your-secret-access-key");
/// println!("Access Key ID: {}", creds.access_key_id);
/// println!("Secret Access Key: {}", creds.secret_access_key);
/// ```
/// This example demonstrates how to create a new instance of the `Credentials` struct
/// using the `new` method, providing the required access key ID and secret access key.
impl Credentials {
    /// Creates a new instance of the `Credentials` struct.
    ///
    /// This constructor method takes in the access key ID and the secret access key
    /// as string slices (`&str`) and returns a new `Credentials` instance with these values.
    ///
    /// # Parameters
    /// - `access_key_id` (`&str`): The access key ID associated with the user or application.  
    ///   This is used for identifying the user or system when making API requests.
    /// - `secret_access_key` (`&str`): The secret access key that corresponds to the access key ID.  
    ///   This is used for signing the requests to ensure the authenticity of the caller.
    ///
    /// # Returns
    /// Returns a new `Credentials` instance (`Self`) containing the provided access key ID and
    /// secret access key as `String` values.
    ///
    /// # Example
    /// ```rust
    /// let creds = Credentials::new("your-access-key-id", "your-secret-access-key");
    /// println!("Access Key ID: {}", creds.access_key_id);
    /// println!("Secret Access Key: {}", creds.secret_access_key);
    /// ```
    pub fn new(access_key_id: &str, secret_access_key: &str) -> Self {
        Credentials {
            access_key_id: access_key_id.to_string(),
            secret_access_key: secret_access_key.to_string(),
        }
    }
}
