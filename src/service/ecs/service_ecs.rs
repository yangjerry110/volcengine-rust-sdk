/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-28 16:45:54
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-06 16:03:38
 * @Description: service ecs
 */
use crate::service::ecs::api_describe_images;
use crate::service::ecs::api_describe_instances;
use crate::service::ecs::api_describe_regions;
use crate::service::ecs::api_describe_zones;
use crate::service::ecs::api_modify_instance_spec;
use crate::service::ecs::api_run_instances;
use crate::service::ecs::api_stop_instance;
use crate::service::ecs::api_stop_instances;
use crate::service::ecs::{Ecs, EcsService};
use crate::volcengine::client::client;
use crate::volcengine::client::client_info;
use crate::volcengine::client::config as client_config;
use crate::volcengine::common;
use crate::volcengine::error::error;
use crate::volcengine::request::handles;
use crate::volcengine::session::session;

impl EcsService for Ecs {
    /// new_ecs
    ///
    /// # 参数
    /// - `session` : session::Session
    ///
    /// # 返回
    /// 成功返回 Ecs, 错误返回 error::Error
    fn new_ecs(session: session::Session) -> Result<Self, error::Error> {
        // new session
        // get client config::config
        let client_config = session.new_client_config(client_config::ClientServiceName::Ecs);

        // set client client_info
        // define client client_info
        let client_info = client_info::ClientInfo::builder()
            .with_service_name(client_config::ClientServiceName::Ecs)
            .with_api_version(common::COMMON_VERSION)
            .with_signing_region(&client_config.signing_region)
            .build()?;

        // handles
        let request_handles = handles::Handles {};

        // define client client
        let client = client::Client::builder()
            .with_client_info(&client_info)
            .with_config(&client_config)
            .with_handles(&request_handles)
            .build()?;

        // return
        Ok(Ecs { client: client })
    }

    /// new run instances api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesResp, 错误返回 error::Error
    async fn new_run_instances(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesResp, error::Error>
    {
        api_run_instances::ApiRunInstance
            .new_run_instances(self, request)
            .await
    }

    /// new describe instances api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::ecs_instance::DescribeInstancesReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::ecs_instance::DescribeInstancesResp, 错误返回 error::Error
    async fn new_describe_instances(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_instance::DescribeInstancesReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_instance::DescribeInstancesResp, error::Error>
    {
        api_describe_instances::ApiDescribeInstances
            .new_describe_instances(self, request)
            .await
    }

    /// new stop instance api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::ecs_instance::StopInstanceReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::ecs_instance::StopInstanceResp, 错误返回 error::Error
    async fn new_stop_instance(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_instance::StopInstanceReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_instance::StopInstanceResp, error::Error>
    {
        api_stop_instance::ApiStopInstance
            .new_stop_instance(self, request)
            .await
    }

    /// new stop instances api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::ecs_instance::StopInstancesReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::ecs_instance::StopInstancesResp, 错误返回 error::Error
    async fn new_stop_instances(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_instance::StopInstancesReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_instance::StopInstancesResp, error::Error>
    {
        api_stop_instances::ApiStopInstances
            .new_stop_instances(self, request)
            .await
    }

    /// new stop instances api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::ecs_instance::ModifyInstanceSpecReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::ecs_instance::ModifyInstanceSpecResp, 错误返回 error::Error
    async fn new_modify_instance_spec(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_instance::ModifyInstanceSpecReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_instance::ModifyInstanceSpecResp, error::Error>
    {
        api_modify_instance_spec::ApiModifyInstanceSpec
            .new_modify_instance_spec(self, request)
            .await
    }

    /// new describe images api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::ecs_image::DescribeImagesReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::ecs_image::DescribeImagesResp, 错误返回 error::Error
    async fn new_describe_images(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_image::DescribeImagesReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_image::DescribeImagesResp, error::Error>
    {
        api_describe_images::ApiDescribeImagesEcs
            .new_describe_images(self, request)
            .await
    }

    /// new describe regions api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeRegionsReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeRegionsResp, 错误返回 error::Error
    async fn new_describe_regions(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeRegionsReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeRegionsResp, error::Error>
    {
        api_describe_regions::ApiDescribeRegionsEcs
            .new_describe_regions(self, request)
            .await
    }

    /// new describe zones api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeZonesReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeZonesResp, 错误返回 error::Error
    async fn new_describe_zones(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeZonesReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeZonesResp, error::Error> {
        api_describe_zones::ApiDescribeZonesEcs
            .new_describe_zones(self, request)
            .await
    }
}
