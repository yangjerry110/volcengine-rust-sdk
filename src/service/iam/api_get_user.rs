/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-22 15:07:51
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-30 11:29:36
 * @Description: get user
 */
use crate::service::iam::Iam;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;

pub struct ApiGetUserIam;

impl ApiGetUserIam {
    /// new_get_user
    ///
    /// # 参数
    /// - `&self`
    /// - `iam` : &Iam
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_user::GetUserReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp, 错误返回 error::Error
    pub async fn new_get_user(
        &self,
        iam: &Iam,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetUserReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp, error::Error> {
        self.new_get_user_request(iam, request).await
    }

    /// new_get_user_request
    ///
    /// # 参数
    /// - `&self`
    /// - `iam` : &Iam
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_user::GetUserReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp, 错误返回 error::Error
    async fn new_get_user_request(
        &self,
        iam: &Iam,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetUserReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp, error::Error> {
        // define request operation
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::IamOperation(
                    operation_config::operation_name_iam::OperationNameIam::GetUser,
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
        let mut api_response = volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp::default();
        api_response.to_struct(response).await?;

        // let api_response = volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp::default();

        // return
        Ok(api_response)
    }
}
