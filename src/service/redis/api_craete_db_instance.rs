/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 10:47:33
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-07 11:02:54
 * @Description: api create DB instance
 */
use volcengine_sdk_protobuf::protobuf::redis_instance;
use crate::volcengine::error::error;
use crate::service::redis;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;

/**
 * @description: ApiCreateDBInstanceRedis
 * @author: Jerry.Yang
 * @date: 2024-11-07 11:01:47
 * @return {*}
 */
pub struct ApiCreateDBInstanceRedis;

/**
 * @description: ApiCreateDBInstanceRedis
 * @author: Jerry.Yang
 * @date: 2024-11-07 11:03:43
 * @return {*}
 */
impl ApiCreateDBInstanceRedis {

    /// new create db instance api 
    /// 
    /// # 参数
    /// - `&self`
    /// - `redis` : &redis::Redis
    /// - `request` : redis_instance::CreateDbInstanceReq
    /// 
    /// # 返回
    /// 成功返回 redis_instance::CreateDbInstanceResp, 错误返回 error::Error
    pub async fn new_create_db_instance_api(&self,redis : &redis::Redis,request : redis_instance::CreateDbInstanceReq) -> Result<redis_instance::CreateDbInstanceResp,error::Error> {
        self.new_create_db_instance_api_request(redis, request).await
    }

    /// new create db instance api request
    /// 
    /// # 参数
    /// - `&self`
    /// - `redis` : &redis::Redis
    /// - `request` : redis_instance::CreateDbInstanceReq
    /// 
    /// # 返回
    /// 成功返回 redis_instance::CreateDbInstanceResp, 错误返回 error::Error
    async fn new_create_db_instance_api_request(&self,redis : &redis::Redis,request : redis_instance::CreateDbInstanceReq) -> Result<redis_instance::CreateDbInstanceResp,error::Error> {
        // define request operation
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::RedisOperation(
                    operation_config::operation_name_redis::OperationNameRedis::CreateDBInstance,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?;

        // get volcengine_request
        let volcengine_request = request::Request::builder()
            .with_client_info(&redis.client.client_info)
            .with_config(&redis.client.config)
            .with_handles(&redis.client.handles)
            .with_operation(&request_operation)
            .build()?;

        // define request
        // send
        let response = volcengine_request.send(request).await?;

        // println!("response : {:?}", response.text().await);

        // 解析响应为 ApiCreateUserResp 结构体
        let mut api_response =
            volcengine_sdk_protobuf::protobuf::redis_instance::CreateDbInstanceResp::default();
        api_response.to_struct(response).await?;

        // let api_response = volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp::default();

        // return
        Ok(api_response)
}
}