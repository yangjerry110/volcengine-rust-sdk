/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 17:23:24
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-05 14:32:34
 * @Description: vpc mod
 */
use crate::volcengine::client::client;
use crate::volcengine::error::error;
use crate::volcengine::session::session;
use std::future::Future;
use volcengine_sdk_protobuf::protobuf::vpc_subnet;
use volcengine_sdk_protobuf::protobuf::vpc_vpc;

mod api_describe_subnets;
mod api_describe_subnets_model;
mod api_describe_vpcs;
mod api_describe_vpcs_model;
pub mod service_vpc;

pub trait VpcService {
    fn new_vpc(session: session::Session) -> Result<Vpc, error::Error>;
    fn new_describe_vpcs(
        &self,
        request: vpc_vpc::DescribeVpcsReq,
    ) -> impl Future<Output = Result<vpc_vpc::DescribeVpcsResp, error::Error>>;
    fn new_describe_subnets(
        &self,
        request: vpc_subnet::DescribeSubnetsReq,
    ) -> impl Future<Output = Result<vpc_subnet::DescribeSubnetsResp, error::Error>>;
}

// 定义 Iam 结构体，用于封装 client 的信息
#[derive(Debug, Clone)]
pub struct Vpc {
    client: client::Client, // 包含 client 信息的实例
}
