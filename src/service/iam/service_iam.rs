/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 15:59:58
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 19:16:39
 * @Description: IAM (Identity and Access Management) Service
 */
use crate::service::iam::api_attach_user_policy;
use crate::service::iam::api_create_login_profile;
use crate::service::iam::api_create_policy;
use crate::service::iam::api_create_project;
use crate::service::iam::api_create_user;
use crate::service::iam::api_delete_login_profile;
use crate::service::iam::api_delete_policy;
use crate::service::iam::api_delete_user;
use crate::service::iam::api_detach_user_policy;
use crate::service::iam::api_get_login_profile;
use crate::service::iam::api_get_policy;
use crate::service::iam::api_get_project;
use crate::service::iam::api_get_security_config;
use crate::service::iam::api_get_user;
use crate::service::iam::api_list_attach_user_policy;
use crate::service::iam::api_list_policy;
use crate::service::iam::api_set_security_config;
use crate::service::iam::api_update_login_profile;
use crate::service::iam::api_update_policy;
use crate::service::iam::api_update_user;
use crate::service::iam::{Iam, IamService};
use crate::volcengine::client::client;
use crate::volcengine::client::client_info;
use crate::volcengine::client::config as client_config;
use crate::volcengine::common;
use crate::volcengine::error::error;
use crate::volcengine::request::handles;
use crate::volcengine::session::session;

/// Implementation of the `IamService` trait for the `Iam` struct.
/// This implementation provides concrete functionality for all the methods defined in the `IamService` trait.
/// By implementing this trait, the `Iam` struct can be used to interact with the Identity and Access Management (IAM) service,
/// performing operations such as creating users, managing policies, and handling security configurations.
impl IamService for Iam {
    /// new_iam
    ///
    /// This method initializes a new IAM service instance with the provided session.
    /// It sets up client configuration, client info, and handles, and builds a client for communication with the IAM service.
    ///
    /// # Parameters
    /// - `session`: The session that will be used to create the IAM service.
    ///
    /// # Returns
    /// Returns a result containing the new `Iam` service instance or an error.
    fn new_iam(session: session::Session) -> Result<Self, error::Error> {
        // Create new client configuration using the session
        let client_config = session.new_client_config(client_config::ClientServiceName::Iam);

        // Set up the client information, including service name and API version
        let client_info = client_info::ClientInfo::builder()
            .with_service_name(client_config::ClientServiceName::Iam)
            .with_api_version(common::COMMON_VERSION)
            .with_signing_region(&client_config.signing_region)
            .build()?;

        // Create request handles (for managing requests and responses)
        let request_handles = handles::Handles {};

        // Build the client with the necessary configuration and client info
        let client = client::Client::builder()
            .with_client_info(&client_info)
            .with_config(&client_config)
            .with_handles(&request_handles)
            .build()?;

        // Return the newly created IAM service instance
        Ok(Iam { client: client })
    }

    /// new_create_user
    ///
    /// This method creates a new user in the IAM system using the provided request parameters.
    ///
    /// # Parameters
    /// - `request`: The request object containing the details for creating the user.
    ///
    /// # Returns
    /// Returns a result containing the response object for creating the user or an error.
    async fn new_create_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::CreateUserReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::CreateUserResp, error::Error> {
        // Delegate to the API for creating the user
        api_create_user::ApiCreateUserIam
            .new_create_user(self, request)
            .await
    }

    /// new_get_user
    ///
    /// This method retrieves the details of an existing user from the IAM system.
    ///
    /// # Parameters
    /// - `request`: The request object containing the details for retrieving the user.
    ///
    /// # Returns
    /// Returns a result containing the response object with user details or an error.
    async fn new_get_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetUserReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp, error::Error> {
        // Delegate to the API for retrieving the user
        api_get_user::ApiGetUserIam
            .new_get_user(self, request)
            .await
    }

    /// new_update_user
    ///
    /// This method updates an existing user's details in the IAM system.
    ///
    /// # Parameters
    /// - `request`: The request object containing the details for updating the user.
    ///
    /// # Returns
    /// Returns a result containing the response object for updating the user or an error.
    async fn new_update_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::UpdateUserReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::UpdateUserResp, error::Error> {
        // Delegate to the API for updating the user
        api_update_user::ApiUpdateUserIam
            .new_update_user(self, request)
            .await
    }

    /// new_create_login_profile
    ///
    /// This method creates a login profile for an existing user in the IAM system.
    ///
    /// # Parameters
    /// - `request`: The request object containing the details for creating the login profile.
    ///
    /// # Returns
    /// Returns a result containing the response object for creating the login profile or an error.
    async fn new_create_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::CreateLoginProfileReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::CreateLoginProfileResp, error::Error>
    {
        // Delegate to the API for creating the login profile
        api_create_login_profile::ApiCreateLoginProfileIam
            .new_create_login_profile(self, request)
            .await
    }

    /// new_get_login_profile
    ///
    /// This method retrieves the login profile of an existing user from the IAM system.
    ///
    /// # Parameters
    /// - `request`: The request object containing the details for retrieving the login profile.
    ///
    /// # Returns
    /// Returns a result containing the response object with login profile details or an error.
    async fn new_get_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetLoginProfileReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::GetLoginProfileResp, error::Error>
    {
        // Delegate to the API for retrieving the login profile
        api_get_login_profile::ApiGetLoginProfileIam
            .new_get_login_profile(self, request)
            .await
    }

    /// new_update_login_profile
    ///
    /// This method updates an existing user's login profile in the IAM system.
    ///
    /// # Parameters
    /// - `request`: The request object containing the details for updating the login profile.
    ///
    /// # Returns
    /// Returns a result containing the response object for updating the login profile or an error.
    async fn new_update_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::UpdateLoginProfileReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::UpdateLoginProfileResp, error::Error>
    {
        // Delegate to the API for updating the login profile
        api_update_login_profile::ApiUpdateLoginProfileIam
            .new_update_login_profile(self, request)
            .await
    }

    /// new_delete_login_profile
    ///
    /// This method deletes an existing user's login profile from the IAM system.
    ///
    /// # Parameters
    /// - `request`: The request object containing the details for deleting the login profile.
    ///
    /// # Returns
    /// Returns a result containing the response object for deleting the login profile or an error.
    async fn new_delete_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileResp, error::Error>
    {
        // Delegate to the API for deleting the login profile
        api_delete_login_profile::ApiDeleteLoginProfileIam
            .new_delete_login_profile(self, request)
            .await
    }

    /// new_set_security_config
    ///
    /// This method sets the security configuration for a user in the IAM system.
    ///
    /// # Parameters
    /// - `request`: The request object containing the details for setting the security configuration.
    ///
    /// # Returns
    /// Returns a result containing the response object for setting the security configuration or an error.
    async fn new_set_security_config(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::SetSecurityConfigReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::SetSecurityConfigResp, error::Error>
    {
        // Delegate to the API for setting the security configuration
        api_set_security_config::ApiSetSecurityConfigIam
            .new_set_security_config(self, request)
            .await
    }

    /// new_get_security_config
    ///
    /// # Parameters
    /// - `&self`
    /// - `request`: A `GetSecurityConfigReq` structure from `volcengine_sdk_protobuf::protobuf::iam_user`
    ///
    /// # Returns
    /// - On success, returns a `GetSecurityConfigResp` structure from `volcengine_sdk_protobuf::protobuf::iam_user`
    /// - On failure, returns an `error::Error`
    async fn new_get_security_config(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetSecurityConfigReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::GetSecurityConfigResp, error::Error>
    {
        api_get_security_config::ApiGetSecurityConfigIam
            .new_get_security_config(self, request)
            .await
    }

    /// new_create_project
    ///
    /// # Parameters
    /// - `&self`
    /// - `request`: A `CreateProjectReq` structure from `volcengine_sdk_protobuf::protobuf::iam_project`
    ///
    /// # Returns
    /// - On success, returns a `CreateProjectResp` structure from `volcengine_sdk_protobuf::protobuf::iam_project`
    /// - On failure, returns an `error::Error`
    async fn new_create_project(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_project::CreateProjectReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_project::CreateProjectResp, error::Error>
    {
        api_create_project::ApiCreateProjectIam
            .new_create_project(self, request)
            .await
    }

    /// new_get_project
    ///
    /// # Parameters
    /// - `&self`
    /// - `request`: A `GetProjectReq` structure from `volcengine_sdk_protobuf::protobuf::iam_project`
    ///
    /// # Returns
    /// - On success, returns a `GetProjectResp` structure from `volcengine_sdk_protobuf::protobuf::iam_project`
    /// - On failure, returns an `error::Error`
    async fn new_get_project(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_project::GetProjectReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_project::GetProjectResp, error::Error> {
        api_get_project::ApiGetProjectIam
            .new_get_project(self, request)
            .await
    }

    /// new_create_policy
    ///
    /// # Parameters
    /// - `&self`
    /// - `request`: A `CreatePolicyReq` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    ///
    /// # Returns
    /// - On success, returns a `CreatePolicyResp` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    /// - On failure, returns an `error::Error`
    async fn new_create_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::CreatePolicyReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::CreatePolicyResp, error::Error> {
        api_create_policy::ApiCreatePolicyIam
            .new_create_policy(self, request)
            .await
    }

    /// new_get_policy
    ///
    /// # Parameters
    /// - `&self`
    /// - `request`: A `GetPolicyReq` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    ///
    /// # Returns
    /// - On success, returns a `GetPolicyResp` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    /// - On failure, returns an `error::Error`
    async fn new_get_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::GetPolicyReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::GetPolicyResp, error::Error> {
        api_get_policy::ApiGetPolicyIam
            .new_get_policy(self, request)
            .await
    }

    /// new_list_policy
    ///
    /// # Parameters
    /// - `&self`
    /// - `request`: A `ListPoliciesReq` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    ///
    /// # Returns
    /// - On success, returns a `ListPoliciesResp` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    /// - On failure, returns an `error::Error`
    async fn new_list_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::ListPoliciesReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::ListPoliciesResp, error::Error> {
        api_list_policy::ApiListPolicyIam
            .new_list_policy(self, request)
            .await
    }

    /// new_update_policy
    ///
    /// # Parameters
    /// - `&self`
    /// - `request`: A `UpdatePolicyReq` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    ///
    /// # Returns
    /// - On success, returns a `UpdatePolicyResp` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    /// - On failure, returns an `error::Error`
    async fn new_update_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::UpdatePolicyReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::UpdatePolicyResp, error::Error> {
        api_update_policy::ApiUpdatePolicyIam
            .new_update_policy(self, request)
            .await
    }

    /// new_delete_policy
    ///
    /// # Parameters
    /// - `&self`
    /// - `request`: A `DeletePolicyReq` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    ///
    /// # Returns
    /// - On success, returns a `DeletePolicyResp` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    /// - On failure, returns an `error::Error`
    async fn new_delete_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::DeletePolicyReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::DeletePolicyResp, error::Error> {
        api_delete_policy::ApiDeletePolicyIam
            .new_delete_policy(self, request)
            .await
    }

    /// new_attach_user_policy
    ///
    /// # Parameters
    /// - `&self`
    /// - `request`: An `AttachUserPolicyReq` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    ///
    /// # Returns
    /// - On success, returns an `AttachUserPolicyResp` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    /// - On failure, returns an `error::Error`
    async fn new_attach_user_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::AttachUserPolicyReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::AttachUserPolicyResp, error::Error>
    {
        api_attach_user_policy::ApiAttachUserPolicyIam
            .new_attach_user_policy(self, request)
            .await
    }

    /// new_list_attach_user_policy
    ///
    /// # Parameters
    /// - `&self`
    /// - `request`: A `ListAttachedUserPoliciesReq` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    ///
    /// # Returns
    /// - On success, returns a `ListAttachedUserPoliciesResp` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    /// - On failure, returns an `error::Error`
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

    /// new_detach_user_policy
    ///
    /// # Parameters
    /// - `&self`
    /// - `request`: A `DetachUserPolicyReq` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    ///
    /// # Returns
    /// - On success, returns a `DetachUserPolicyResp` structure from `volcengine_sdk_protobuf::protobuf::iam_policy`
    /// - On failure, returns an `error::Error`
    async fn new_detach_user_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::DetachUserPolicyReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_policy::DetachUserPolicyResp, error::Error>
    {
        api_detach_user_policy::ApiDetachUserPolicyIam
            .new_detach_user_policy(self, request)
            .await
    }

    /// new_delete_user
    ///
    /// # Parameters
    /// - `&self`
    /// - `request`: A `DeleteUserReq` structure from `volcengine_sdk_protobuf::protobuf::iam_user`
    ///
    /// # Returns
    /// - On success, returns a `DeleteUserResp` structure from `volcengine_sdk_protobuf::protobuf::iam_user`
    /// - On failure, returns an `error::Error`
    async fn new_delete_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::DeleteUserReq,
    ) -> Result<volcengine_sdk_protobuf::protobuf::iam_user::DeleteUserResp, error::Error> {
        api_delete_user::ApiDeleteUserIam
            .new_delete_user(self, request)
            .await
    }
}
