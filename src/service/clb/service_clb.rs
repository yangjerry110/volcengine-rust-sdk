/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-12 17:23:37
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 14:44:03
 * @Description: service clb
 */
use crate::service::clb::api_describe_load_balancers;
use crate::service::clb::Clb;
use crate::service::clb::ServiceClb;
use crate::volcengine::client::client;
use crate::volcengine::client::client_info;
use crate::volcengine::client::config as client_config;
use crate::volcengine::common;
use crate::volcengine::error::error;
use crate::volcengine::request::handles;
use crate::volcengine::session::session;

/**
 * @description: `ServiceClb` trait implementation for `Clb` struct
 *
 * This implementation defines the methods required to create and interact with the CLB service
 * through the `ServiceClb` trait. The trait includes functionality for initializing a new `Clb`
 * instance and sending requests to the CLB service, such as describing load balancers.
 *
 * - `new_clb`: Creates a new instance of `Clb` with a valid client configuration.
 * - `new_describe_load_balancers`: Sends a request to describe load balancers using the API defined in
 *   `api_describe_load_balancers::ApiDescribeLoadBalancersClb`.
 *
 * The `Clb` struct will use the `client::Client`, `client_info::ClientInfo`, and `handles::Handles`
 * to make requests to the API and parse the responses appropriately.
 *
 * @date: 2024-11-13 10:54:42
 * @return: ServiceClb trait methods implementation for Clb
 */
impl ServiceClb for Clb {
    /// `new_clb` initializes a new instance of the `Clb` struct.
    ///
    /// This method creates a `Clb` service by setting up the client configuration,
    /// client information, and the necessary handles for making requests.
    /// The method returns a `Clb` instance on success or an error if the configuration fails.
    ///
    /// # Arguments
    /// - `session`: The session object used to retrieve the necessary client configuration for the CLB service.
    ///
    /// # Returns
    /// - `Ok(Clb)`: A new `Clb` instance if the initialization succeeds.
    /// - `Err(error::Error)`: Returns an error if the configuration or client setup fails.
    fn new_clb(session: session::Session) -> Result<Self, error::Error> {
        // Create a new client configuration from the session.
        let client_config = session.new_client_config(client_config::ClientServiceName::Clb);

        // Construct client info using the client configuration and common API version.
        let client_info = client_info::ClientInfo::builder()
            .with_service_name(client_config::ClientServiceName::Clb)
            .with_api_version(common::COMMON_VERSION)
            .with_signing_region(&client_config.signing_region)
            .build()?;

        // Initialize handles (to be used for request management).
        let request_handles = handles::Handles {};

        // Build the client using the client info, configuration, and request handles.
        let client = client::Client::builder()
            .with_client_info(&client_info)
            .with_config(&client_config)
            .with_handles(&request_handles)
            .build()?;

        // Return the initialized `Clb` instance.
        Ok(Clb { client: client })
    }

    /// `new_describe_load_balancers` sends a request to the CLB service to describe load balancers.
    ///
    /// This method builds the request to the `DescribeLoadBalancers` API, sends the request asynchronously,
    /// and parses the response into a structured format.
    /// It returns the parsed response on success, or an error if the request or response handling fails.
    ///
    /// # Arguments
    /// - `&self`: The reference to the `Clb` instance calling this method.
    /// - `request`: The request struct (`DescribeLoadBalancersReq`) containing parameters for the load balancer query.
    ///
    /// # Returns
    /// - `Ok(DescribeLoadBalancersResp)`: A successful response containing details of the load balancers.
    /// - `Err(error::Error)`: An error if the request fails or the response cannot be parsed.
    async fn new_describe_load_balancers(
        &self,
        request: volcengine_sdk_protobuf::protobuf::lb_instance::DescribeLoadBalancersReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::lb_instance::DescribeLoadBalancersResp,
        error::Error,
    > {
        // Call the `ApiDescribeLoadBalancersClb` API method to handle the request.
        api_describe_load_balancers::ApiDescribeLoadBalancersClb
            .new_describe_load_balancers_api(self, request)
            .await
    }
}
