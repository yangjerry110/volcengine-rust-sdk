/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-28 16:45:54
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 16:59:31
 * @Description: Service for ECS (Elastic Compute Service)
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

/// EcsService implementation for Ecs
/// Implementation of the `EcsService` trait for the `Ecs` struct.
/// This implementation provides concrete methods for interacting with ECS services.
impl EcsService for Ecs {
    /// Creates a new `Ecs` instance using the provided session.
    /// This method is the primary way to initialize the ECS client.
    ///
    /// # Parameters:
    /// - `session`: A `session::Session` object containing authentication and configuration information.
    ///
    /// # Returns:
    /// A `Result` containing an `Ecs` instance on success or an `error::Error` on failure.
    fn new_ecs(session: session::Session) -> Result<Self, error::Error> {
        // Create a new session and retrieve the client configuration
        let client_config = session.new_client_config(client_config::ClientServiceName::Ecs);

        // Set up client information
        let client_info = client_info::ClientInfo::builder()
            .with_service_name(client_config::ClientServiceName::Ecs)
            .with_api_version(common::COMMON_VERSION_2020_04_01)
            .with_signing_region(&client_config.signing_region)
            .build()?;

        // Create handles
        let request_handles = handles::Handles {};

        // Build the client with the provided information and configuration
        let client = client::Client::builder()
            .with_client_info(&client_info)
            .with_config(&client_config)
            .with_handles(&request_handles)
            .build()?;

        // Return the new Ecs instance
        Ok(Ecs { client: client })
    }

    /// Initiates a request to run instances in ECS.
    /// This method uses the internal `ApiRunInstance` to handle the request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `Ecs`.
    /// - `request`: A `RunInstancesReq` object containing the parameters for running instances.
    ///
    /// # Returns:
    /// A `Result` containing a `RunInstancesResp` on success or an `error::Error` on failure.
    async fn new_run_instances(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesResp, error::Error>
    {
        api_run_instances::ApiRunInstance
            .new_run_instances(self, request)
            .await
    }

    /// Initiates a request to describe instances in ECS.
    /// This method uses the internal `ApiDescribeInstances` to handle the request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `Ecs`.
    /// - `request`: A `DescribeInstancesReq` object containing the parameters for describing instances.
    ///
    /// # Returns:
    /// A `Result` containing a `DescribeInstancesResp` on success or an `error::Error` on failure.
    async fn new_describe_instances(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_instance::DescribeInstancesReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_instance::DescribeInstancesResp, error::Error>
    {
        api_describe_instances::ApiDescribeInstances
            .new_describe_instances(self, request)
            .await
    }

    /// Initiates a request to stop a single instance in ECS.
    /// This method uses the internal `ApiStopInstance` to handle the request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `Ecs`.
    /// - `request`: A `StopInstanceReq` object containing the parameters for stopping an instance.
    ///
    /// # Returns:
    /// A `Result` containing a `StopInstanceResp` on success or an `error::Error` on failure.
    async fn new_stop_instance(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_instance::StopInstanceReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_instance::StopInstanceResp, error::Error>
    {
        api_stop_instance::ApiStopInstance
            .new_stop_instance(self, request)
            .await
    }

    /// Initiates a request to stop multiple instances in ECS.
    /// This method uses the internal `ApiStopInstances` to handle the request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `Ecs`.
    /// - `request`: A `StopInstancesReq` object containing the parameters for stopping multiple instances.
    ///
    /// # Returns:
    /// A `Result` containing a `StopInstancesResp` on success or an `error::Error` on failure.
    async fn new_stop_instances(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_instance::StopInstancesReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_instance::StopInstancesResp, error::Error>
    {
        api_stop_instances::ApiStopInstances
            .new_stop_instances(self, request)
            .await
    }

    /// Initiates a request to modify the specifications of an instance in ECS.
    /// This method uses the internal `ApiModifyInstanceSpec` to handle the request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `Ecs`.
    /// - `request`: A `ModifyInstanceSpecReq` object containing the parameters for modifying instance specifications.
    ///
    /// # Returns:
    /// A `Result` containing a `ModifyInstanceSpecResp` on success or an `error::Error` on failure.
    async fn new_modify_instance_spec(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_instance::ModifyInstanceSpecReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_instance::ModifyInstanceSpecResp, error::Error>
    {
        api_modify_instance_spec::ApiModifyInstanceSpec
            .new_modify_instance_spec(self, request)
            .await
    }

    /// Initiates a request to describe images in ECS.
    /// This method uses the internal `ApiDescribeImagesEcs` to handle the request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `Ecs`.
    /// - `request`: A `DescribeImagesReq` object containing the parameters for describing images.
    ///
    /// # Returns:
    /// A `Result` containing a `DescribeImagesResp` on success or an `error::Error` on failure.
    async fn new_describe_images(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_image::DescribeImagesReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_image::DescribeImagesResp, error::Error>
    {
        api_describe_images::ApiDescribeImagesEcs
            .new_describe_images(self, request)
            .await
    }

    /// Initiates a request to describe regions in ECS.
    /// This method uses the internal `ApiDescribeRegionsEcs` to handle the request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `Ecs`.
    /// - `request`: A `DescribeRegionsReq` object containing the parameters for describing regions.
    ///
    /// # Returns:
    /// A `Result` containing a `DescribeRegionsResp` on success or an `error::Error` on failure.
    async fn new_describe_regions(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeRegionsReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeRegionsResp, error::Error>
    {
        api_describe_regions::ApiDescribeRegionsEcs
            .new_describe_regions(self, request)
            .await
    }

    /// Initiates a request to describe zones in ECS.
    /// This method uses the internal `ApiDescribeZonesEcs` to handle the request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `Ecs`.
    /// - `request`: A `DescribeZonesReq` object containing the parameters for describing zones.
    ///
    /// # Returns:
    /// A `Result` containing a `DescribeZonesResp` on success or an `error::Error` on failure.
    async fn new_describe_zones(
        &self,
        request: volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeZonesReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::ecs_zone::DescribeZonesResp, error::Error> {
        api_describe_zones::ApiDescribeZonesEcs
            .new_describe_zones(self, request)
            .await
    }
}
