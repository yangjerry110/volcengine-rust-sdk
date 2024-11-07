/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 10:52:40
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-30 11:35:11
 * @Description: describe images
 */
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::{service::ecs, volcengine::error::error};
use volcengine_sdk_protobuf::protobuf::ecs_image;

pub struct ApiDescribeImagesEcs;

impl ApiDescribeImagesEcs {
    /// new describe images api
    ///
    /// # 参数
    /// - `&self`
    /// - `ecs` : &ecs::Ecs
    /// - `request` : ecs_image::DescribeImagesReq
    ///
    /// # 返回
    /// 成功返回 ecs_image::DescribeImagesResp, 错误返回 error::Error
    pub async fn new_describe_images(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_image::DescribeImagesReq,
    ) -> Result<ecs_image::DescribeImagesResp, error::Error> {
        self.new_describe_images_request(ecs, request).await
    }

    /// new describe images api request
    ///
    /// # 参数
    /// - `&self`
    /// - `ecs` : &ecs::Ecs
    /// - `request` : ecs_image::DescribeImagesReq
    ///
    /// # 返回
    /// 成功返回 ecs_image::DescribeImagesResp, 错误返回 error::Error
    async fn new_describe_images_request(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_image::DescribeImagesReq,
    ) -> Result<ecs_image::DescribeImagesResp, error::Error> {
        // define request operation
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::EcsOperation(
                    operation_config::operation_name_ecs::OperationNameEcs::DescribeImages,
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
            .with_client_info(&ecs.client.client_info)
            .with_config(&ecs.client.config)
            .with_handles(&ecs.client.handles)
            .with_operation(&request_operation)
            .build()?;

        // define request
        // send
        let response = volcengine_request.send(request).await?;

        // 解析响应为 ApiCreateUserResp 结构体
        let api_response = response
            .json::<volcengine_sdk_protobuf::protobuf::ecs_image::DescribeImagesResp>()
            .await
            .map_err(|e| error::Error::ErrParseResponse(e))?;

        // return
        Ok(api_response)
    }
}
