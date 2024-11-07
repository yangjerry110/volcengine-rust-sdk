/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 17:29:44
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-30 11:39:28
 * @Description: service vpc
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

/**
 * @description: VpcService
 * @author: Jerry.Yang
 * @date: 2024-10-29 17:43:14
 * @return {*}
 */
impl VpcService for Vpc {
    /// new_vpc
    ///
    /// # 参数
    /// - `session`: session::Session
    ///
    /// # 返回
    /// Vpc
    fn new_vpc(session: session::Session) -> Result<Self, error::Error> {
        // new session
        // get client config::config
        let client_config = session.new_client_config(client_config::ClientServiceName::Vpc);

        // set client client_info
        // define client client_info
        let client_info = client_info::ClientInfo::builder()
            .with_service_name(client_config::ClientServiceName::Vpc)
            .with_api_version(common::COMMON_VERSION)
            .with_signing_region(&client_config.signing_region)
            .build()?;

        // handles
        let request_handles = handles::Handles {};

        // define client client
        let client = client::Client::builder()
            .with_client_info(&client_info)
            .with_config(&client_config)
            .with_handles(&request_handles)
            .build()?;

        // return
        Ok(Vpc { client: client })
    }

    /// new describe vpcs api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::vpc_vpc::DescribeVpcsReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::vpc_vpc::DescribeVpcsResp, 错误返回 error::Error
    async fn new_describe_vpcs(
        &self,
        request: volcengine_sdk_protobuf::protobuf::vpc_vpc::DescribeVpcsReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::vpc_vpc::DescribeVpcsResp, error::Error> {
        api_describe_vpcs::ApiDescribeVpcsVpc
            .new_describe_vpcs(self, request)
            .await
    }

    /// new describe subnets api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::vpc_subnet::DescribeSubnetsReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::vpc_subnet::DescribeSubnetsResp, 错误返回 error::Error
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
