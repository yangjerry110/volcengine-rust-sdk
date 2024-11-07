/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:47:46
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-05 10:57:15
 * @Description: api create db instance
 */
use crate::service::rds;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use volcengine_sdk_protobuf::protobuf::rds_instance;

/**
 * @description: ApiCreateDbInstanceRds
 * @author: Jerry.Yang
 * @date: 2024-11-05 10:57:32
 * @return {*}
 */
pub struct ApiCreateDbInstanceRds;

/**
 * @description: ApiCreateDbInstanceRds
 * @author: Jerry.Yang
 * @date: 2024-11-05 10:57:24
 * @return {*}
 */
impl ApiCreateDbInstanceRds {
    /// new create db instance api
    ///
    /// # 参数
    /// - `&self`
    /// - `rds` : &rds::Rds
    /// - `reqeust` : rds_instance::CreateDbInstanceReq
    ///
    /// # 返回
    /// 成功返回 rds_instance::CreateDbInstanceResp, 错误返回 error::Error
    pub async fn new_create_db_instance(
        &self,
        rds: &rds::Rds,
        request: rds_instance::CreateDbInstanceReq,
    ) -> Result<rds_instance::CreateDbInstanceResp, error::Error> {
        self.new_create_db_instance_request(rds, request).await
    }

    /// new create db instance api request
    ///
    /// # 参数
    /// - `&self`
    /// - `rds` : &rds::Rds
    /// - `reqeust` : rds_instance::CreateDbInstanceReq
    ///
    /// # 返回
    /// 成功返回 rds_instance::CreateDbInstanceResp, 错误返回 error::Error
    async fn new_create_db_instance_request(
        &self,
        rds: &rds::Rds,
        request: rds_instance::CreateDbInstanceReq,
    ) -> Result<rds_instance::CreateDbInstanceResp, error::Error> {
        // define request operation
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::RdsOperation(
                    operation_config::operation_name_rds::OperationNameRds::CreateDBInstance,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::POST,
            )
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?;

        // set request
        // get volcengine_request
        let volcengine_request = request::Request::builder()
            .with_client_info(&rds.client.client_info)
            .with_config(&rds.client.config)
            .with_handles(&rds.client.handles)
            .with_operation(&request_operation)
            .build()?;

        // define request
        // send
        let response = volcengine_request.send(request).await?;

        // 解析响应为 ApiCreateUserResp 结构体
        let api_response = response
            .json::<volcengine_sdk_protobuf::protobuf::rds_instance::CreateDbInstanceResp>()
            .await
            .map_err(|e| error::Error::ErrParseResponse(e))?;

        // return
        Ok(api_response)
    }
}
