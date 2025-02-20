/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-12 17:16:17
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:26:05
 * @Description: API for describing load balancers.
 */
use crate::service::clb;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::lb_instance;

/// A struct representing the "Describe Load Balancers" API.
/// This struct encapsulates the API logic for describing load balancers.
/// It provides methods to build the API request and send it, as well as handling responses.
///
/// The struct itself does not hold any fields but serves as a container for the API logic.
pub struct ApiDescribeLoadBalancersClb;

/// Implementation of the "Describe Load Balancers" API functionality.
/// This `impl` block provides methods to interact with the "Describe Load Balancers" endpoint of the
/// CLB (Cloud Load Balancer) service. It includes the necessary logic to build and send requests,
/// process responses, and handle errors.
///
/// The `ApiDescribeLoadBalancersClb` struct itself does not hold any data but serves as a container
/// for the functions needed to communicate with the API, providing an abstraction over the raw HTTP requests
/// and the protobuf-based responses.
impl ApiDescribeLoadBalancersClb {
    /// Creates a new "Describe Load Balancers" API request and sends it to the server.
    ///
    /// This method acts as a public API to initiate the request for describing load balancers.
    /// It takes in the necessary `clb` (Cloud Load Balancer) client and the `request` parameters,
    /// and then forwards the call to the internal function to build and send the request.
    ///
    /// # Arguments
    /// - `clb`: A reference to the `clb::Clb` client, which contains the client information, configuration, and necessary handles.
    /// - `request`: The request body of type `DescribeLoadBalancersReq` that holds the parameters for the load balancer query.
    ///
    /// # Returns
    /// - `Ok`: If the request is successful, it returns `DescribeLoadBalancersResp`, which contains the response with the load balancers information.
    /// - `Err`: In case of failure, it returns an error of type `error::Error`.
    pub async fn new_describe_load_balancers_api(
        &self,
        clb: &clb::Clb,
        request: lb_instance::DescribeLoadBalancersReq,
    ) -> Result<lb_instance::DescribeLoadBalancersResp, error::Error> {
        // Calls the internal function to prepare the API request and send it.
        self.new_describe_load_balancers_api_request(clb, request)
            .await
    }

    /// Constructs and sends the "Describe Load Balancers" API request to the server.
    ///
    /// This is the internal method that prepares the API request, configures the necessary
    /// operation, and sends the request to the backend service. The response is then parsed and returned.
    ///
    /// # Arguments
    /// - `clb`: A reference to the `clb::Clb` client, which contains configuration and client information.
    /// - `request`: The `DescribeLoadBalancersReq` request containing the necessary parameters for the load balancer query.
    ///
    /// # Returns
    /// - `Ok`: Returns the parsed response as `DescribeLoadBalancersResp` on success.
    /// - `Err`: Returns an error of type `error::Error` in case of failure during request or response handling.
    async fn new_describe_load_balancers_api_request(
        &self,
        clb: &clb::Clb,
        request: lb_instance::DescribeLoadBalancersReq,
    ) -> Result<lb_instance::DescribeLoadBalancersResp, error::Error> {
        // Define the request operation by configuring the necessary API operation details.
        // This section constructs an `Operation` object, specifying the API endpoint details for querying load balancers.
        let request_operation = operation::Operation::builder()
            // Set the operation name to "DescribeLoadBalancers" which corresponds to the CLB (Cloud Load Balancer) API.
            .with_operation_name(
                operation_config::operation_name::OperationName::ClbOperation(
                    operation_config::operation_name_clb::OperationNameClb::DescribeLoadBalancers,
                ),
            )
            // Specify the HTTP method for this operation. In this case, it's a GET request.
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            // Set the HTTP path for the operation. Here, the default path is used.
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            // Build the operation object, ensuring all configurations are properly set.
            .build()?;

        // Set up the request, configuring necessary client information, configuration, and handles.
        // Build the volcengine request object.
        // This section constructs the request object using the previously defined operation and client configurations.
        let response = request::Request::builder()
            // Include client information from the CLB client.
            .with_client_info(&clb.client.client_info)
            // Include configuration details from the CLB client.
            .with_config(&clb.client.config)
            // Include handles (e.g., for logging, error handling) from the CLB client.
            .with_handles(&clb.client.handles)
            // Attach the previously defined operation to this request.
            .with_operation(&request_operation)
            // Build the request object.
            .build()?
            // Send the request and await the response.
            .send(request)
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp =
            volcengine_sdk_protobuf::protobuf::lb_instance::DescribeLoadBalancersResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
