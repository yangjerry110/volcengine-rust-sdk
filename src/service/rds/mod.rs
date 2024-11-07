/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:36:11
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-06 15:59:35
 * @Description: rds mod
 */
use crate::volcengine::client::client;
use crate::volcengine::error::error;
use crate::volcengine::session::session;
use std::future::Future;
use volcengine_sdk_protobuf::protobuf::{
    rds_account, rds_allow, rds_database, rds_endpoint, rds_instance,
};

mod api_create_db_account;
mod api_create_db_account_model;
mod api_create_db_database;
mod api_create_db_database_model;
mod api_create_db_endpoint;
mod api_create_db_endpoint_model;
mod api_create_db_instance;
mod api_create_db_instance_model;
mod api_describe_db_instance_detail;
mod api_describe_db_instance_detail_model;
mod api_modify_allow_list;
mod api_modify_allow_list_model;
pub mod service_rds;

/**
 * @description: RdsService
 * @author: Jerry.Yang
 * @date: 2024-11-05 14:27:29
 * @return {*}
 */
pub trait RdsService {
    fn new_rds(session: session::Session) -> Result<Rds, error::Error>;
    fn new_create_db_instance(
        &self,
        request: rds_instance::CreateDbInstanceReq,
    ) -> impl Future<Output = Result<rds_instance::CreateDbInstanceResp, error::Error>>;
    fn new_describe_db_instance_detail(
        &self,
        request: rds_instance::DescribeDbInstanceDetailReq,
    ) -> impl Future<Output = Result<rds_instance::DescribeDbInstanceDetailResp, error::Error>>;
    fn new_create_db_endpoint(
        &self,
        request: rds_endpoint::CreateDbEndpointReq,
    ) -> impl Future<Output = Result<rds_endpoint::CreateDbEndpointResp, error::Error>>;
    fn new_create_db_account(
        &self,
        request: rds_account::CreateDbAccountReq,
    ) -> impl Future<Output = Result<rds_account::CreateDbAccountResp, error::Error>>;
    fn new_create_db_database(
        &self,
        request: rds_database::CreateDatabaseReq,
    ) -> impl Future<Output = Result<rds_database::CreateDatabaseResp, error::Error>>;
    fn new_modify_allow_list(
        &self,
        request: rds_allow::ModifyAllowListReq,
    ) -> impl Future<Output = Result<rds_allow::ModifyAllowListResp, error::Error>>;
}

/**
 * @description: Rds
 * @author: Jerry.Yang
 * @date: 2024-11-05 14:27:20
 * @return {*}
 */
#[derive(Debug, Clone)]
pub struct Rds {
    client: client::Client,
}
