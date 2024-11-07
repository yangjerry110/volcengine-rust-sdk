/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-18 11:31:37
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-07 10:39:29
 * @Description: endpoint
 */
#[derive(Debug, Clone)]
pub enum Endpoint {
    IamEndpoint,
    EcsEndpoint,
    VpcEndpoint,
    RdsEndpoint,
    RedisEndpoint,
}

#[derive(Default)]
pub struct ResolvedEndpoint {
    // The endpoint URL
    pub url: String,

    // The region that should be used for signing requests.
    pub signing_region: String,

    // The service name that should be used for signing requests.
    pub signing_name: String,

    // States that the signing name for this endpoint was derived from metadata
    // passed in, but was not explicitly modeled.
    pub signing_name_derived: bool,
    // The signing method that should be used for signing requests.
    // pub signing_method: String,
}

impl Endpoint {
    pub fn get_endpoint(&self) -> &str {
        match self {
            Endpoint::IamEndpoint => "iam.volcengineapi.com",
            Endpoint::EcsEndpoint => "open.volcengineapi.com",
            Endpoint::VpcEndpoint => "open.volcengineapi.com",
            Endpoint::RdsEndpoint => "rds.volcengineapi.com",
            Endpoint::RedisEndpoint => "redis.volcengineapi.com",
        }
    }
}

/// add_scheme 添加http头
///
/// # 参数
/// - `endpoint` : &str
/// - `disable_ssl` : bool
///
/// # 返回
/// 成功返回 String
pub fn add_scheme(endpoint: &str, disable_ssl: bool) -> String {
    // 判断是否以 http || https 开头
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        return endpoint.to_string();
    }

    // 没有http的开头，判断disable_ssl
    if disable_ssl {
        return format!("https://{}", endpoint);
    }

    // http的开头
    format!("http://{}", endpoint)
}
