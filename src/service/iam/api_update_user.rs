/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 14:55:04
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-30 11:27:33
 * @Description:
 */
use crate::service::iam::Iam;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::{error::error, request::response::ApiResponse};
use volcengine_sdk_protobuf::protobuf::iam_user;

/**
 * @description: ApiUpdateUserIam
 * @author: Jerry.Yang
 * @date: 2024-10-25 15:01:29
 * @return {*}
 */
pub struct ApiUpdateUserIam;

impl ApiUpdateUserIam {
    /// new_update_user
    ///
    /// # 参数
    /// - `&self`
    /// - `iam` : &Iam
    /// - `request` : iam_user::UpdateUserReq
    ///
    /// # 参数
    /// 成功返回 iam_user::UpdateUserResp, 错误返回 error::Error
    pub async fn new_update_user(
        &self,
        iam: &Iam,
        request: iam_user::UpdateUserReq,
    ) -> Result<iam_user::UpdateUserResp, error::Error> {
        self.new_update_user_request(iam, request).await
    }

    /// new_update_user_request
    ///
    /// # 参数
    /// - `&self`
    /// - `iam` : &Iam
    /// - `request` : iam_user::UpdateUserReq
    ///
    /// # 参数
    /// 成功返回 iam_user::UpdateUserResp, 错误返回 error::Error
    async fn new_update_user_request(
        &self,
        iam: &Iam,
        request: iam_user::UpdateUserReq,
    ) -> Result<iam_user::UpdateUserResp, error::Error> {
        // define request operation
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::IamOperation(
                    operation_config::operation_name_iam::OperationNameIam::UpdateUser,
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
            volcengine_sdk_protobuf::protobuf::iam_user::UpdateUserResp::default();
        api_response.to_struct(response).await?;

        // let api_response = volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp::default();

        // return
        Ok(api_response)
    }
}
