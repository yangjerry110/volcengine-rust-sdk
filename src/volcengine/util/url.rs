/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-18 10:49:00
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-07 10:39:55
 * @Description: util url
 */
use crate::volcengine::{client::config, endpoint::endpoint};

#[allow(dead_code)]
pub struct Url {
    pub service_name: config::ClientServiceName,
    pub region: String,
}

impl Url {
    pub fn get_endpoint(&self) -> String {
        // 根据service_name,获取endpoint
        let url_config_endpoint = match self.service_name {
            config::ClientServiceName::Iam => endpoint::Endpoint::IamEndpoint,
            config::ClientServiceName::Ecs => endpoint::Endpoint::EcsEndpoint,
            config::ClientServiceName::Vpc => endpoint::Endpoint::VpcEndpoint,
            config::ClientServiceName::Rds => endpoint::Endpoint::RdsEndpoint,
            config::ClientServiceName::Redis => endpoint::Endpoint::RedisEndpoint,
        };

        // region 相关的，暂时还没这个需求，先空着

        // 返回
        url_config_endpoint.get_endpoint().to_string()
    }
}
