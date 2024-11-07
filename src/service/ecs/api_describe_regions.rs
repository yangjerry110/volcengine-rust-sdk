/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 11:08:30
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-30 11:34:31
 * @Description: describe regions
 */
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::{service::ecs, volcengine::error::error};
use volcengine_sdk_protobuf::protobuf::ecs_zone;

pub struct ApiDescribeRegionsEcs;

impl ApiDescribeRegionsEcs {
    /// new describe regions api
    ///
    /// # 参数
    /// - `&self`
    /// - `ecs` : &ecs::Ecs
    /// - `request` : ecs_zone::DescribeRegionsReq
    ///
    /// # 返回
    /// 成功返回 ecs_zone::DescribeRegionsResp, 错误返回 error::Error
    pub async fn new_describe_regions(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_zone::DescribeRegionsReq,
    ) -> Result<ecs_zone::DescribeRegionsResp, error::Error> {
        self.new_describe_regions_request(ecs, request).await
    }

    /// new describe regions api request
    ///
    /// # 参数
    /// - `&self`
    /// - `ecs` : &ecs::Ecs
    /// - `request` : ecs_zone::DescribeRegionsReq
    ///
    /// # 返回
    /// 成功返回 ecs_zone::DescribeRegionsResp, 错误返回 error::Error
    async fn new_describe_regions_request(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_zone::DescribeRegionsReq,
    ) -> Result<ecs_zone::DescribeRegionsResp, error::Error> {
        // define request operation
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::EcsOperation(
                    operation_config::operation_name_ecs::OperationNameEcs::DescribeRegions,
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
            .json::<volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeRegionsResp>()
            .await
            .map_err(|e| error::Error::ErrParseResponse(e))?;

        // return
        Ok(api_response)
    }
}
