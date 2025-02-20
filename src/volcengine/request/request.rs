/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 16:35:22
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 11:36:18
 * @Description: request
 */
use crate::volcengine::client::client_info;
use crate::volcengine::client::config;
use crate::volcengine::error::error;
use crate::volcengine::request::handles;
use crate::volcengine::request::operation;
use crate::volcengine::request::request;
use crate::volcengine::request::send;
use crate::volcengine::request::send::SendRequest;
use std::collections::HashMap;
use std::future::Future;

/// Trait representing an API request.
///
/// This trait is designed for types that represent API requests. It provides methods to
/// convert the request into a `HashMap` and a byte array, making it easier to prepare
/// the request for sending over HTTP. Any type implementing this trait must also implement
/// the `Send` trait, ensuring that the request can be safely transferred between threads.
pub trait ApiRequest: Send {
    /// Converts the request into a `HashMap<String, String>`.
    ///
    /// This method takes the request and transforms it into a key-value pair representation
    /// where both keys and values are of type `String`. This is typically useful for
    /// serializing request data or for cases where the request needs to be converted
    /// to a more human-readable format (like for logging or debugging).
    ///
    /// # Returns
    /// - `HashMap<String, String>`: A `HashMap` where the keys are strings and the values are strings,
    /// representing the parameters or fields of the request.
    ///
    /// # Example
    /// ```rust
    /// let request = MyRequest { ... };
    /// let hashmap = request.to_hashmap();
    /// ```
    fn to_hashmap(&self) -> HashMap<String, String>;

    /// Converts the request into a byte array suitable for sending in an HTTP request.
    ///
    /// This method converts the request data into a `Vec<u8>`, which represents the request body
    /// as a sequence of bytes. This is particularly useful when the request needs to be sent
    /// over the network, for example, in the body of an HTTP POST or PUT request.
    ///
    /// # Returns
    /// - `Vec<u8>`: A `Vec<u8>` containing the byte representation of the request body, which can
    /// be sent as the body of an HTTP request.
    ///
    /// # Example
    /// ```rust
    /// let request = MyRequest { ... };
    /// let byte_body = request.to_body();
    /// ```
    fn to_body(&self) -> Vec<u8>;
}

/// Trait for handling requests to the Volcengine API.
///
/// This trait defines the core methods required for interacting with the Volcengine API. It includes
/// a method to format requests into a `HashMap` and another to send the API request and handle the response.
/// Implementing this trait allows types to convert requests into the appropriate format and handle communication
/// with the Volcengine service.
pub trait RequestVolcengine {
    /// Formats the provided request into a `HashMap<String, String>`.
    ///
    /// This method takes a serializable request object and converts it into a `HashMap` where each field
    /// in the request is represented as a key-value pair. It filters out any `null` or empty values to ensure
    /// only valid fields are included in the resulting map. This can be useful for serializing data that
    /// will be sent to the Volcengine API, making it easier to construct HTTP request bodies or query parameters.
    ///
    /// # Parameters
    /// - `request`: A reference to a serializable object (must implement `serde::Serialize`).
    ///
    /// # Returns
    /// - `HashMap<String, String>`: A `HashMap` where the keys are field names and the values are their corresponding
    /// values from the request. Null or empty values are excluded.
    ///
    /// # Example
    /// ```rust
    /// let request = MyRequest { ... };
    /// let formatted_map = request.format_request_to_hashmap(&request);
    /// ```
    fn format_request_to_hashmap<T: serde::Serialize>(request: &T) -> HashMap<String, String>;

    /// Sends an API request to the Volcengine API and returns the response.
    ///
    /// This method is responsible for sending the request to the Volcengine API and returning the response,
    /// or an error if something goes wrong during the request. The request must implement the `ApiRequest` trait,
    /// which ensures that the necessary data can be serialized and transmitted to the API.
    ///
    /// # Parameters
    /// - `request`: An instance that implements the `ApiRequest` trait. This represents the request data to be sent.
    ///
    /// # Returns
    /// - `Result<reqwest::Response, error::Error>`: A `Result` containing either the `reqwest::Response` if the request
    /// was successful, or an `error::Error` if something went wrong during the request.
    ///
    /// # Example
    /// ```rust
    /// let request = MyApiRequest { ... };
    /// let response = my_request_handler.send(request).await;
    /// ```
    fn send<T: request::ApiRequest>(
        &self,
        request: T,
    ) -> impl Future<Output = Result<reqwest::Response, error::Error>>;
}

/// Represents a request to the Volcengine API.
///
/// This struct holds all the necessary components for making a request to the Volcengine API.
/// It contains the configuration, client information, operation details, and handles to manage
/// the request operations. The struct is designed to encapsulate the various elements required
/// for interacting with the Volcengine service, ensuring that all necessary data is included
/// when making the API request.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Allows unused code, useful during development or testing.
pub struct Request {
    /// Configuration settings for the request.
    ///
    /// This field contains the configuration settings necessary to make the request. It might
    /// include things like authentication details, API keys, endpoint URLs, or other settings
    /// related to the request environment.
    pub config: config::Config,

    /// Information about the client making the request.
    ///
    /// This field contains details about the client or the system that is making the request.
    /// This could include information such as the client ID, version, operating system, and
    /// any other metadata relevant to identifying the client.
    pub client_info: client_info::ClientInfo,

    /// Handles for managing request operations.
    ///
    /// This field provides handles to manage the lifecycle and execution of the request.
    /// These could include handles for making retries, timeouts, managing API rate limits,
    /// or other request-related operations.
    pub handles: handles::Handles,

    /// Operation details for the request.
    ///
    /// This field holds the specific operation that the request will perform. It might include
    /// data like the action or API method being called, parameters for that action, or other
    /// relevant details for the request's operation.
    pub operation: operation::Operation,
}

/// Implementation of the `Request` struct, providing methods to construct and manage a request.
///
/// This block of code defines methods associated with the `Request` struct, such as the `builder` method.
/// The `Request` struct represents an API request to the Volcengine API and is composed of various components
/// including the configuration, client information, handles, and operation. The methods in this `impl` block
/// allow for the creation and management of these components, making it easier to construct and send requests.
///
/// The builder pattern is used here, allowing you to step-by-step configure the components of the `Request`
/// before it is finalized and sent. This is especially useful for building requests with varying components.
///
/// # Example Usage
/// ```rust
/// let request = Request::builder()
///     .config(config)
///     .client_info(client_info)
///     .handles(handles)
///     .operation(operation)
///     .build();
/// ```
impl Request {
    /// Constructs a new `RequestBuilder` instance for building a `Request`.
    ///
    /// This method provides a fluent interface for constructing a `Request` step by step. It allows
    /// for setting the configuration, client information, handles, and operation before finalizing
    /// and creating the actual `Request` object. This is useful when you need to build a request
    /// with optional components that may vary between requests.
    ///
    /// # Returns
    /// Returns a `RequestBuilder` that enables the construction of a `Request` object. The builder
    /// can be used to set each of the fields in the `Request` struct, allowing for customization
    /// of the request before it is finalized.
    ///
    /// # Example
    /// ```
    /// let request = Request::builder()
    ///     .config(config)
    ///     .client_info(client_info)
    ///     .handles(handles)
    ///     .operation(operation)
    ///     .build();
    /// ```
    pub fn builder() -> RequestBuilder {
        RequestBuilder {
            config: None,
            client_info: None,
            handles: None,
            operation: None,
        }
    }
}

/// Implementation of the `RequestVolcengine` trait for the `Request` struct.
///
/// This block provides methods for formatting the request into a `HashMap` and sending an API request.
/// The methods implement the functionality needed to interact with the Volcengine API. The `Request` struct
/// is adapted to handle serialization and request sending according to the Volcengine API's requirements.
///
/// The methods in this implementation allow for easy conversion of the request data to a format expected
/// by the Volcengine API and also provide a way to send the request over HTTP using the `reqwest` library.
///
/// # Example Usage
/// ```rust
/// let request = Request::builder()
///     .config(config)
///     .client_info(client_info)
///     .handles(handles)
///     .operation(operation)
///     .build();
///
/// let result = request.send(api_request);
/// ```
impl RequestVolcengine for Request {
    /// Formats the request into a `HashMap<String, String>`.
    ///
    /// # Parameters
    /// - `request`: A reference to a serializable object.
    ///
    /// # Returns
    /// Returns a `HashMap<String, String>` representation of the request, filtering out null or empty values.
    fn format_request_to_hashmap<T: serde::Serialize>(request: &T) -> HashMap<String, String> {
        // Serialize the request into a JSON value.
        let value = serde_json::to_value(request).unwrap();
        // Convert the JSON value into a map of key-value pairs.
        let map = value.as_object().unwrap();

        // Convert each entry from `Value` to `String`, filtering out empty or null values.
        let mut request_hashmap: HashMap<String, String> = HashMap::new();
        for (k, v) in map.iter() {
            if v.is_null() || v == "" || v == "null" {
                // Skip null, empty, or "null" values.
                continue;
            }

            if v.is_array() {
                // If the value is an array, convert each element to `key.index=value`.
                if let Some(arr) = v.as_array() {
                    if !arr.is_empty() {
                        // Map each array element to "key.index=value" format
                        for (i, x) in arr.iter().enumerate() {
                            // judge x object
                            if x.is_object() {
                                if let Some(mapx) = x.as_object() {
                                    for (mapx_key, mapx_value) in mapx.iter() {
                                        // judge mapx_value
                                        if mapx_value.is_null()
                                            || mapx_value == ""
                                            || mapx_value == "null"
                                        {
                                            // Skip null, empty, or "null" values.
                                            continue;
                                        }

                                        // judge mapx_value is_array
                                        if mapx_value.is_array() {
                                            if let Some(mapx_value_arr) = mapx_value.as_array() {
                                                if !mapx_value_arr.is_empty() {
                                                    // for mapx_value_arr
                                                    for (
                                                        mapx_value_arr_key,
                                                        mapx_value_arr_value,
                                                    ) in mapx_value_arr.iter().enumerate()
                                                    {
                                                        // judge mapx_value
                                                        if mapx_value_arr_value.is_null()
                                                            || mapx_value_arr_value == ""
                                                            || mapx_value_arr_value == "null"
                                                        {
                                                            // Skip null, empty, or "null" values.
                                                            continue;
                                                        }
                                                        // insert request_hashmap
                                                        request_hashmap.insert(
                                                            format!(
                                                                "{}.{}.{}.{}",
                                                                k.clone(),
                                                                i + 1,
                                                                mapx_key,
                                                                mapx_value_arr_key + 1
                                                            ),
                                                            mapx_value_arr_value
                                                                .to_string()
                                                                .replace("\"", ""),
                                                        );
                                                    }
                                                }
                                            }
                                            continue;
                                        }

                                        // insert request_hashmap
                                        request_hashmap.insert(
                                            format!("{}.{}.{}", k.clone(), i + 1, mapx_key),
                                            mapx_value.to_string().replace("\"", ""),
                                        );
                                    }
                                }
                                continue;
                            }

                            if !x.is_null() {
                                request_hashmap.insert(
                                    format!("{}.{}", k, i + 1),
                                    x.to_string().replace("\"", ""),
                                );
                            }
                        }
                    }
                }
            } else {
                // judge k
                // if k == PolicyDocument
                if k == "PolicyDocument" {
                    // policy_document
                    let mut policy_document = v.to_string().replace("\\", "");
                    policy_document = policy_document[1..policy_document.len() - 1].to_string();
                    request_hashmap.insert(k.clone(), policy_document);
                    continue;
                }

                // judge k
                // if k == NewPolicyDocument
                if k == "NewPolicyDocument" {
                    // policy_document
                    let mut policy_document = v.to_string().replace("\\", "");
                    policy_document = policy_document[1..policy_document.len() - 1].to_string();
                    request_hashmap.insert(k.clone(), policy_document);
                    continue;
                }

                // Otherwise, directly convert the value to a string and remove quotes
                request_hashmap.insert(k.clone(), v.to_string().replace("\"", ""));
            }
        }

        request_hashmap
    }

    /// Sends the request to the Volcengine API.
    ///
    /// # Parameters
    /// - `request`: An instance implementing the `ApiRequest` trait.
    ///
    /// # Returns
    /// Returns a `Result` containing the `reqwest::Response` or an error.
    ///
    /// This function sends the request and waits for the API response asynchronously.
    async fn send<T: request::ApiRequest>(
        &self,
        request: T,
    ) -> Result<reqwest::Response, error::Error> {
        send::Send::set_request(self).send(&request).await
    }
}

/// Builder for constructing a `Request` instance.
///
/// The `RequestBuilder` struct provides a flexible way to create a `Request` object by gradually setting its
/// fields using the builder pattern. Each field of the `Request` is optional and can be set individually.
/// Once all desired fields are set, the `build` method can be called to finalize the construction of the `Request`
/// object. This builder pattern ensures that the `Request` is properly configured before being used.
///
/// # Example Usage
/// ```rust
/// let request = RequestBuilder::new()
///     .config(config)
///     .client_info(client_info)
///     .handles(handles)
///     .operation(operation)
///     .build();
/// ```
///
/// # Fields
/// - `config`: An optional `Config` struct that contains the configuration for the request.
/// - `client_info`: An optional `ClientInfo` struct that contains information about the client making the request.
/// - `handles`: An optional `Handles` struct that provides handles for managing request operations.
/// - `operation`: An optional `Operation` struct that holds details about the operation to be performed in the request.
pub struct RequestBuilder {
    pub config: Option<config::Config>, // Optional configuration for the request.
    pub client_info: Option<client_info::ClientInfo>, // Optional client information.
    pub handles: Option<handles::Handles>, // Optional handles for request operations.
    pub operation: Option<operation::Operation>, // Optional operation details.
}

/// Implementation of methods for constructing and finalizing a `Request` object.
///
/// The `RequestBuilder` struct provides methods to set each optional field of the `Request` object. It allows for controlled
/// construction of a `Request` instance by providing a set of methods to set specific properties (such as configuration,
/// client info, handles, and operation details). Once all fields are set, the `build()` method finalizes the creation of
/// the `Request`. This ensures that an incomplete or invalid `Request` cannot be constructed, adhering to the builder pattern.
///
/// The builder pattern provides a more readable and maintainable way to construct complex objects. This also helps in
/// avoiding issues related to having multiple constructors with different signatures.
impl RequestBuilder {
    /// Sets the configuration for the request builder.
    ///
    /// # Parameters
    /// - `config`: A reference to the `Config` instance.
    ///
    /// # Returns
    /// Returns the updated `RequestBuilder` with the provided configuration set.
    pub fn with_config(mut self, config: &config::Config) -> Self {
        self.config = Some(config.clone());
        self
    }

    /// Sets the client information for the request builder.
    ///
    /// # Parameters
    /// - `client_info`: A reference to the `ClientInfo` instance.
    ///
    /// # Returns
    /// Returns the updated `RequestBuilder` with the provided client information set.
    pub fn with_client_info(mut self, client_info: &client_info::ClientInfo) -> Self {
        self.client_info = Some(client_info.clone());
        self
    }

    /// Sets the handles for the request builder.
    ///
    /// # Parameters
    /// - `handles`: A reference to the `Handles` instance.
    ///
    /// # Returns
    /// Returns the updated `RequestBuilder` with the provided handles set.
    pub fn with_handles(mut self, handles: &handles::Handles) -> Self {
        self.handles = Some(handles.clone());
        self
    }

    /// Sets the operation details for the request builder.
    ///
    /// # Parameters
    /// - `operation`: A reference to the `Operation` instance.
    ///
    /// # Returns
    /// Returns the updated `RequestBuilder` with the provided operation details set.
    pub fn with_operation(mut self, operation: &operation::Operation) -> Self {
        self.operation = Some(operation.clone());
        self
    }

    /// Builds the final `Request` object.
    ///
    /// This method checks that all required fields are set. If any of the required fields are missing,
    /// it returns an error. If all fields are provided, it constructs and returns a `Request` object.
    ///
    /// # Returns
    /// Returns a `Result` containing the constructed `Request` or an error if any required field is missing.
    pub fn build(self) -> Result<Request, error::Error> {
        // Check if config is provided.
        if self.config.is_none() {
            return Err(error::Error::ErrUtilRequestBuildRequestNo(
                "config".to_string(),
            ));
        }

        // Check if client_info is provided.
        if self.client_info.is_none() {
            return Err(error::Error::ErrUtilRequestBuildRequestNo(
                "client_info".to_string(),
            ));
        }

        // Check if handles are provided.
        if self.handles.is_none() {
            return Err(error::Error::ErrUtilRequestBuildRequestNo(
                "handles".to_string(),
            ));
        }

        // Check if operation is provided.
        if self.operation.is_none() {
            return Err(error::Error::ErrUtilRequestBuildRequestNo(
                "operation".to_string(),
            ));
        }

        // Return the constructed Request.
        Ok(Request {
            config: self.config.unwrap(),
            client_info: self.client_info.unwrap(),
            handles: self.handles.unwrap(),
            operation: self.operation.unwrap(),
        })
    }
}
