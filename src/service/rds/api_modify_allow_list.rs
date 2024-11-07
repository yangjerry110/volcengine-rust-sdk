use crate::service::rds;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use volcengine_sdk_protobuf::protobuf::rds_allow;

/**
 * @description: ApiModifyAllowListRds
 * @author: Jerry.Yang
 * @date: 2024-11-06 15:58:44
 * @return {*}
 */
pub struct ApiModifyAllowListRds;

/**
 * @description: ApiModifyAllowListRds
 * @author: Jerry.Yang
 * @date: 2024-11-06 15:58:36
 * @return {*}
 */
impl ApiModifyAllowListRds {
    /// new modify allow list api
    ///
    /// # 参数
    /// - `&self`
    /// - `rds` : &rds::Rds
    /// - `request` : rds_allow::ModifyAllowListReq
    ///
    /// # 返回
    /// 成功返回 rds_allow::ModifyAllowListResp, 错误返回 error::Error
    pub async fn new_modify_allow_list(
        &self,
        rds: &rds::Rds,
        request: rds_allow::ModifyAllowListReq,
    ) -> Result<rds_allow::ModifyAllowListResp, error::Error> {
        self.new_modify_allow_list_request(rds, request).await
    }

    /// new modify allow list api request
    ///
    /// # 参数
    /// - `&self`
    /// - `rds` : &rds::Rds
    /// - `request` : rds_allow::ModifyAllowListReq
    ///
    /// # 返回
    /// 成功返回 rds_allow::ModifyAllowListResp, 错误返回 error::Error
    async fn new_modify_allow_list_request(
        &self,
        rds: &rds::Rds,
        request: rds_allow::ModifyAllowListReq,
    ) -> Result<rds_allow::ModifyAllowListResp, error::Error> {
        // define request operation
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::RdsOperation(
                    operation_config::operation_name_rds::OperationNameRds::ModifyAllowList,
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
            .json::<volcengine_sdk_protobuf::protobuf::rds_allow::ModifyAllowListResp>()
            .await
            .map_err(|e| error::Error::ErrParseResponse(e))?;

        // return
        Ok(api_response)
    }
}
