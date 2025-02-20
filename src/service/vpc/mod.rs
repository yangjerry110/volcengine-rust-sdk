/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 17:23:24
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 14:37:16
 * @Description: Module for managing VPCs and subnets using the Volcengine SDK.
 */
use crate::volcengine::client::client;
use crate::volcengine::error::error;
use crate::volcengine::session::session;
use std::future::Future;
use volcengine_sdk_protobuf::protobuf::vpc_subnet;
use volcengine_sdk_protobuf::protobuf::vpc_vpc;

// Import modules for various VPC operations
mod api_describe_subnets;
mod api_describe_subnets_model;
mod api_describe_vpcs;
mod api_describe_vpcs_model;
pub mod service_vpc;
mod tests;

/// Defines the VpcService trait, providing methods for various VPC operations.
/// This trait encapsulates the functionality required to interact with the Volcengine VPC service.
pub trait VpcService {
    /// Creates a new VPC service instance from a given session.
    ///
    /// # Arguments
    /// - `session`: The session object containing the necessary configuration and credentials.
    ///
    /// # Returns
    /// - `Result<Vpc, error::Error>`: On success, returns a new instance of the Vpc struct.
    ///   On failure, returns an error indicating the cause of the failure.
    fn new_vpc(session: session::Session) -> Result<Vpc, error::Error>;

    /// Describes VPCs.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current VPC service instance.
    /// - `request`: The request structure containing the parameters for describing VPCs.
    ///
    /// # Returns
    /// - `impl Future<Output = Result<vpc_vpc::DescribeVpcsResp, error::Error>>`: On success, returns a future that resolves to the response from the VPC service.
    ///   On failure, returns an error indicating the cause of the failure.
    fn new_describe_vpcs(
        &self,
        request: vpc_vpc::DescribeVpcsReq,
    ) -> impl Future<Output = Result<vpc_vpc::DescribeVpcsResp, error::Error>>;

    /// Describes VPC subnets.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current VPC service instance.
    /// - `request`: The request structure containing the parameters for describing VPC subnets.
    ///
    /// # Returns
    /// - `impl Future<Output = Result<vpc_subnet::DescribeSubnetsResp, error::Error>>`: On success, returns a future that resolves to the response from the VPC service.
    ///   On failure, returns an error indicating the cause of the failure.
    fn new_describe_subnets(
        &self,
        request: vpc_subnet::DescribeSubnetsReq,
    ) -> impl Future<Output = Result<vpc_subnet::DescribeSubnetsResp, error::Error>>;
}

/// Represents the VPC service, encapsulating the client information required to interact with the Volcengine VPC service.
#[derive(Debug, Clone)]
pub struct Vpc {
    /// The client used to make requests to the Volcengine VPC service.
    client: client::Client,
}
