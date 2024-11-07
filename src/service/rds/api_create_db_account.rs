/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:47:46
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-05 11:35:09
 * @Description: api create db account
 */
use crate::service::rds;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use volcengine_sdk_protobuf::protobuf::rds_account;

/**
 * @description: ApiCreateDbInstanceRds
 * @author: Jerry.Yang
 * @date: 2024-11-05 10:57:32
 * @return {*}
 */
pub struct ApiCreateDbAccountRds;

/**
 * @description: ApiCreateDbInstanceRds
 * @author: Jerry.Yang
 * @date: 2024-11-05 10:57:24
 * @return {*}
 */
impl ApiCreateDbAccountRds {
    /// new_create_db_account
    ///
    /// # 参数
    /// - `&self`
    /// - `rds` : &rds::Rds
    /// - `reqeust` : rds_account::CreateDbAccountReq
    ///
    /// # 返回
    /// 成功返回 rds_account::CreateDbAccountResp, 错误返回 error::Error
    pub async fn new_create_db_account(
        &self,
        rds: &rds::Rds,
        request: rds_account::CreateDbAccountReq,
    ) -> Result<rds_account::CreateDbAccountResp, error::Error> {
        self.new_create_db_account_request(rds, request).await
    }

    /// new_create_db_account_request
    ///
    /// # 参数
    /// - `&self`
    /// - `rds` : &rds::Rds
    /// - `reqeust` : rds_account::CreateDbAccountReq
    ///
    /// # 返回
    /// 成功返回 rds_account::CreateDbAccountResp, 错误返回 error::Error
    async fn new_create_db_account_request(
        &self,
        rds: &rds::Rds,
        request: rds_account::CreateDbAccountReq,
    ) -> Result<rds_account::CreateDbAccountResp, error::Error> {
        // define request operation
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::RdsOperation(
                    operation_config::operation_name_rds::OperationNameRds::CreateDBAccount,
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
            .json::<volcengine_sdk_protobuf::protobuf::rds_account::CreateDbAccountResp>()
            .await
            .map_err(|e| error::Error::ErrParseResponse(e))?;

        // return
        Ok(api_response)
    }
}
