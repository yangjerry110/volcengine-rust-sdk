use crate::service::redis::{Redis,RedisService};
use crate::volcengine::error::error;
use crate::volcengine::session::session;
use crate::volcengine::client::config as client_config;
use crate::volcengine::client::client_info;
use crate::volcengine::common;
use crate::volcengine::request::handles;
use crate::volcengine::client::client;

use super::api_craete_db_instance;

/**
 * @description: impl RedisService for Redis
 * @author: Jerry.Yang
 * @date: 2024-11-07 10:46:07
 * @return {*}
 */
impl RedisService for Redis {

    /// new redis
    /// 
    /// # 参数
    /// - `&self`
    /// - `session` : session::Session
    /// 
    /// # 返回
    /// 成功返回 Redis, 错误返回 error::Error
    fn new_redis(&self,session : session::Session) -> Result<Self,error::Error> {
        // new session
        // get client config::config
        let client_config = session.new_client_config(client_config::ClientServiceName::Redis);

        // set client client_info
        // define client client_info
        let client_info = client_info::ClientInfo::builder()
            .with_service_name(client_config::ClientServiceName::Redis)
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
        Ok(Redis { client: client })
    }

    /// new create db instance
    /// 
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::redis_instance::CreateDbInstanceReq
    /// 
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::redis_instance::CreateDbInstanceResp, 错误返回 error::Error
    async fn new_create_db_instance(&self,request : volcengine_sdk_protobuf::protobuf::redis_instance::CreateDbInstanceReq) -> Result<volcengine_sdk_protobuf::protobuf::redis_instance::CreateDbInstanceResp,error::Error> {
        api_craete_db_instance::ApiCreateDBInstanceRedis.new_create_db_instance_api(self, request).await
    }
}