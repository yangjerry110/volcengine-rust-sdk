/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 15:59:58
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-06 16:03:30
 * @Description: service iam
 */
use crate::service::iam::api_create_login_profile;
use crate::service::iam::api_create_project;
use crate::service::iam::api_create_user;
use crate::service::iam::api_delete_login_profile;
use crate::service::iam::api_get_login_profile;
use crate::service::iam::api_get_project;
use crate::service::iam::api_get_security_config;
use crate::service::iam::api_get_user;
use crate::service::iam::api_set_security_config;
use crate::service::iam::api_update_login_profile;
use crate::service::iam::api_update_user;
use crate::service::iam::{Iam, IamService};
use crate::volcengine::client::client;
use crate::volcengine::client::client_info;
use crate::volcengine::client::config as client_config;
use crate::volcengine::common;
use crate::volcengine::error::error;
use crate::volcengine::request::handles;
use crate::volcengine::session::session;

use super::api_attach_user_policy;
use super::api_create_policy;
use super::api_delete_policy;
use super::api_detach_user_policy;
use super::api_get_policy;
use super::api_list_attach_user_policy;
use super::api_list_policy;
use super::api_update_policy;

impl IamService for Iam {
    /// new_iam
    ///
    /// # 参数
    /// - `session`: session::Session
    ///
    /// # 返回
    /// Iam
    fn new_iam(session: session::Session) -> Result<Self, error::Error> {
        // new session
        // get client config::config
        let client_config = session.new_client_config(client_config::ClientServiceName::Iam);

        // set client client_info
        // define client client_info
        let client_info = client_info::ClientInfo::builder()
            .with_service_name(client_config::ClientServiceName::Iam)
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
        Ok(Iam { client: client })
    }

    /// new_create_user
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_user::CreateUserReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_user::CreateUserResp, 错误返回 error::Error
    async fn new_create_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::CreateUserReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::CreateUserResp, error::Error> {
        api_create_user::ApiCreateUserIam
            .new_create_user(self, request)
            .await
    }

    /// new_get_user
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_user::GetUserReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp, 错误返回 error::Error
    async fn new_get_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetUserReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp, error::Error> {
        api_get_user::ApiGetUserIam
            .new_get_user(self, request)
            .await
    }

    /// new_update_user
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_user::UpdateUserReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_user::UpdateUserResp, 错误返回 error::Error
    async fn new_update_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::UpdateUserReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::UpdateUserResp, error::Error> {
        api_update_user::ApiUpdateUserIam
            .new_update_user(self, request)
            .await
    }

    /// new_create_login_profile
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_user::CreateLoginProfileReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_user::CreateLoginProfileResp, 错误返回 error::Error
    async fn new_create_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::CreateLoginProfileReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::CreateLoginProfileResp, error::Error>
    {
        api_create_login_profile::ApiCreateLoginProfileIam
            .new_create_login_profile(self, request)
            .await
    }

    /// new_get_login_profile
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_user::GetLoginProfileReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_user::GetLoginProfileResp, 错误返回 error::Error
    async fn new_get_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetLoginProfileReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::GetLoginProfileResp, error::Error>
    {
        api_get_login_profile::ApiGetLoginProfileIam
            .new_get_login_profile(self, request)
            .await
    }

    /// new_update_login_profile
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_user::UpdateLoginProfileReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_user::UpdateLoginProfileResp, 错误返回 error::Error
    async fn new_update_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::UpdateLoginProfileReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::UpdateLoginProfileResp, error::Error>
    {
        api_update_login_profile::ApiUpdateLoginProfileIam
            .new_update_login_profile(self, request)
            .await
    }

    /// new_delete_login_profile
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileResp, 错误返回 error::Error
    async fn new_delete_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileResp, error::Error>
    {
        api_delete_login_profile::ApiDeleteLoginProfileIam
            .new_delete_login_profile(self, request)
            .await
    }

    /// new_set_security_config
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_user::SetSecurityConfigReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_user::SetSecurityConfigResp, 错误返回 error::Error
    async fn new_set_security_config(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::SetSecurityConfigReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::SetSecurityConfigResp, error::Error>
    {
        api_set_security_config::ApiSetSecurityConfigIam
            .new_set_security_config(self, request)
            .await
    }

    /// new_get_security_config
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_user::GetSecurityConfigReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_user::GetSecurityConfigResp, 错误返回 error::Error
    async fn new_get_security_config(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetSecurityConfigReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::GetSecurityConfigResp, error::Error>
    {
        api_get_security_config::ApiGetSecurityConfigIam
            .new_get_security_config(self, request)
            .await
    }

    /// new create project api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_project::CreateProjectReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_project::CreateProjectResp, 错误返回 error::Error
    async fn new_create_project(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_project::CreateProjectReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_project::CreateProjectResp, error::Error>
    {
        api_create_project::ApiCreateProjectIam
            .new_create_project(self, request)
            .await
    }

    /// new get project api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_project::GetProjectReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_project::GetProjectResp, 错误返回 error::Error
    async fn new_get_project(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_project::GetProjectReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_project::GetProjectResp, error::Error> {
        api_get_project::ApiGetProjectIam
            .new_get_project(self, request)
            .await
    }

    /// new create policy api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_policy::CreatePolicyReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_policy::CreatePolicyResp, 错误返回 error::Error
    async fn new_create_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::CreatePolicyReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::CreatePolicyResp, error::Error> {
        api_create_policy::ApiCreatePolicyIam
            .new_create_policy(self, request)
            .await
    }

    /// new get policy api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_policy::GetPolicyReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_policy::GetPolicyResp, 错误返回 error::Error
    async fn new_get_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::GetPolicyReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::GetPolicyResp, error::Error> {
        api_get_policy::ApiGetPolicyIam
            .new_get_policy(self, request)
            .await
    }

    /// new list policy api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_policy::ListPoliciesReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_policy::ListPoliciesResp, 错误返回 error::Error
    async fn new_list_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::ListPoliciesReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::ListPoliciesResp, error::Error> {
        api_list_policy::ApiListPolicyIam
            .new_list_policy(self, request)
            .await
    }

    /// new update policy api
    ///
    /// # 参数
    /// - `&self`
    /// - `request`: volcengine_sdk_protobuf::protobuf::iam_policy::UpdatePolicyReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_policy::UpdatePolicyResp, 错误返回 error::Error
    async fn new_update_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::UpdatePolicyReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::UpdatePolicyResp, error::Error> {
        api_update_policy::ApiUpdatePolicyIam
            .new_update_policy(self, request)
            .await
    }

    /// new delete policy api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_policy::DeletePolicyReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_policy::DeletePolicyResp, 错误返回 error::Error
    async fn new_delete_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::DeletePolicyReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::DeletePolicyResp, error::Error> {
        api_delete_policy::ApiDeletePolicyIam
            .new_delete_policy(self, request)
            .await
    }

    /// new attach user policy api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_policy::AttachUserPolicyReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_policy::AttachUserPolicyResp, 错误返回 error::Error
    async fn new_attach_user_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::AttachUserPolicyReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::AttachUserPolicyResp, error::Error>
    {
        api_attach_user_policy::ApiAttachUserPolicyIam
            .new_attach_user_policy(self, request)
            .await
    }

    /// new list attach user policy api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_policy::ListAttachedUserPoliciesReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_policy::ListAttachedUserPoliciesResp, 错误返回 error::Error
    async fn new_list_attach_user_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::ListAttachedUserPoliciesReq,
    ) -> Result<
        volcengine_sdk_protobuf::protobuf::iam_policy::ListAttachedUserPoliciesResp,
        error::Error,
    > {
        api_list_attach_user_policy::ApiListAttachUserPolicyIam
            .new_list_attach_user_policy(self, request)
            .await
    }

    /// new detach user policy api
    ///
    /// # 参数
    /// - `&self`
    /// - `request` : volcengine_sdk_protobuf::protobuf::iam_policy::DetachUserPolicyReq
    ///
    /// # 返回
    /// 成功返回 volcengine_sdk_protobuf::protobuf::iam_policy::DetachUserPolicyResp, 错误返回 error::Error
    async fn new_detach_user_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::DetachUserPolicyReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::DetachUserPolicyResp, error::Error>
    {
        api_detach_user_policy::ApiDetachUserPolicyIam
            .new_detach_user_policy(self, request)
            .await
    }
}
