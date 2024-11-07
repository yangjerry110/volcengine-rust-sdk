/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 10:40:08
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-07 11:04:20
 * @Description: redis 
 */
use std::future::Future;
use crate::volcengine::client::client;
use crate::volcengine::error::error;
use crate::volcengine::session::session;
use volcengine_sdk_protobuf::protobuf::redis_instance;

mod api_create_db_instance_model;
mod api_craete_db_instance;
pub mod service_redis;

/**
 * @description: RedisService
 * @author: Jerry.Yang
 * @date: 2024-11-07 10:42:04
 * @return {*}
 */
pub trait RedisService {
    fn new_redis(&self,session : session::Session) -> Result<Redis,error::Error>;
    fn new_create_db_instance(&self,request : redis_instance::CreateDbInstanceReq) -> impl Future<Output = Result<redis_instance::CreateDbInstanceResp,error::Error>>;
}

/**
 * @description: Redis
 * @author: Jerry.Yang
 * @date: 2024-11-07 10:41:10
 * @return {*}
 */
#[derive(Debug, Clone)]
pub struct Redis {
    client : client::Client
}