/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 17:33:44
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:33:09
 * @Description: API for describing VPC subnets.
 */
use crate::service::vpc;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::vpc_subnet;

/// A struct representing the API for describing VPC subnets.
/// This struct encapsulates the functionality required to send a request to the Volcengine VPC service
/// to retrieve information about VPC subnets.
pub struct ApiDescribeSubnetsVpc;

/// Implementation of methods for the `ApiDescribeSubnetsVpc` struct.
/// This implementation provides the necessary logic to construct and send a request to the Volcengine VPC service
/// to describe VPC subnets, as well as handle the response.
impl ApiDescribeSubnetsVpc {
    /// Public method to describe VPC subnets.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiDescribeSubnetsVpc`.
    /// - `vpc`: Reference to a `Vpc` instance, which contains client information, configuration, and handles.
    /// - `request`: A `DescribeSubnetsReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<vpc_subnet::DescribeSubnetsResp, error::Error>`: On success, returns a `DescribeSubnetsResp` structure containing the response from the VPC service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    pub async fn new_describe_subnets(
        &self,
        vpc: &vpc::Vpc,
        request: vpc_subnet::DescribeSubnetsReq,
    ) -> Result<vpc_subnet::DescribeSubnetsResp, error::Error> {
        // Delegate the request handling to the private method `new_describe_vpcs_request`.
        self.new_describe_vpcs_request(vpc, request).await
    }

    /// Private method to handle the request to describe VPC subnets.
    ///
    /// This method constructs the request operation, builds the request, sends it to the Volcengine VPC service,
    /// and parses the response.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiDescribeSubnetsVpc`.
    /// - `vpc`: Reference to a `Vpc` instance, which contains client information, configuration, and handles.
    /// - `request`: A `DescribeSubnetsReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<vpc_subnet::DescribeSubnetsResp, error::Error>`: On success, returns a `DescribeSubnetsResp` structure containing the response from the VPC service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_describe_vpcs_request(
        &self,
        vpc: &vpc::Vpc,
        request: vpc_subnet::DescribeSubnetsReq,
    ) -> Result<vpc_subnet::DescribeSubnetsResp, error::Error> {
        // Define the request operation with the specific operation name, HTTP method, and path.
        // The operation name corresponds to the "DescribeSubnets" action in the Volcengine VPC service.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::VpcOperation(
                    operation_config::operation_name_vpc::OperationNameVpc::DescribeSubnets,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?;

        // Build the Volcengine request using the client information, configuration, handles, and the defined operation.
        let response = request::Request::builder()
            .with_client_info(&vpc.client.client_info)
            .with_config(&vpc.client.config)
            .with_handles(&vpc.client.handles)
            .with_operation(&request_operation)
            .build()?
            .send(request)
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp =
            volcengine_sdk_protobuf::protobuf::vpc_subnet::DescribeSubnetsResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
