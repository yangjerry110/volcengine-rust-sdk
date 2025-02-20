/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 17:29:44
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 14:38:16
 * @Description: Implementation of the VPC service, providing methods for managing VPCs and subnets.
 */
use super::{Vpc, VpcService};
use crate::service::vpc::api_describe_subnets;
use crate::service::vpc::api_describe_vpcs;
use crate::volcengine::client::client;
use crate::volcengine::client::client_info;
use crate::volcengine::client::config as client_config;
use crate::volcengine::common;
use crate::volcengine::error::error;
use crate::volcengine::request::handles;
use crate::volcengine::session::session;

/// Implementation of the VpcService trait for the Vpc struct.
/// This implementation provides the necessary logic to interact with the Volcengine VPC service,
/// including creating a new VPC service instance, describing VPCs, and describing subnets.
impl VpcService for Vpc {
    /// Creates a new VPC service instance from a given session.
    ///
    /// # Arguments
    /// - `session`: The session object containing the necessary configuration and credentials.
    ///
    /// # Returns
    /// - `Result<Self, error::Error>`: On success, returns a new instance of the Vpc struct.
    ///   On failure, returns an error indicating the cause of the failure.
    fn new_vpc(session: session::Session) -> Result<Self, error::Error> {
        // Create a new client configuration for the VPC service.
        let client_config = session.new_client_config(client_config::ClientServiceName::Vpc);

        // Build the client information with the required parameters.
        let client_info = client_info::ClientInfo::builder()
            .with_service_name(client_config::ClientServiceName::Vpc)
            .with_api_version(common::COMMON_VERSION_2020_04_01)
            .with_signing_region(&client_config.signing_region)
            .build()?;

        // Initialize the request handles.
        let request_handles = handles::Handles {};

        // Build the client with the provided information.
        let client = client::Client::builder()
            .with_client_info(&client_info)
            .with_config(&client_config)
            .with_handles(&request_handles)
            .build()?;

        // Return the new VPC service instance.
        Ok(Vpc { client: client })
    }

    /// Describes VPCs.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current VPC service instance.
    /// - `request`: The request structure containing the parameters for describing VPCs.
    ///
    /// # Returns
    /// - `Result<volcengine_sdk_protobuf::protobuf::vpc_vpc::DescribeVpcsResp, error::Error>`: On success, returns the response from the VPC service.
    ///   On failure, returns an error indicating the cause of the failure.
    async fn new_describe_vpcs(
        &self,
        request: volcengine_sdk_protobuf::protobuf::vpc_vpc::DescribeVpcsReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::vpc_vpc::DescribeVpcsResp, error::Error> {
        api_describe_vpcs::ApiDescribeVpcsVpc
            .new_describe_vpcs(self, request)
            .await
    }

    /// Describes VPC subnets.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current VPC service instance.
    /// - `request`: The request structure containing the parameters for describing VPC subnets.
    ///
    /// # Returns
    /// - `Result<volcengine_sdk_protobuf::protobuf::vpc_subnet::DescribeSubnetsResp, error::Error>`: On success, returns the response from the VPC service.
    ///   On failure, returns an error indicating the cause of the failure.
    async fn new_describe_subnets(
        &self,
        request: volcengine_sdk_protobuf::protobuf::vpc_subnet::DescribeSubnetsReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::vpc_subnet::DescribeSubnetsResp, error::Error>
    {
        api_describe_subnets::ApiDescribeSubnetsVpc
            .new_describe_subnets(self, request)
            .await
    }
}
