/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 17:33:44
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-30 11:27:05
 * @Description: api describe vpcs
 */
use crate::service::vpc;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::{error::error, request::response::ApiResponse};
use volcengine_sdk_protobuf::protobuf::vpc_subnet;

/**
 * @description: ApiDescribeVpcsVpc
 * @author: Jerry.Yang
 * @date: 2024-10-29 17:40:57
 * @return {*}
 */
pub struct ApiDescribeSubnetsVpc;

/**
 * @description: ApiDescribeSubnetsVpc
 * @author: Jerry.Yang
 * @date: 2024-10-29 17:40:47
 * @return {*}
 */
impl ApiDescribeSubnetsVpc {
    /// new describe subnets api
    ///
    /// # 参数
    /// - `&self`
    /// - `vpc` : &vpc::Vpc
    /// - `request` : vpc_subnet::DescribeSubnetsReq
    ///
    /// # 返回
    /// 成功返回 vpc_subnet::DescribeSubnetsResp, 错误返回 error::Error
    pub async fn new_describe_subnets(
        &self,
        vpc: &vpc::Vpc,
        request: vpc_subnet::DescribeSubnetsReq,
    ) -> Result<vpc_subnet::DescribeSubnetsResp, error::Error> {
        self.new_describe_vpcs_request(vpc, request).await
    }

    /// new describe vpcs api request
    ///
    /// # 参数
    /// - `&self`
    /// - `vpc` : &vpc::Vpc
    /// - `request` : vpc_subnet::DescribeSubnetsReq
    ///
    /// # 返回
    /// 成功返回 vpc_subnet::DescribeSubnetsResp, 错误返回 error::Error
    async fn new_describe_vpcs_request(
        &self,
        vpc: &vpc::Vpc,
        request: vpc_subnet::DescribeSubnetsReq,
    ) -> Result<vpc_subnet::DescribeSubnetsResp, error::Error> {
        // define request operation
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::VpcOperation(
                    operation_config::operation_name_vpc::OperationNameVpc::DescribeSubnets,
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
            .with_client_info(&vpc.client.client_info)
            .with_config(&vpc.client.config)
            .with_handles(&vpc.client.handles)
            .with_operation(&request_operation)
            .build()?;

        // define request
        // send
        let response = volcengine_request.send(request).await?;

        // println!("response : {:?}", response.text().await);

        // 解析响应为 ApiCreateUserResp 结构体
        let mut api_response =
            volcengine_sdk_protobuf::protobuf::vpc_subnet::DescribeSubnetsResp::default();
        api_response.to_struct(response).await?;

        // let api_response = volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp::default();

        // return
        Ok(api_response)
    }
}
