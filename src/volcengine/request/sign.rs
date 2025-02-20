/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 14:58:16
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:09:32
 * @Description: sign
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request;
use hex::encode;
use hmac::{Hmac, Mac};
use reqwest::RequestBuilder;
use reqwest::Url;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Trait that defines methods for generating HMAC-SHA256 signatures and constructing canonical requests
/// for signing HTTP requests with AWS Signature Version 4 or a similar signing scheme.
pub trait SignRequest {
    /// Generates an HMAC-SHA256 signature for the provided content using the given key.
    ///
    /// # Parameters
    /// - `key`: The key to use for HMAC-SHA256.
    /// - `content`: The content to be signed.
    ///
    /// # Returns
    /// - A `Vec<u8>` containing the result of the HMAC-SHA256 hash.
    fn hmac_sha256(&self, key: &[u8], content: &str) -> Vec<u8>;

    /// Generates the signed key from the secret key, date, region, and service. This signed key is used
    /// to sign the request as part of the AWS Signature Version 4 signing process.
    ///
    /// # Parameters
    /// - `secret_key`: The secret key used for generating the signed key.
    /// - `date`: The date of the request.
    /// - `region`: The region where the service resides.
    /// - `service`: The service for which the request is being made.
    ///
    /// # Returns
    /// - A `Vec<u8>` containing the signed key.
    fn get_signed_key(&self, secret_key: &str, date: &str, region: &str, service: &str) -> Vec<u8>;

    /// Hashes the provided data using SHA-256 and returns the resulting hash.
    ///
    /// # Parameters
    /// - `data`: The data to be hashed.
    ///
    /// # Returns
    /// - A `Vec<u8>` containing the SHA-256 hash of the data.
    fn hash_sha256(&self, data: &[u8]) -> Vec<u8>;

    /// Retrieves the header keys that should be signed from the provided request.
    ///
    /// # Parameters
    /// - `reqwest_request`: The request object from which header keys will be extracted.
    ///
    /// # Returns
    /// - A `Vec<&str>` containing the header keys that should be signed.
    fn get_sign_header_keys(&self, reqwest_request: &reqwest::Request) -> Vec<&str>;

    /// Builds the final signature string for the request, which includes the canonical request and string to sign.
    /// The result is an HMAC-SHA256 signature used to authorize the request.
    ///
    /// # Parameters
    /// - `now_date`: The current date (formatted as "yyyyMMdd").
    /// - `request`: The request object that contains configuration and client info.
    /// - `api_request`: The API request that is being made (generic over `ApiRequest` trait).
    /// - `request_builder`: The builder used to construct the request (used to extract method, headers, etc.).
    ///
    /// # Returns
    /// - A `Result<String, error::Error>` where `Ok(String)` is the final signature, and `Err(error)` indicates failure.
    fn build_signature<T: request::ApiRequest>(
        &self,
        now_date: &str,
        request: &request::Request,
        api_request: &T,
        request_builder: &RequestBuilder,
    ) -> Result<String, error::Error>;

    /// Builds the query string for the request, which is used in the canonical request.
    ///
    /// # Parameters
    /// - `request_builder`: The builder used to construct the request.
    ///
    /// # Returns
    /// - A `Result<String, error::Error>` containing the query string or an error.
    fn build_sign_request_query_str(
        &self,
        request_builder: &RequestBuilder,
    ) -> Result<String, error::Error>;

    /// Builds the string representation of the request headers to be included in the canonical request.
    ///
    /// # Parameters
    /// - `reqwest_request`: The request object from which headers are extracted.
    ///
    /// # Returns
    /// - A `Result<String, error::Error>` containing the string of headers or an error.
    fn build_sign_headers_str(
        &self,
        reqwest_request: &reqwest::Request,
    ) -> Result<String, error::Error>;

    /// Builds the string of header keys to be included in the canonical request for signing.
    ///
    /// # Parameters
    /// - `reqwest_request`: The request object from which header keys are extracted.
    ///
    /// # Returns
    /// - A `Result<String, error::Error>` containing the header keys as a string or an error.
    fn build_sign_header_keys_str(
        &self,
        reqwest_request: &reqwest::Request,
    ) -> Result<String, error::Error>;

    /// Builds the payload string to be included in the canonical request for signing. The payload is typically
    /// the body of the request, hashed in the case of POST requests.
    ///
    /// # Parameters
    /// - `api_request`: The API request object from which the payload is extracted.
    ///
    /// # Returns
    /// - A `Result<String, error::Error>` containing the hashed payload or an error.
    fn build_sign_payload<T: request::ApiRequest>(
        &self,
        api_request: &T,
    ) -> Result<String, error::Error>;
}

/// `Sign` struct is used to handle the signing process for HTTP requests.
/// It includes methods for generating HMAC-SHA256 signatures, building canonical request strings,
/// and constructing signature headers and payloads, commonly used in AWS Signature Version 4
/// or similar signing schemes.
///
/// The `Sign` struct itself is typically used as a utility to perform cryptographic operations
/// and construct signature data for API requests.
#[derive(Debug, Default, Clone)]
pub struct Sign;

/// Implementation of the `SignRequest` trait for the `Sign` struct.
///
/// This block provides the actual logic for each method declared in the `SignRequest` trait,
/// enabling the `Sign` struct to perform various tasks related to signing HTTP requests.
/// The methods defined here handle operations such as:
///
/// - Generating HMAC-SHA256 signatures.
/// - Creating the canonical request string that will be signed.
/// - Building the signature headers for the request.
/// - Computing the payload hash.
/// - Constructing the final signature string for API requests.
///
/// These methods work together to enable secure signing of requests, typically for API interactions
/// where request authentication or authorization is required using signature-based methods (e.g., AWS Signature V4).
impl SignRequest for Sign {
    /// Builds the signature for an API request using HMAC-SHA256.
    ///
    /// This function follows the process of creating a signed request for API authentication
    /// based on the HMAC-SHA256 algorithm, which is commonly used in services like AWS Signature Version 4.
    /// The steps include generating a canonical request, computing a hash of it, constructing the signing string,
    /// and applying HMAC with the signing key to produce the final signature.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `Sign` struct that implements the `SignRequest` trait.
    /// - `now_date`: A string representing the current date in `YYYYMMDD'T'HHMMSS'Z'` format.
    /// - `request`: A reference to the `request::Request` object, which contains client configuration and request-specific information.
    /// - `api_request`: A reference to a trait object (`Box<dyn request::ApiRequest>`), representing the actual API request being made.
    /// - `request_builder`: A reference to the `RequestBuilder` used to build the actual HTTP request.
    ///
    /// # Returns
    /// - `Ok(String)`: The generated signature as a hexadecimal string.
    /// - `Err(error::Error)`: Returns an error if any step in the signing process fails.
    fn build_signature<T: request::ApiRequest>(
        &self,
        now_date: &str,
        request: &request::Request,
        api_request: &T,
        request_builder: &RequestBuilder,
    ) -> Result<String, error::Error> {
        // Clone the request_builder to avoid modifying the original
        let request_builder_clone = request_builder
            .try_clone()
            .ok_or_else(|| error::Error::ErrRequestBuilderIsNone)?;

        // Build the actual request using the cloned builder
        let reqwest_request = request_builder_clone
            .build()
            .map_err(|e| error::Error::ErrRequest(e))?;

        // Retrieve the HTTP method (e.g., GET, POST) for the request
        let method = reqwest_request.method().as_str();

        // Build the query string for the request (used for signing)
        let api_request_query_str = self.build_sign_request_query_str(request_builder)?;

        // Build the string for signing the headers (used for canonical headers)
        let sign_headers_str = self.build_sign_headers_str(&reqwest_request)?;

        // Get the keys of the headers that need to be signed
        let sign_header_keys_str = self.build_sign_header_keys_str(&reqwest_request)?;

        // Build the payload string that needs to be signed
        let sign_payload = self.build_sign_payload(api_request)?;

        // Construct the CanonicalRequest string by concatenating the method, URL, query string, headers, and payload
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method,                // HTTP method (e.g., GET)
            "/",                   // Assuming root URL (this can be customized if needed)
            api_request_query_str, // Query string for the request
            sign_headers_str,      // Headers to be signed
            sign_header_keys_str,  // Header keys to be signed
            sign_payload           // Payload to be signed
        );

        // Compute the SHA256 hash of the canonical request
        let hash_canonical_request = hex::encode(self.hash_sha256(canonical_request.as_bytes()));

        // Construct the credential scope for the signing process (date/region/service/request)
        let sign_credential_scope_date = &now_date[..8]; // Extract date from `now_date`
        let sign_credential_scope = format!(
            "{}/{}/{}/request",
            sign_credential_scope_date,                // Date (YYYYMMDD)
            request.client_info.signing_region,        // Signing region (e.g., "us-east-1")
            request.client_info.service_name.as_str()  // Service name (e.g., "s3")
        );

        // Create the StringToSign, which includes the signing algorithm, timestamp, credential scope, and hashed canonical request
        let sign_string = format!(
            "HMAC-SHA256\n{}\n{}\n{}",
            now_date,               // The current timestamp
            sign_credential_scope,  // Credential scope (date/region/service/request)
            hash_canonical_request  // Hash of the canonical request
        );

        // Extract the date part from `now_date` to use for signing
        let sign_key_date_str = &now_date[..8];

        // Generate the signing key using the secret key, date, region, and service
        let signed_key = self.get_signed_key(
            request.config.config.credentials.secret_access_key.as_str(),
            sign_key_date_str,                           // Date (YYYYMMDD)
            request.client_info.signing_region.as_str(), // Region (e.g., "us-east-1")
            request.client_info.service_name.as_str(),   // Service (e.g., "s3")
        );

        // Generate the signature using HMAC-SHA256 and the signing key
        let signature = encode(self.hmac_sha256(&signed_key, &sign_string));

        // Return the final signature
        Ok(signature)
    }

    /// Builds the payload hash for signing an API request.
    ///
    /// This function generates the hash of the request payload. It first converts the request's body
    /// to a byte slice, hashes it using SHA256, and then encodes the hash in hexadecimal format.
    /// The resulting payload hash is used in the canonical request for signing the API request.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `Sign` struct that implements the `SignRequest` trait.
    /// - `api_request`: A reference to a trait object (`Box<dyn request::ApiRequest>`), representing the actual API request being made.
    ///
    /// # Returns
    /// - `Ok(String)`: The resulting payload hash encoded in hexadecimal format.
    /// - `Err(error::Error)`: Returns an error if any issue occurs during hashing or encoding.
    fn build_sign_payload<T: request::ApiRequest>(
        &self,
        api_request: &T,
    ) -> Result<String, error::Error> {
        // Generate the SHA256 hash of the request body (payload)
        let payload_hash = encode(self.hash_sha256(api_request.to_body().as_ref()));

        // Return the resulting payload hash as a hexadecimal string
        Ok(payload_hash)
    }

    /// Builds the string of signed header keys for the API request.
    ///
    /// This function processes the header keys of the request and converts them into a lowercase,
    /// semicolon-separated string, which is part of the canonical request for signing. This string
    /// will be used to generate the signature for the request.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `Sign` struct that implements the `SignRequest` trait.
    /// - `reqwest_request`: A reference to the `reqwest::Request` object that represents the HTTP request
    ///   to be signed. The headers of this request will be used to build the signed header keys string.
    ///
    /// # Returns
    /// - `Ok(String)`: The signed header keys as a semicolon-separated lowercase string.
    /// - `Err(error::Error)`: Returns an error if any issue occurs during header processing.
    fn build_sign_header_keys_str(
        &self,
        reqwest_request: &reqwest::Request,
    ) -> Result<String, error::Error> {
        // Retrieve the list of header keys to be signed from the request.
        let sign_header_keys = self.get_sign_header_keys(reqwest_request);

        // Create a mutable vector to store the lowercase version of the header keys.
        let mut lower_sign_header_keys: Vec<String> = Vec::new();

        // Iterate through each header key, convert it to a lowercase string, and add it to the list.
        for sign_header_key in sign_header_keys {
            // Convert the header key to a string and then to lowercase.
            let sign_header_key_string = sign_header_key.to_string();
            lower_sign_header_keys.push(sign_header_key_string.to_lowercase());
        }

        // Join the lowercase header keys with a semicolon (;) and return as the final string.
        Ok(lower_sign_header_keys.join(";"))
    }

    /// Builds the string of signed headers for the API request.
    ///
    /// This function processes the headers of the given `reqwest::Request` and generates a string that
    /// represents the headers to be signed. It includes both standard headers and special cases like the `Host` header.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `Sign` struct that implements the `SignRequest` trait.
    /// - `reqwest_request`: A reference to the `reqwest::Request` object that contains the headers to be signed.
    ///   The function retrieves the headers from this request to generate the signed headers string.
    ///
    /// # Returns
    /// - `Ok(String)`: Returns the signed headers string, formatted with each header in lowercase followed by its value, separated by a colon.
    /// - `Err(error::Error)`: Returns an error if any issue occurs during header processing or if required headers are missing.
    fn build_sign_headers_str(
        &self,
        reqwest_request: &reqwest::Request,
    ) -> Result<String, error::Error> {
        // Create a vector to store the header strings that will be signed.
        let mut sign_headers: Vec<String> = Vec::new();

        // Retrieve the list of header keys to be signed from the request.
        let sign_header_keys = self.get_sign_header_keys(reqwest_request);

        // Iterate through each header key to process it.
        for sign_header_key in sign_header_keys {
            // Check if the current header key is "Host".
            if sign_header_key == "Host" {
                // Get the URL of the request.
                let url = reqwest_request.url().as_str();
                let parse_url =
                    Url::parse(url).map_err(|e| error::Error::ErrRequestSignGetHost(e))?;

                // Check if the URL contains a host.
                if parse_url.host_str().is_none() {
                    return Err(error::Error::ErrRequestSignGetHostNone);
                }

                // Retrieve the host string from the URL.
                let host_str = parse_url.host_str().unwrap();

                // Convert the "Host" header key to lowercase and add it to the signed headers.
                let lower_sign_header_key = sign_header_key.to_lowercase();
                sign_headers.push(format!("{}:{}", lower_sign_header_key, host_str));
                continue;
            }

            // For other headers, retrieve their values from the request's headers.
            let header_value = reqwest_request.headers().get(sign_header_key);

            // Check if the header value is missing.
            if header_value.is_none() {
                return Err(error::Error::ErrRequestSignGetHeaderNone(
                    sign_header_key.to_string(),
                ));
            }

            // Unwrap the header value and convert it to a string.
            let header_value = header_value.unwrap();
            let header_value_str = header_value
                .to_str()
                .map_err(|e| error::Error::ErrRequestHeaderIsErr(e))?;

            // Convert the header key to lowercase and add it to the signed headers string.
            let lower_sign_header_key = sign_header_key.to_lowercase();
            sign_headers.push(format!("{}:{}", lower_sign_header_key, header_value_str));
        }

        // Optionally, add an empty string if the HTTP method is "GET" (commented out).
        // let method = reqwest_request.method().as_str();
        // if method == "GET" {
        //     sign_headers.push(String::from(""));
        // }

        // Append an empty string as per the signing process requirements.
        sign_headers.push(String::from(""));

        // Join the signed headers into a single string, with each header on a new line.
        let sign_headers_str = sign_headers.join("\n");

        // Return the final signed headers string.
        Ok(sign_headers_str)
    }

    /// Builds the query string for signing the request.
    ///
    /// This function processes the query parameters of the provided `request_builder`, extracts them,
    /// encodes the values, and constructs a query string that can be used in signing the request.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `Sign` struct that implements the `SignRequest` trait.
    /// - `request_builder`: A reference to the `RequestBuilder` object, which is used to construct the request.
    ///   It contains the URL and query parameters that will be used for signing.
    ///
    /// # Returns
    /// - `Ok(String)`: Returns the formatted query string for signing the request. This string consists of
    ///   key-value pairs from the URL query parameters, encoded appropriately.
    /// - `Err(error::Error)`: Returns an error if there are any issues during the request building or query extraction.
    fn build_sign_request_query_str(
        &self,
        request_builder: &RequestBuilder,
    ) -> Result<String, error::Error> {
        // Clone the `request_builder` to allow mutation and prevent borrowing issues.
        let request_builder_clone = request_builder
            .try_clone()
            .ok_or_else(|| error::Error::ErrRequestBuilderIsNone)?;

        // Build the final request from the cloned `request_builder`.
        let reqwest_request = request_builder_clone
            .build()
            .map_err(|e| error::Error::ErrRequest(e))?;

        // Retrieve the URL of the request.
        let url = reqwest_request.url().as_str();

        // Parse the URL into a `Url` object to easily extract the query parameters.
        let url_parse = Url::parse(url).map_err(|e| error::Error::ErrParse(e))?;

        // Extract the query parameters from the URL and convert them into a HashMap.
        // The `query_pairs()` function returns an iterator of key-value pairs.
        let query_pairs = url_parse.query_pairs();
        let request_hashmap: BTreeMap<String, String> = query_pairs.into_owned().collect();

        // Create a vector to hold the query parameter strings in the format "key=value".
        let mut request_vec = Vec::new();
        for (key, value) in request_hashmap.iter() {
            // Encode the value of each query parameter using `urlencoding` to ensure special characters
            // are safely included in the query string.
            request_vec.push(format!("{}={}", key, urlencoding::encode(value)));
        }

        // Join the elements of the `request_vec` into a single query string with "&" separating each key-value pair.
        let request_query_str = request_vec.join("&");

        // Return the generated query string.
        Ok(request_query_str)
    }

    /// Retrieves the headers that need to be signed based on the HTTP request method.
    ///
    /// This function returns the list of HTTP headers that should be included in the signing process.
    /// The headers to be signed are determined by the HTTP method. For a `POST` request, additional headers
    /// like `"X-Content-Sha256"` are included, while for other methods like `GET`, only `"Host"` and `"X-Date"`
    /// are included in the list.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `Sign` struct that implements this method.
    /// - `reqwest_request`: A reference to the `reqwest::Request` object, which represents the HTTP request
    ///   that will be signed.
    ///
    /// # Returns
    /// - `Vec<&str>`: A vector of string slices representing the header keys that need to be signed.
    ///   The function returns a different set of headers depending on the HTTP method.
    ///   - For `POST` requests: `["Host", "X-Date", "X-Content-Sha256"]`
    ///   - For other methods (like `GET`): `["Host", "X-Date"]`
    fn get_sign_header_keys(&self, reqwest_request: &reqwest::Request) -> Vec<&str> {
        // Get the HTTP method of the request (e.g., "POST", "GET", etc.)
        let method = reqwest_request.method().as_str();

        // For POST requests, return a list of headers that include "X-Content-Sha256"
        if method == "POST" {
            let result = vec!["Host", "X-Date", "X-Content-Sha256"];
            return result;
        }

        // For other HTTP methods (such as GET), only return "Host" and "X-Date"
        vec!["Host", "X-Date"]
    }

    /// Computes the HMAC-SHA256 hash of the given content using the provided key.
    ///
    /// This function uses the HMAC (Hash-based Message Authentication Code) algorithm with the SHA-256 hash function
    /// to generate a secure hash based on the provided key and content. It is commonly used for signing data
    /// and verifying data integrity in cryptographic applications.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `Sign` struct that implements this method.
    /// - `key`: A byte slice (`&[u8]`) representing the secret key used to generate the HMAC hash.
    ///   This key should be kept secure and private.
    /// - `content`: A string slice (`&str`) representing the data or message that will be hashed.
    ///   This is the content that will be authenticated by the HMAC algorithm.
    ///
    /// # Returns
    /// - `Vec<u8>`: A vector of bytes (`Vec<u8>`) containing the HMAC-SHA256 hash of the content.
    ///   This is the result of applying the HMAC-SHA256 algorithm to the `content` using the `key`.
    ///
    /// # Example
    /// ```rust
    /// let key = b"secret_key";
    /// let content = "message_to_sign";
    /// let hmac = sign.hmac_sha256(key, content);
    /// ```
    /// This will return the HMAC-SHA256 hash of the message `"message_to_sign"` using the key `"secret_key"`.
    fn hmac_sha256(&self, key: &[u8], content: &str) -> Vec<u8> {
        // Create a new HMAC-SHA256 instance using the provided key
        let mut mac = Hmac::<Sha256>::new_from_slice(key).unwrap();

        // Update the HMAC instance with the content (converted to bytes)
        mac.update(content.as_bytes());

        // Finalize the HMAC calculation and convert the resulting hash to a Vec<u8>
        mac.finalize().into_bytes().to_vec()
    }

    /// Generates a signed key using HMAC-SHA256 based on the provided parameters.
    ///
    /// This function calculates a series of HMAC-SHA256 hashes to generate a "signed key" based on the provided
    /// secret key, date, region, and service. This process is commonly used in authentication schemes like
    /// AWS Signature Version 4, where multiple HMAC operations are applied to the secret key to derive
    /// keys for different steps of the signing process.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `Sign` struct that implements this method.
    /// - `secret_key`: A string slice (`&str`) representing the secret key used for signing.
    ///   This key is typically kept secret and is used to authenticate requests.
    /// - `date`: A string slice (`&str`) representing the date, used as part of the signing process.
    ///   Typically formatted as `yyyyMMdd` (e.g., `"20250205"`).
    /// - `region`: A string slice (`&str`) representing the region (e.g., `"us-east-1"`) where the service is hosted.
    /// - `service`: A string slice (`&str`) representing the service name (e.g., `"s3"` or `"iam"`).
    ///
    /// # Returns
    /// - `Vec<u8>`: A vector of bytes (`Vec<u8>`) representing the final signed key. This key is used to
    ///   sign the request or data.
    ///
    /// # Example
    /// ```rust
    /// let signed_key = sign.get_signed_key("secret_key", "20250205", "us-east-1", "s3");
    /// ```
    /// This will return the signed key used for signing requests to the S3 service on the specified date
    /// in the given region.
    fn get_signed_key(&self, secret_key: &str, date: &str, region: &str, service: &str) -> Vec<u8> {
        // Generate the HMAC for the date using the secret key
        let k_date = self.hmac_sha256(secret_key.as_bytes(), date);

        // Generate the HMAC for the region using the HMAC from the previous step
        let k_region = self.hmac_sha256(&k_date, region);

        // Generate the HMAC for the service using the HMAC from the previous step
        let k_service = self.hmac_sha256(&k_region, service);

        // Generate the final signed key by applying HMAC for "request"
        self.hmac_sha256(&k_service, "request")
    }

    /// Computes the SHA256 hash of the provided data.
    ///
    /// This function applies the SHA256 hash function to the given byte slice and returns the resulting
    /// hash as a vector of bytes (`Vec<u8>`). SHA256 is a cryptographic hash function that outputs a 256-bit
    /// hash value, commonly used for data integrity verification and digital signatures.
    ///
    /// # Arguments
    /// - `&self`: A reference to the `Sign` struct that implements this method.
    /// - `data`: A byte slice (`&[u8]`) representing the data to be hashed. This can be any sequence of bytes
    ///   that you want to hash using the SHA256 algorithm.
    ///
    /// # Returns
    /// - `Vec<u8>`: A vector of bytes (`Vec<u8>`) representing the SHA256 hash of the input data.
    ///
    /// # Example
    /// ```rust
    /// let hash = sign.hash_sha256(b"message_to_hash");
    /// ```
    /// This will return the SHA256 hash of the byte slice `b"message_to_hash"`.
    fn hash_sha256(&self, data: &[u8]) -> Vec<u8> {
        // Create a new SHA256 hasher
        let mut hasher = Sha256::new();

        // Update the hasher with the input data
        hasher.update(data);

        // Finalize the hash calculation and return the hash as a vector of bytes
        hasher.finalize().to_vec()
    }
}
