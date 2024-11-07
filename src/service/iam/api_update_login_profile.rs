/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 15:46:30
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-30 11:28:27
 * @Description: api update login profile
 */
use crate::service::iam::Iam;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::{error::error, request::response::ApiResponse};
use volcengine_sdk_protobuf::protobuf::iam_user;

/**
 * @description: ApiUpdateLoginProfileIam
 * @author: Jerry.Yang
 * @date: 2024-10-25 15:01:29
 * @return {*}
 */
pub struct ApiUpdateLoginProfileIam;

impl ApiUpdateLoginProfileIam {
    /// new_update_login_profile
    ///
    /// # 参数
    /// - `&self`
    /// - `iam` : &Iam
    /// - `request` : iam_user::UpdateLoginProfileReq
    ///
    /// # 参数
    /// 成功返回 iam_user::UpdateLoginProfileResp, 错误返回 error::Error
    pub async fn new_update_login_profile(
        &self,
        iam: &Iam,
        request: iam_user::UpdateLoginProfileReq,
    ) -> Result<iam_user::UpdateLoginProfileResp, error::Error> {
        self.new_update_login_profile_request(iam, request).await
    }

    /// new_update_login_profile_request
    ///
    /// # 参数
    /// - `&self`
    /// - `iam` : &Iam
    /// - `request` : iam_user::UpdateLoginProfileReq
    ///
    /// # 参数
    /// 成功返回 iam_user::UpdateLoginProfileResp, 错误返回 error::Error
    async fn new_update_login_profile_request(
        &self,
        iam: &Iam,
        request: iam_user::UpdateLoginProfileReq,
    ) -> Result<iam_user::UpdateLoginProfileResp, error::Error> {
        // define request operation
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::IamOperation(
                    operation_config::operation_name_iam::OperationNameIam::UpdateLoginProfile,
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
            .with_client_info(&iam.client.client_info)
            .with_config(&iam.client.config)
            .with_handles(&iam.client.handles)
            .with_operation(&request_operation)
            .build()?;

        // define request
        // send
        let response = volcengine_request.send(request).await?;

        // println!("response : {:?}", response.text().await);

        // 解析响应为 ApiCreateUserResp 结构体
        let mut api_response =
            volcengine_sdk_protobuf::protobuf::iam_user::UpdateLoginProfileResp::default();
        api_response.to_struct(response).await?;

        // let api_response = volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp::default();

        // return
        Ok(api_response)
    }
}
