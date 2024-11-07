/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-06 10:56:34
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-06 11:29:06
 * @Description: create policy
 */
use crate::service::iam;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use volcengine_sdk_protobuf::protobuf::iam_policy;

/**
 * @description: ApiDeletePolicyIam
 * @author: Jerry.Yang
 * @date: 2024-11-06 11:02:06
 * @return {*}
 */
pub struct ApiDeletePolicyIam;

/**
 * @description: ApiDeletePolicyIam
 * @author: Jerry.Yang
 * @date: 2024-11-06 11:01:59
 * @return {*}
 */
impl ApiDeletePolicyIam {
    /// new delete policy api
    ///
    /// # 参数
    /// - `&self`
    /// - `iam` : &iam::Iam
    /// - `request` : iam_policy::DeletePolicyReq
    ///
    /// # 返回
    /// 成功返回 iam_policy::DeletePolicyResp, 错误返回 error::Error
    pub async fn new_delete_policy(
        &self,
        iam: &iam::Iam,
        request: iam_policy::DeletePolicyReq,
    ) -> Result<iam_policy::DeletePolicyResp, error::Error> {
        self.new_delete_policy_request(iam, request).await
    }

    /// new delete policy api request
    ///
    /// # 参数
    /// - `&self`
    /// - `iam` : &iam::Iam
    /// - `request` : iam_policy::DeletePolicyReq
    ///
    /// # 返回
    /// 成功返回 iam_policy::DeletePolicyResp, 错误返回 error::Error
    async fn new_delete_policy_request(
        &self,
        iam: &iam::Iam,
        request: iam_policy::DeletePolicyReq,
    ) -> Result<iam_policy::DeletePolicyResp, error::Error> {
        // define request operation
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::IamOperation(
                    operation_config::operation_name_iam::OperationNameIam::DeletePolicy,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?;

        // set request
        // get volcengine_request
        let volcengine_request = request::Request::builder()
            .with_client_info(&iam.client.client_info)
            .with_config(&iam.client.config)
            .with_handles(&iam.client.handles)
            .with_operation(&request_operation)
            .build()?;

        // define request
        // send
        let response = volcengine_request.send(request).await?;

        // 解析响应为 ApiCreateUserResp 结构体
        let api_response = response
            .json::<volcengine_sdk_protobuf::protobuf::iam_policy::DeletePolicyResp>()
            .await
            .map_err(|e| error::Error::ErrParseResponse(e))?;

        // return
        Ok(api_response)
    }
}
