use crate::service::iam::Iam;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use volcengine_sdk_protobuf::protobuf::iam_user;

/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 15:13:33
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-30 11:31:07
 * @Description: api delete login profile
 */
pub struct ApiDeleteLoginProfileIam;

impl ApiDeleteLoginProfileIam {
    /// new_get_login_profile
    ///
    /// # 参数
    /// - `&self`
    /// - `iam` : &Iam
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileResp, 错误返回 error::Error
    pub async fn new_delete_login_profile(
        &self,
        iam: &Iam,
        request: iam_user::DeleteLoginProfileReq,
    ) -> Result<iam_user::DeleteLoginProfileResp, error::Error> {
        self.new_delete_login_profile_request(iam, request).await
    }

    /// new_get_login_profile_request
    ///
    /// # 参数
    /// - `&self`
    /// - `iam` : &Iam
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileResp, 错误返回 error::Error
    async fn new_delete_login_profile_request(
        &self,
        iam: &Iam,
        request: iam_user::DeleteLoginProfileReq,
    ) -> Result<iam_user::DeleteLoginProfileResp, error::Error> {
        // define request operation
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::IamOperation(
                    operation_config::operation_name_iam::OperationNameIam::DeleteLoginProfile,
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
            .json::<volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileResp>()
            .await
            .map_err(|e| error::Error::ErrParseResponse(e))?;

        // return
        Ok(api_response)
    }
}
