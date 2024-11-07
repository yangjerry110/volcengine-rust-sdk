/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:36:29
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-06 15:59:52
 * @Description: service rds
 */
use crate::service::rds::api_create_db_account;
use crate::service::rds::api_create_db_database;
use crate::service::rds::api_create_db_endpoint;
use crate::service::rds::api_create_db_instance;
use crate::service::rds::api_describe_db_instance_detail;
use crate::service::rds::{Rds, RdsService};
use crate::volcengine::client::client;
use crate::volcengine::client::client_info;
use crate::volcengine::client::config as client_config;
use crate::volcengine::common;
use crate::volcengine::error::error;
use crate::volcengine::request::handles;
use crate::volcengine::session::session;

use super::api_modify_allow_list;

impl RdsService for Rds {
    /// new rds
    ///
    /// # 参数
    /// - `session` : session::Session
    ///
    /// # 返回
    /// 成功返回 Self, 错误返回 error::Error
    fn new_rds(session: session::Session) -> Result<Self, error::Error> {
        // new session
        // get client config::config
        let client_config = session.new_client_config(client_config::ClientServiceName::Rds);

        // set client client_info
        // define client client_info
        let client_info = client_info::ClientInfo::builder()
            .with_service_name(client_config::ClientServiceName::Rds)
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
        Ok(Rds { client: client })
    }

    /// new create db instance
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::rds_instance::CreateDbInstanceReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::rds_instance::CreateDbInstanceResp, 错误返回 error::Error
    async fn new_create_db_instance(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_instance::CreateDbInstanceReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::rds_instance::CreateDbInstanceResp, error::Error>
    {
        api_create_db_instance::ApiCreateDbInstanceRds
            .new_create_db_instance(self, request)
            .await
    }

    /// new describe db instance detail
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::rds_instance::DescribeDbInstanceDetailReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::rds_instance::DescribeDbInstanceDetailResp, 错误返回 error::Error
    async fn new_describe_db_instance_detail(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_instance::DescribeDbInstanceDetailReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::rds_instance::DescribeDbInstanceDetailResp,
        error::Error,
    > {
        api_describe_db_instance_detail::ApiDescribeDbInstanceDetailRds
            .new_describe_db_instance_detail(self, request)
            .await
    }

    /// new create db endpoint
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::rds_endpoint::CreateDbEndpointReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::rds_endpoint::CreateDbEndpointResp, 错误返回 error::Error
    async fn new_create_db_endpoint(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_endpoint::CreateDbEndpointReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::rds_endpoint::CreateDbEndpointResp, error::Error>
    {
        api_create_db_endpoint::ApiCreateDbEndpointRds
            .new_create_db_endpoint(self, request)
            .await
    }

    /// new create db account
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::rds_account::CreateDbAccountReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::rds_account::CreateDbAccountResp, 错误返回 error::Error
    async fn new_create_db_account(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_account::CreateDbAccountReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::rds_account::CreateDbAccountResp, error::Error>
    {
        api_create_db_account::ApiCreateDbAccountRds
            .new_create_db_account(self, request)
            .await
    }

    /// new create db database
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::rds_database::CreateDatabaseReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::rds_database::CreateDatabaseResp, 错误返回 error::Error
    async fn new_create_db_database(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_database::CreateDatabaseReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::rds_database::CreateDatabaseResp, error::Error>
    {
        api_create_db_database::ApiCreateDbDatabaseRds
            .new_create_db_database(self, request)
            .await
    }

    /// new modify allow list api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::rds_allow::ModifyAllowListReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::rds_allow::ModifyAllowListResp, 错误返回 error::Error
    async fn new_modify_allow_list(
        &self,
        request: volcengine_sdk_protobuf::protobuf::rds_allow::ModifyAllowListReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::rds_allow::ModifyAllowListResp, error::Error>
    {
        api_modify_allow_list::ApiModifyAllowListRds
            .new_modify_allow_list(self, request)
            .await
    }
}
