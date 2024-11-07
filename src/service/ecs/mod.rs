/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-28 16:35:18
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-29 14:43:42
 * @Description: mod
 */
use crate::volcengine::client::client;
use crate::volcengine::error::error;
use crate::volcengine::session::session;
use std::future::Future;
use volcengine_sdk_protobuf::protobuf::ecs_image;
use volcengine_sdk_protobuf::protobuf::ecs_instance;
use volcengine_sdk_protobuf::protobuf::ecs_zone;

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
pub mod service_ecs;

pub trait EcsService {
    fn new_ecs(session: session::Session) -> Result<Ecs, error::Error>;
    fn new_run_instances(
        &self,
        request: ecs_instance::RunInstancesReq,
    ) -> impl Future<Output = Result<ecs_instance::RunInstancesResp, error::Error>>;
    fn new_describe_instances(
        &self,
        request: ecs_instance::DescribeInstancesReq,
    ) -> impl Future<Output = Result<ecs_instance::DescribeInstancesResp, error::Error>>;
    fn new_stop_instance(
        &self,
        request: ecs_instance::StopInstanceReq,
    ) -> impl Future<Output = Result<ecs_instance::StopInstanceResp, error::Error>>;
    fn new_stop_instances(
        &self,
        request: ecs_instance::StopInstancesReq,
    ) -> impl Future<Output = Result<ecs_instance::StopInstancesResp, error::Error>>;
    fn new_modify_instance_spec(
        &self,
        request: ecs_instance::ModifyInstanceSpecReq,
    ) -> impl Future<Output = Result<ecs_instance::ModifyInstanceSpecResp, error::Error>>;
    fn new_describe_images(
        &self,
        request: ecs_image::DescribeImagesReq,
    ) -> impl Future<Output = Result<ecs_image::DescribeImagesResp, error::Error>>;
    fn new_describe_regions(
        &self,
        request: ecs_zone::DescribeRegionsReq,
    ) -> impl Future<Output = Result<ecs_zone::DescribeRegionsResp, error::Error>>;
    fn new_describe_zones(
        &self,
        request: ecs_zone::DescribeZonesReq,
    ) -> impl Future<Output = Result<ecs_zone::DescribeZonesResp, error::Error>>;
}

#[derive(Debug, Clone)]
pub struct Ecs {
    client: client::Client,
}
