/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-12 17:14:39
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-13 10:52:58
 * @Description: mod
 */
use crate::volcengine::client::client;
use crate::volcengine::error::error;
use crate::volcengine::session::session;
use std::future::Future;
use volcengine_sdk_protobuf::protobuf::lb_instance;

mod api_describe_load_balancers;
mod api_describe_load_balancers_model;
pub mod service_clb;

/**
 * @description: ServiceClb
 * @author: Jerry.Yang
 * @date: 2024-11-13 10:51:58
 * @return {*}
 */
pub trait ServiceClb {
    fn new_clb(session: session::Session) -> Result<Clb, error::Error>;
    fn new_describe_load_balancers(
        &self,
        request: lb_instance::DescribeLoadBalancersReq,
    ) -> impl Future<Output = Result<lb_instance::DescribeLoadBalancersResp, error::Error>>;
}

/**
 * @description: Clb
 * @author: Jerry.Yang
 * @date: 2024-11-13 10:51:51
 * @return {*}
 */
#[derive(Debug, Clone)]
pub struct Clb {
    client: client::Client,
}
