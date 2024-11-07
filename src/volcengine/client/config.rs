/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-18 10:33:04
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-07 10:39:01
 * @Description: client config
 */
use crate::volcengine::config;
use crate::volcengine::request::handles;

#[derive(Debug, Clone)]
pub struct Config {
    pub config: config::Config,
    pub handles: handles::Handles,
    pub endpoint: String,
    pub signing_region: String,
    pub signing_name: String,
    pub signing_name_derived: bool,
}

#[derive(Debug, Clone)]
pub enum ClientServiceName {
    Iam,
    Ecs,
    Vpc,
    Rds,
    Redis,
}

impl ClientServiceName {
    pub fn as_str(&self) -> &str {
        match self {
            ClientServiceName::Iam => "iam",
            ClientServiceName::Ecs => "ecs",
            ClientServiceName::Vpc => "vpc",
            ClientServiceName::Rds => "rds_mysql",
            ClientServiceName::Redis => "redis",
        }
    }
}
