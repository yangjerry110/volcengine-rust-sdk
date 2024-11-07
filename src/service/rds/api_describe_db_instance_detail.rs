/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:47:46
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-05 11:22:21
 * @Description: api describe db instance detail
 */
use crate::service::rds;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use volcengine_sdk_protobuf::protobuf::rds_instance;

/**
 * @description: ApiDescribeDbInstanceDetailRds
 * @author: Jerry.Yang
 * @date: 2024-11-05 10:57:32
 * @return {*}
 */
pub struct ApiDescribeDbInstanceDetailRds;

/**
 * @description: ApiCreateDbInstanceRds
 * @author: Jerry.Yang
 * @date: 2024-11-05 10:57:24
 * @return {*}
 */
impl ApiDescribeDbInstanceDetailRds {
    /// new_describe_db_instance_detail
    ///
    /// # 参数
    /// - `&self`
    /// - `rds` : &rds::Rds
    /// - `reqeust` : rds_instance::DescribeDbInstanceDetailReq
    ///
    /// # 返回
    /// 成功返回 rds_instance::DescribeDbInstanceDetailResp, 错误返回 error::Error
    pub async fn new_describe_db_instance_detail(
        &self,
        rds: &rds::Rds,
        request: rds_instance::DescribeDbInstanceDetailReq,
    ) -> Result<rds_instance::DescribeDbInstanceDetailResp, error::Error> {
        self.new_describe_db_instance_detail_request(rds, request)
            .await
    }

    /// new create db instance api request
    ///
    /// # 参数
    /// - `&self`
    /// - `rds` : &rds::Rds
    /// - `reqeust` : rds_instance::DescribeDbInstanceDetailReq
    ///
    /// # 返回
    /// 成功返回 rds_instance::DescribeDbInstanceDetailResp, 错误返回 error::Error
    async fn new_describe_db_instance_detail_request(
        &self,
        rds: &rds::Rds,
        request: rds_instance::DescribeDbInstanceDetailReq,
    ) -> Result<rds_instance::DescribeDbInstanceDetailResp, error::Error> {
        // define request operation
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::RdsOperation(
                    operation_config::operation_name_rds::OperationNameRds::DescribeDBInstanceDetail,
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
            .json::<volcengine_sdk_protobuf::protobuf::rds_instance::DescribeDbInstanceDetailResp>()
            .await
            .map_err(|e| error::Error::ErrParseResponse(e))?;

        // return
        Ok(api_response)
    }
}
