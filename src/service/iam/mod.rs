/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 14:34:10
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-06 14:47:49
 * @Description: IamService 模块及其相关接口定义
 */
use crate::volcengine::client::client;
use crate::volcengine::error::error;
use crate::volcengine::session::session;
use std::future::Future;

// 包含 IAM 服务相关的 API 模型
mod api_attach_user_policy;
mod api_attach_user_policy_model;
mod api_create_login_profile;
mod api_create_login_profile_model;
mod api_create_policy;
mod api_create_policy_model;
mod api_create_project;
mod api_create_project_model;
mod api_create_user;
mod api_create_user_model;
mod api_delete_login_profile;
mod api_delete_login_profile_model;
mod api_delete_policy;
mod api_delete_policy_model;
mod api_detach_user_policy;
mod api_detach_user_policy_model;
mod api_get_login_profile;
mod api_get_login_profile_model;
mod api_get_policy;
mod api_get_policy_model;
mod api_get_project;
mod api_get_project_model;
mod api_get_security_config;
mod api_get_security_config_model;
mod api_get_user;
mod api_get_user_model;
mod api_list_attach_user_policy;
mod api_list_attach_user_policy_model;
mod api_list_policy;
mod api_list_policy_model;
mod api_set_security_config;
mod api_set_security_config_model;
mod api_update_login_profile;
mod api_update_login_profile_model;
mod api_update_policy;
mod api_update_policy_model;
mod api_update_user;
mod api_update_user_model;
pub mod service_iam;
mod test;

// 定义 IamService trait，提供一系列与 IAM 相关的 API 接口
pub trait IamService {
    // 创建新的 IAM 实例
    fn new_iam(session: session::Session) -> Result<Iam, error::Error>;

    // 创建用户的 API 接口，返回异步操作结果
    fn new_create_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::CreateUserReq,
    ) -> impl Future<
        Output = Result<volcengine_sdk_protobuf::protobuf::iam_user::CreateUserResp, error::Error>,
    >;

    // 获取用户信息的 API 接口
    fn new_get_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetUserReq,
    ) -> impl Future<
        Output = Result<volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp, error::Error>,
    >;

    // 更新用户信息的 API 接口
    fn new_update_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::UpdateUserReq,
    ) -> impl Future<
        Output = Result<volcengine_sdk_protobuf::protobuf::iam_user::UpdateUserResp, error::Error>,
    >;

    // 创建登录配置的 API 接口
    fn new_create_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::CreateLoginProfileReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_user::CreateLoginProfileResp,
            error::Error,
        >,
    >;

    // 获取登录配置的 API 接口
    fn new_get_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetLoginProfileReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_user::GetLoginProfileResp,
            error::Error,
        >,
    >;

    // 更新登录配置的 API 接口
    fn new_update_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::UpdateLoginProfileReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_user::UpdateLoginProfileResp,
            error::Error,
        >,
    >;

    // 删除登录配置的 API 接口
    fn new_delete_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileResp,
            error::Error,
        >,
    >;

    // 设置安全配置的 API 接口
    fn new_set_security_config(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::SetSecurityConfigReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_user::SetSecurityConfigResp,
            error::Error,
        >,
    >;

    // 获取安全配置的 API 接口
    fn new_get_security_config(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetSecurityConfigReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_user::GetSecurityConfigResp,
            error::Error,
        >,
    >;

    // 创建项目
    fn new_create_project(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_project::CreateProjectReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_project::CreateProjectResp,
            error::Error,
        >,
    >;

    // 获取项目
    fn new_get_project(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_project::GetProjectReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_project::GetProjectResp,
            error::Error,
        >,
    >;

    // createPolicy
    fn new_create_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::CreatePolicyReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::CreatePolicyResp,
            error::Error,
        >,
    >;

    // GetPolicy
    fn new_get_policy(
        &self,
        reqeust: volcengine_sdk_protobuf::protobuf::iam_policy::GetPolicyReq,
    ) -> impl Future<
        Output = Result<volcengine_sdk_protobuf::protobuf::iam_policy::GetPolicyResp, error::Error>,
    >;

    // list policy
    fn new_list_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::ListPoliciesReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::ListPoliciesResp,
            error::Error,
        >,
    >;

    // update policy
    fn new_update_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::UpdatePolicyReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::UpdatePolicyResp,
            error::Error,
        >,
    >;

    // delete policy
    fn new_delete_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::DeletePolicyReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::DeletePolicyResp,
            error::Error,
        >,
    >;

    // attach user policy
    fn new_attach_user_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::AttachUserPolicyReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::AttachUserPolicyResp,
            error::Error,
        >,
    >;

    // new_list_attach_user_policy_request
    fn new_list_attach_user_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::ListAttachedUserPoliciesReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::ListAttachedUserPoliciesResp,
            error::Error,
        >,
    >;

    // new detach user policy api
    fn new_detach_user_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::DetachUserPolicyReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::DetachUserPolicyResp,
            error::Error,
        >,
    >;
}

/**
 * @description: Iam
 * @author: Jerry.Yang
 * @date: 2024-11-06 11:21:39
 * @return {*}
 */
// 定义 Iam 结构体，用于封装 client 的信息
#[derive(Debug, Clone)]
pub struct Iam {
    client: client::Client, // 包含 client 信息的实例
}
