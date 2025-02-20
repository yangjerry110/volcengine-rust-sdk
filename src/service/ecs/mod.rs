/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-28 16:35:18
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 16:56:35
 * @Description: Module for ECS (Elastic Compute Service) operations
 */
use crate::volcengine::client::client;
use crate::volcengine::error::error;
use crate::volcengine::session::session;
use std::future::Future;
use volcengine_sdk_protobuf::protobuf::ecs_image;
use volcengine_sdk_protobuf::protobuf::ecs_instance;
use volcengine_sdk_protobuf::protobuf::ecs_zone;

// Public module for ECS service related operations.
pub mod service_ecs;

// Submodules for various ECS API operations.
// These modules contain the specific implementations for different ECS operations such as describing images, instances, regions, zones, modifying instance specifications, running instances, stopping single and multiple instances.
mod api_describe_images;
mod api_describe_images_models;
mod api_describe_instances;
mod api_describe_instances_model;
mod api_describe_regions;
mod api_describe_regions_model;
mod api_describe_zones;
mod api_describe_zones_models;
mod api_modify_instance_spec;
mod api_modify_instance_spec_model;
mod api_run_instances;
mod api_run_instances_model;
mod api_stop_instance;
mod api_stop_instance_model;
mod api_stop_instances;
mod api_stop_instances_models;

// Test module for ECS operations.
mod tests;

/// The `EcsService` trait defines the interface for interacting with the ECS (Elastic Compute Service).
/// It provides methods for various ECS operations such as running instances, describing instances, stopping instances,
/// modifying instance specifications, and describing images, regions, and zones.
pub trait EcsService {
    /// Creates a new instance of the `Ecs` struct using the provided session.
    ///
    /// # Parameters:
    /// - `session`: A `Session` object containing authentication and configuration information.
    ///
    /// # Returns:
    /// A `Result` containing an `Ecs` instance on success or an `error::Error` on failure.
    fn new_ecs(session: session::Session) -> Result<Ecs, error::Error>;

    /// Initiates a request to run instances in ECS.
    ///
    /// # Parameters:
    /// - `request`: A `RunInstancesReq` object containing the parameters for running instances.
    ///
    /// # Returns:
    /// A `Future` that resolves to a `Result` containing a `RunInstancesResp` on success or an `error::Error` on failure.
    fn new_run_instances(
        &self,
        request: ecs_instance::RunInstancesReq,
    ) -> impl Future<Output = Result<ecs_instance::RunInstancesResp, error::Error>>;

    /// Initiates a request to describe instances in ECS.
    ///
    /// # Parameters:
    /// - `request`: A `DescribeInstancesReq` object containing the parameters for describing instances.
    ///
    /// # Returns:
    /// A `Future` that resolves to a `Result` containing a `DescribeInstancesResp` on success or an `error::Error` on failure.
    fn new_describe_instances(
        &self,
        request: ecs_instance::DescribeInstancesReq,
    ) -> impl Future<Output = Result<ecs_instance::DescribeInstancesResp, error::Error>>;

    /// Initiates a request to stop a single instance in ECS.
    ///
    /// # Parameters:
    /// - `request`: A `StopInstanceReq` object containing the parameters for stopping a single instance.
    ///
    /// # Returns:
    /// A `Future` that resolves to a `Result` containing a `StopInstanceResp` on success or an `error::Error` on failure.
    fn new_stop_instance(
        &self,
        request: ecs_instance::StopInstanceReq,
    ) -> impl Future<Output = Result<ecs_instance::StopInstanceResp, error::Error>>;

    /// Initiates a request to stop multiple instances in ECS.
    ///
    /// # Parameters:
    /// - `request`: A `StopInstancesReq` object containing the parameters for stopping multiple instances.
    ///
    /// # Returns:
    /// A `Future` that resolves to a `Result` containing a `StopInstancesResp` on success or an `error::Error` on failure.
    fn new_stop_instances(
        &self,
        request: ecs_instance::StopInstancesReq,
    ) -> impl Future<Output = Result<ecs_instance::StopInstancesResp, error::Error>>;

    /// Initiates a request to modify the specifications of an instance in ECS.
    ///
    /// # Parameters:
    /// - `request`: A `ModifyInstanceSpecReq` object containing the parameters for modifying instance specifications.
    ///
    /// # Returns:
    /// A `Future` that resolves to a `Result` containing a `ModifyInstanceSpecResp` on success or an `error::Error` on failure.
    fn new_modify_instance_spec(
        &self,
        request: ecs_instance::ModifyInstanceSpecReq,
    ) -> impl Future<Output = Result<ecs_instance::ModifyInstanceSpecResp, error::Error>>;

    /// Initiates a request to describe images in ECS.
    ///
    /// # Parameters:
    /// - `request`: A `DescribeImagesReq` object containing the parameters for describing images.
    ///
    /// # Returns:
    /// A `Future` that resolves to a `Result` containing a `DescribeImagesResp` on success or an `error::Error` on failure.
    fn new_describe_images(
        &self,
        request: ecs_image::DescribeImagesReq,
    ) -> impl Future<Output = Result<ecs_image::DescribeImagesResp, error::Error>>;

    /// Initiates a request to describe regions in ECS.
    ///
    /// # Parameters:
    /// - `request`: A `DescribeRegionsReq` object containing the parameters for describing regions.
    ///
    /// # Returns:
    /// A `Future` that resolves to a `Result` containing a `DescribeRegionsResp` on success or an `error::Error` on failure.
    fn new_describe_regions(
        &self,
        request: ecs_zone::DescribeRegionsReq,
    ) -> impl Future<Output = Result<ecs_zone::DescribeRegionsResp, error::Error>>;

    /// Initiates a request to describe zones in ECS.
    ///
    /// # Parameters:
    /// - `request`: A `DescribeZonesReq` object containing the parameters for describing zones.
    ///
    /// # Returns:
    /// A `Future` that resolves to a `Result` containing a `DescribeZonesResp` on success or an `error::Error` on failure.
    fn new_describe_zones(
        &self,
        request: ecs_zone::DescribeZonesReq,
    ) -> impl Future<Output = Result<ecs_zone::DescribeZonesResp, error::Error>>;
}

/// The `Ecs` struct represents the client for interacting with the ECS (Elastic Compute Service).
/// It encapsulates the client configuration and provides methods for various ECS operations.
#[derive(Debug, Clone)]
pub struct Ecs {
    /// The client configuration used for making requests to the ECS service.
    client: client::Client,
}
