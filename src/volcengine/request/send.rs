use crate::volcengine::error::error;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::sign;
use crate::volcengine::request::sign::SignRequest;
use chrono::Utc;
use reqwest::{Client, RequestBuilder};
use std::collections::HashMap;
use std::future::Future;

/// Trait for handling the creation, signing, and sending of API requests.
/// Provides methods for building request headers, signing them, and sending requests.
pub trait SendRequest {
    /// Sets the request for the struct implementing this trait.
    ///
    /// This method takes a reference to a `request::Request` and returns an instance
    /// of the implementing type with the request set.
    ///
    /// # Arguments
    /// - `request`: A reference to a `request::Request` that will be set.
    ///
    /// # Returns
    /// Returns an instance of the implementing struct with the `request` set.
    fn set_request(request: &request::Request) -> Self;

    /// Sends the request to the server and returns the response.
    ///
    /// This method is responsible for sending the request to the server asynchronously
    /// and returning the server's response. It handles the serialization of the request,
    /// making the API call, and processing the response.
    ///
    /// # Arguments
    /// - `request`: A reference to a type implementing `ApiRequest`, which defines
    ///   the structure of the request.
    ///
    /// # Returns
    /// A future that resolves to a result containing the server's response or an error.
    fn send<T: request::ApiRequest>(
        &self,
        request: &T,
    ) -> impl Future<Output = Result<reqwest::Response, error::Error>>;

    /// Builds the request headers, including all necessary metadata.
    ///
    /// This method is responsible for building the headers for the HTTP request. The headers
    /// are added to the request builder, which will later be used to make the API call.
    ///
    /// # Arguments
    /// - `now_date`: The current date in the required format, typically used for signing headers.
    /// - `request_builder`: A `RequestBuilder` instance that will be populated with the headers.
    /// - `request`: A reference to a type implementing `ApiRequest` to extract specific header data.
    ///
    /// # Returns
    /// Returns a `Result` containing either the populated `RequestBuilder` with headers or an error.
    fn build_request_headers<T: request::ApiRequest>(
        &self,
        now_date: &str,
        request_builder: RequestBuilder,
        request: &T,
    ) -> Result<RequestBuilder, error::Error>;

    /// Builds the request headers and adds the authorization header.
    ///
    /// This method extends `build_request_headers` by adding an authorization header to
    /// the request. The authorization header is typically generated using an HMAC or similar
    /// signing method to ensure secure communication.
    ///
    /// # Arguments
    /// - `now_date`: The current UTC date in a specific format, used for signing the authorization.
    /// - `request`: A reference to a type implementing `ApiRequest`, which provides data for the request.
    /// - `request_builder`: The request builder to which the authorization header will be added.
    ///
    /// # Returns
    /// Returns a `Result` containing the modified `RequestBuilder` with the authorization header
    /// or an error.
    fn build_request_headers_authorization<T: request::ApiRequest>(
        &self,
        now_date: &str,
        request: &T,
        request_builder: RequestBuilder,
    ) -> Result<RequestBuilder, error::Error>;

    /// Builds the request object itself.
    ///
    /// This method is responsible for constructing the base HTTP request, including
    /// the URL, HTTP method, and other essential components required to perform the request.
    ///
    /// # Arguments
    /// - `request`: A reference to a type implementing `ApiRequest` to obtain the necessary data
    ///   for the request, such as the URL and HTTP method.
    ///
    /// # Returns
    /// Returns a `Result` containing the constructed `RequestBuilder` or an error.
    fn build_request<T: request::ApiRequest>(
        &self,
        request: &T,
    ) -> Result<RequestBuilder, error::Error>;

    /// Converts the request parameters into a `HashMap`.
    ///
    /// This method formats the parameters for the request into a `HashMap`, where the keys
    /// are the names of the parameters and the values are their respective values.
    ///
    /// # Arguments
    /// - `request`: A reference to a type implementing `ApiRequest`, which contains the parameters
    ///   to be formatted.
    ///
    /// # Returns
    /// A `HashMap<String, String>` containing the formatted request parameters.
    fn format_request_hashmap<T: request::ApiRequest>(
        &self,
        request: &T,
    ) -> HashMap<String, String>;

    /// Retrieves the current UTC date formatted as a string.
    ///
    /// This method generates the current UTC date in a specific format, which is commonly used
    /// for API signature generation or as part of headers for request authentication.
    ///
    /// # Returns
    /// A string representing the current UTC date in the required format (e.g., `20250205T120000Z`).
    fn get_x_date(&self) -> String;
}

/// Represents a `Send` struct used for handling an API request.
///
/// This struct is designed to hold an API request (`request::Request`) and provide methods
/// for processing and sending the request to a server. The struct can be cloned and debugged
/// for testing or inspection purposes.
#[derive(Debug, Clone)]
pub struct Send {
    /// The request instance to be sent.
    ///
    /// This field holds the request object of type `request::Request`, which contains the data
    /// that will be sent to the API endpoint. It includes parameters such as the URL, method,
    /// and body of the request.
    ///
    /// # Fields
    /// - `request`: The `request::Request` object that will be sent to the server.
    request: request::Request,
}

/**
 * @description: Implementation of SendRequest trait for Send struct.
 * @author: Jerry.Yang
 * @date: 2024-11-08 10:46:32
 * @return {*}
 */
impl SendRequest for Send {
    /// Sets the request for the Send instance.
    ///
    /// # Arguments
    /// - `request`: The request object to be set.
    ///
    /// # Returns
    /// Returns an instance of `Send`.
    fn set_request(request: &request::Request) -> Self {
        Send {
            request: request.clone(),
        }
    }

    /// Sends the HTTP request.
    ///
    /// # Arguments
    /// - `request`: The request object to be sent.
    ///
    /// # Returns
    /// Returns a `Result` containing either the `reqwest::Response` or an error.
    async fn send<T: request::ApiRequest>(
        &self,
        request: &T,
    ) -> Result<reqwest::Response, error::Error> {
        // Get the current date and time in UTC.
        let now_date_string = self.get_x_date();
        let now_date = now_date_string.as_str();

        // Build the request.
        let request_builder = self.build_request(request)?;

        // Build the request headers.
        let request_builder = self.build_request_headers(&now_date, request_builder, request)?;

        // Add the authorization header.
        let request_builder =
            self.build_request_headers_authorization(&now_date, request, request_builder)?;

        // Send the request and handle the response.
        let response = request_builder
            .send()
            .await
            .map_err(|e| error::Error::ErrRequest(e))?;

        // Return the response.
        Ok(response)
    }

    /// Builds the request headers.
    ///
    /// # Arguments
    /// - `now_date`: The current date and time as a string.
    /// - `request_builder`: The request builder.
    /// - `request`: The request object.
    ///
    /// # Returns
    /// Returns the request builder with headers or an error.
    fn build_request_headers<T: request::ApiRequest>(
        &self,
        now_date: &str,
        request_builder: RequestBuilder,
        request: &T,
    ) -> Result<RequestBuilder, error::Error> {
        // Create a signing object.
        let request_sign = sign::Sign::default();

        // Clone the request builder.
        let request_builder_clone = request_builder
            .try_clone()
            .ok_or_else(|| error::Error::ErrRequestBuilderIsNone)?;

        // Add basic headers.
        let request_builder_with_headers = request_builder_clone.header("X-Date", now_date);

        // Get the request method and set headers based on the method.
        let reqwest_request = request_builder
            .build()
            .map_err(|e| error::Error::ErrRequest(e))?;

        // Get the HTTP method.
        let method = reqwest_request.method().as_str();

        // If the method is POST, add additional headers.
        if method == "POST" {
            // Hash the request body to create a digest.
            let payload_hash = hex::encode(request_sign.hash_sha256(&request.to_body()));

            // Add the `X-Content-Sha256` header.
            let request_builder_with_headers =
                request_builder_with_headers.header("X-Content-Sha256", payload_hash);
            return Ok(request_builder_with_headers);
        }

        // Return the request builder with headers.
        Ok(request_builder_with_headers)
    }

    /// Builds the authorization header.
    ///
    /// # Arguments
    /// - `now_date`: The current date and time as a string.
    /// - `request`: The request object.
    /// - `request_builder`: The request builder.
    ///
    /// # Returns
    /// Returns the request builder with the authorization header.
    fn build_request_headers_authorization<T: request::ApiRequest>(
        &self,
        now_date: &str,
        request: &T,
        request_builder: RequestBuilder,
    ) -> Result<RequestBuilder, error::Error> {
        // Clone the request builder.
        let request_builder_clone = request_builder
            .try_clone()
            .ok_or_else(|| error::Error::ErrRequestBuilderIsNone)?;

        // Build the request and get the signing headers.
        let reqwest_request = request_builder_clone
            .build()
            .map_err(|e| error::Error::ErrRequest(e))?;
        let request_sign = sign::Sign::default();
        let sign_headers = request_sign.get_sign_header_keys(&reqwest_request);

        // Create a vector of lowercase signing headers.
        let mut authorization_sign_headers: Vec<String> = Vec::new();
        for sign_header in sign_headers {
            authorization_sign_headers.push(sign_header.to_lowercase());
        }

        // Join the signing headers into a single string.
        let authorization_sign_header_str = authorization_sign_headers.join(";");

        // Generate the signature.
        let signature =
            request_sign.build_signature(now_date, &self.request, request, &request_builder)?;

        // Build the `Authorization` header.
        let short_date = &now_date[..8];
        let authorization = format!(
            r#"HMAC-SHA256 Credential={AccessKey}/{ShortDate}/{Region}/{Service}/request, SignedHeaders={SignedHeaders}, Signature={Signature}"#,
            AccessKey = self.request.config.config.credentials.access_key_id,
            ShortDate = short_date,
            Region = self.request.client_info.signing_region,
            Service = self.request.client_info.service_name.as_str(),
            SignedHeaders = authorization_sign_header_str,
            Signature = signature,
        );

        // Add the authorization header to the request.
        let request_builder_with_headers = request_builder.header("Authorization", authorization);

        // Return the request builder with the authorization header.
        Ok(request_builder_with_headers)
    }

    /// Builds the request.
    ///
    /// # Arguments
    /// - `request`: The request object.
    ///
    /// # Returns
    /// Returns the request builder or an error.
    fn build_request<T: request::ApiRequest>(
        &self,
        request: &T,
    ) -> Result<RequestBuilder, error::Error> {
        let client = Client::new();

        // Format the request parameters into a query string.
        let query_string = self
            .format_request_hashmap(request)
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<String>>()
            .join("&");

        // Set the request address.
        let request_addr = format!(
            "{}{}?{}",
            self.request.config.endpoint,
            self.request.operation.clone().http_path.to_string(),
            query_string
        );

        // Build the request based on the HTTP method.
        let request_builder = match self.request.operation.http_method {
            operation_config::operation_http_method::OperationHttpMethod::GET => {
                client.get(&request_addr)
            }
            operation_config::operation_http_method::OperationHttpMethod::POST => {
                let body = request.to_body();
                client.post(&request_addr).body(body)
            }
        };

        // Return the request builder.
        Ok(request_builder)
    }

    /// Formats the request into a HashMap.
    ///
    /// # Arguments
    /// - `request`: The request object.
    ///
    /// # Returns
    /// Returns the formatted HashMap.
    fn format_request_hashmap<T: request::ApiRequest>(
        &self,
        request: &T,
    ) -> HashMap<String, String> {
        // Define the request HashMap.
        let mut request_hashmap = request.to_hashmap();

        // Set the Action parameter.
        request_hashmap.insert(
            String::from("Action"),
            self.request.operation.name.to_string(),
        );

        // Set the Version parameter.
        request_hashmap.insert(
            String::from("Version"),
            self.request.client_info.api_version.to_string(),
        );

        // Return the formatted HashMap.
        request_hashmap
    }

    /// Gets the current date and time in UTC as a formatted string.
    ///
    /// # Returns
    /// Returns the formatted date and time as a String.
    fn get_x_date(&self) -> String {
        Utc::now().format("%Y%m%dT%H%M%SZ").to_string()
    }
}
