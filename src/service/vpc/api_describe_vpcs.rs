/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 17:33:44
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:33:42
 * @Description: API for describing VPCs.
 */
use crate::service::vpc;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::vpc_vpc;

/// A struct representing the API for describing VPCs.
/// This struct encapsulates the functionality required to send a request to the Volcengine VPC service
/// to retrieve information about VPCs.
pub struct ApiDescribeVpcsVpc;

/// Implementation of methods for the `ApiDescribeVpcsVpc` struct.
/// This implementation provides the necessary logic to construct and send a request to the Volcengine VPC service
/// to describe VPCs, as well as handle the response.
impl ApiDescribeVpcsVpc {
    /// Public method to describe VPCs.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiDescribeVpcsVpc`.
    /// - `vpc`: Reference to a `Vpc` instance, which contains client information, configuration, and handles.
    /// - `request`: A `DescribeVpcsReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<vpc_vpc::DescribeVpcsResp, error::Error>`: On success, returns a `DescribeVpcsResp` structure containing the response from the VPC service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    pub async fn new_describe_vpcs(
        &self,
        vpc: &vpc::Vpc,
        request: vpc_vpc::DescribeVpcsReq,
    ) -> Result<vpc_vpc::DescribeVpcsResp, error::Error> {
        // Delegate the request handling to the private method `new_describe_vpcs_request`.
        self.new_describe_vpcs_request(vpc, request).await
    }

    /// Private method to handle the request to describe VPCs.
    ///
    /// This method constructs the request operation, builds the request, sends it to the Volcengine VPC service,
    /// and parses the response.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiDescribeVpcsVpc`.
    /// - `vpc`: Reference to a `Vpc` instance, which contains client information, configuration, and handles.
    /// - `request`: A `DescribeVpcsReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<vpc_vpc::DescribeVpcsResp, error::Error>`: On success, returns a `DescribeVpcsResp` structure containing the response from the VPC service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_describe_vpcs_request(
        &self,
        vpc: &vpc::Vpc,
        request: vpc_vpc::DescribeVpcsReq,
    ) -> Result<vpc_vpc::DescribeVpcsResp, error::Error> {
        // Define the request operation with the specific operation name, HTTP method, and path.
        // The operation name corresponds to the "DescribeVpcs" action in the Volcengine VPC service.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::VpcOperation(
                    operation_config::operation_name_vpc::OperationNameVpc::DescribeVpcs,
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
        let mut resp = volcengine_sdk_protobuf::protobuf::vpc_vpc::DescribeVpcsResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
