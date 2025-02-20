/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 14:34:10
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-05 19:02:06
 * @Description: Definition of the IamService module and its related interfaces
 */
use crate::volcengine::client::client;
use crate::volcengine::error::error;
use crate::volcengine::session::session;
use std::future::Future;

// Include API models related to the IAM service
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
mod api_delete_user;
mod api_delete_user_model;
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

// Define the IamService trait, which provides a series of API interfaces related to IAM
pub trait IamService {
    // Create a new IAM instance
    /// Creates a new IAM instance.
    ///
    /// # Arguments
    /// - `session`: A `Session` struct containing session information.
    ///
    /// # Returns
    /// - On success, returns an `Iam` instance.
    /// - On error, returns an `Error` struct indicating the reason for the failure.
    fn new_iam(session: session::Session) -> Result<Iam, error::Error>;

    // API interface for creating a user, returning the result of an asynchronous operation
    /// Initiates an API call to create a new user.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `CreateUserReq` struct containing the details for creating a user.
    ///
    /// # Returns
    /// - A future that resolves to a `Result` containing either a `CreateUserResp` struct on success
    ///   or an `Error` struct on failure.
    fn new_create_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::CreateUserReq,
    ) -> impl Future<
        Output = Result<volcengine_sdk_protobuf::protobuf::iam_user::CreateUserResp, error::Error>,
    >;

    // API interface for getting user information
    /// Initiates an API call to retrieve user information.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `GetUserReq` struct containing the details for getting user information.
    ///
    /// # Returns
    /// - A future that resolves to a `Result` containing either a `GetUserResp` struct on success
    ///   or an `Error` struct on failure.
    fn new_get_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetUserReq,
    ) -> impl Future<
        Output = Result<volcengine_sdk_protobuf::protobuf::iam_user::GetUserResp, error::Error>,
    >;

    // API interface for updating user information
    /// Initiates an asynchronous API call to update user information.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: An `UpdateUserReq` struct that contains the new user information to be updated.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains an `UpdateUserResp` struct
    ///   indicating the result of the update operation. On failure, it contains an `Error` struct
    ///   explaining the reason for the update failure.
    fn new_update_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::UpdateUserReq,
    ) -> impl Future<
        Output = Result<volcengine_sdk_protobuf::protobuf::iam_user::UpdateUserResp, error::Error>,
    >;

    // API interface for creating a login profile
    /// Initiates an asynchronous API call to create a login profile.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `CreateLoginProfileReq` struct that holds the details for creating a login profile.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `CreateLoginProfileResp` struct
    ///   indicating the successful creation of the login profile. On failure, it contains an `Error` struct
    ///   with information about the creation failure.
    fn new_create_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::CreateLoginProfileReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_user::CreateLoginProfileResp,
            error::Error,
        >,
    >;

    // API interface for getting a login profile
    /// Initiates an asynchronous API call to retrieve a login profile.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `GetLoginProfileReq` struct that contains the necessary information to get the login profile.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `GetLoginProfileResp` struct
    ///   with the retrieved login profile information. On failure, it contains an `Error` struct indicating
    ///   the reason for the retrieval failure.
    fn new_get_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetLoginProfileReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_user::GetLoginProfileResp,
            error::Error,
        >,
    >;

    // API interface for updating the login profile
    /// Initiates an asynchronous API call to update the login profile.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: An `UpdateLoginProfileReq` struct that contains the new information for updating the login profile,
    ///              such as new password, password reset requirements, etc.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains an `UpdateLoginProfileResp` struct
    ///   which indicates the result of the update operation. On failure, it contains an `Error` struct
    ///   explaining the reason for the update failure, like an invalid request or server - side error.
    fn new_update_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::UpdateLoginProfileReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_user::UpdateLoginProfileResp,
            error::Error,
        >,
    >;

    // API interface for deleting the login profile
    /// Initiates an asynchronous API call to delete the login profile.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `DeleteLoginProfileReq` struct that holds the necessary information to identify
    ///              the login profile to be deleted, such as the user ID associated with the profile.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `DeleteLoginProfileResp` struct
    ///   indicating the successful deletion of the login profile. On failure, it contains an `Error` struct
    ///   with details about the deletion failure, for example, the profile does not exist or there are
    ///   associated dependencies preventing deletion.
    fn new_delete_login_profile(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_user::DeleteLoginProfileResp,
            error::Error,
        >,
    >;

    // API interface for setting the security configuration
    /// Initiates an asynchronous API call to set the security configuration.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `SetSecurityConfigReq` struct that includes the new security configuration settings,
    ///              such as password complexity rules, multi - factor authentication requirements, etc.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `SetSecurityConfigResp` struct
    ///   indicating that the security configuration has been successfully set. On failure, it contains an `Error` struct
    ///   describing the reason for the configuration - setting failure, like invalid configuration values.
    fn new_set_security_config(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::SetSecurityConfigReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_user::SetSecurityConfigResp,
            error::Error,
        >,
    >;

    // API interface for getting the security configuration
    /// Initiates an asynchronous API call to retrieve the security configuration.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `GetSecurityConfigReq` struct that contains the necessary information to identify
    ///              which security configuration to retrieve, such as the scope or user context.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `GetSecurityConfigResp` struct
    ///   with the retrieved security configuration details, like password policies, access control settings, etc.
    ///   On failure, it contains an `Error` struct indicating the reason for the retrieval failure,
    ///   such as insufficient permissions or a server - side issue.
    fn new_get_security_config(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::GetSecurityConfigReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_user::GetSecurityConfigResp,
            error::Error,
        >,
    >;

    // API interface for creating a project
    /// Initiates an asynchronous API call to create a new project.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `CreateProjectReq` struct that holds the details for creating a project,
    ///              such as the project name, description, and initial settings.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `CreateProjectResp` struct
    ///   indicating the successful creation of the project, which may include the project ID and other metadata.
    ///   On failure, it contains an `Error` struct with information about the creation failure,
    ///   like a duplicate project name or invalid input.
    fn new_create_project(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_project::CreateProjectReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_project::CreateProjectResp,
            error::Error,
        >,
    >;

    // API interface for getting a project
    /// Initiates an asynchronous API call to retrieve a project's information.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `GetProjectReq` struct that contains the necessary information to identify
    ///              the project to retrieve, such as the project ID.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `GetProjectResp` struct
    ///   with the retrieved project details, including its name, description, and current status.
    ///   On failure, it contains an `Error` struct indicating the reason for the retrieval failure,
    ///   such as the project not existing or insufficient permissions.
    fn new_get_project(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_project::GetProjectReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_project::GetProjectResp,
            error::Error,
        >,
    >;

    // API interface for creating a policy
    /// Initiates an asynchronous API call to create a new policy.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `CreatePolicyReq` struct that holds the details for creating a policy,
    ///              such as the policy name, description, and permission rules.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `CreatePolicyResp` struct
    ///   indicating the successful creation of the policy, which may include the policy ID and other metadata.
    ///   On failure, it contains an `Error` struct with information about the creation failure,
    ///   like a duplicate policy name or invalid permission rules.
    fn new_create_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::CreatePolicyReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::CreatePolicyResp,
            error::Error,
        >,
    >;

    // API interface for getting a policy
    /// Initiates an asynchronous API call to retrieve a specific policy.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `reqeust`: Note: There is a possible typo here, it should probably be `request`. A `GetPolicyReq` struct
    ///              that contains the necessary information to identify the policy to retrieve,
    ///              such as the policy ID or policy name.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `GetPolicyResp` struct
    ///   with the detailed information of the requested policy, including its name, description,
    ///   and permission rules. On failure, it contains an `Error` struct indicating the reason for
    ///   the retrieval failure, such as the policy not existing or insufficient permissions.
    fn new_get_policy(
        &self,
        reqeust: volcengine_sdk_protobuf::protobuf::iam_policy::GetPolicyReq,
    ) -> impl Future<
        Output = Result<volcengine_sdk_protobuf::protobuf::iam_policy::GetPolicyResp, error::Error>,
    >;

    // API interface for listing policies
    /// Initiates an asynchronous API call to list existing policies.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `ListPoliciesReq` struct that may contain filtering criteria, such as
    ///              pagination information, policy types, or specific search keywords to narrow down
    ///              the list of policies to be retrieved.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `ListPoliciesResp` struct
    ///   which includes a list of policies that match the specified criteria. Each policy in the list
    ///   may have basic information like policy ID, name, and description. On failure, it contains an `Error`
    ///   struct indicating the reason for the listing failure, such as an invalid filter parameter or
    ///   a server - side error.
    fn new_list_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::ListPoliciesReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::ListPoliciesResp,
            error::Error,
        >,
    >;

    // API interface for updating a policy
    /// Initiates an asynchronous API call to update an existing policy.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: An `UpdatePolicyReq` struct that contains the updated information for the policy,
    ///              such as new policy name, description, or modified permission rules. It also needs
    ///              to have information to identify the policy to be updated, like the policy ID.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains an `UpdatePolicyResp` struct
    ///   indicating the successful update of the policy. On failure, it contains an `Error` struct
    ///   explaining the reason for the update failure, such as the policy not existing, invalid update
    ///   data, or insufficient permissions.
    fn new_update_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::UpdatePolicyReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::UpdatePolicyResp,
            error::Error,
        >,
    >;

    // API interface for deleting a policy
    /// Initiates an asynchronous API call to delete a specific policy.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `DeletePolicyReq` struct that contains the necessary information to identify
    ///              the policy to be deleted, such as the policy ID or policy name.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `DeletePolicyResp` struct
    ///   indicating that the policy has been successfully deleted. On failure, it contains an `Error`
    ///   struct indicating the reason for the deletion failure, such as the policy not existing,
    ///   the policy being in use, or insufficient permissions.
    fn new_delete_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::DeletePolicyReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::DeletePolicyResp,
            error::Error,
        >,
    >;

    // API interface for attaching a policy to a user
    /// Initiates an asynchronous API call to attach a specific policy to a user.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: An `AttachUserPolicyReq` struct that contains the information about the user
    ///              and the policy to be attached, such as the user ID and the policy ID.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains an `AttachUserPolicyResp`
    ///   struct indicating that the policy has been successfully attached to the user. On failure,
    ///   it contains an `Error` struct indicating the reason for the attachment failure, such as
    ///   the user or policy not existing, the policy already being attached, or insufficient permissions.
    fn new_attach_user_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::AttachUserPolicyReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::AttachUserPolicyResp,
            error::Error,
        >,
    >;

    // API interface for listing attached policies of a user
    /// Initiates an asynchronous API call to list all the policies attached to a specific user.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `ListAttachedUserPoliciesReq` struct that contains the information about the user
    ///              for whom the attached policies are to be listed, such as the user ID.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `ListAttachedUserPoliciesResp`
    ///   struct that includes a list of policies attached to the user, with details like policy ID,
    ///   policy name, etc. On failure, it contains an `Error` struct indicating the reason for the
    ///   listing failure, such as the user not existing or insufficient permissions.
    fn new_list_attach_user_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::ListAttachedUserPoliciesReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::ListAttachedUserPoliciesResp,
            error::Error,
        >,
    >;

    // API interface for detaching a policy from a user
    /// Initiates an asynchronous API call to detach a specific policy from a user.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `DetachUserPolicyReq` struct that holds the necessary information to identify
    ///              the user and the policy to be detached. This typically includes the user ID and
    ///              the policy ID.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `DetachUserPolicyResp` struct
    ///   indicating that the policy has been successfully detached from the user. On failure, it contains
    ///   an `Error` struct with details about the failure, such as the user or policy not existing,
    ///   the policy not being attached to the user in the first place, or insufficient permissions.
    fn new_detach_user_policy(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_policy::DetachUserPolicyReq,
    ) -> impl Future<
        Output = Result<
            volcengine_sdk_protobuf::protobuf::iam_policy::DetachUserPolicyResp,
            error::Error,
        >,
    >;

    // API interface for deleting a user
    /// Initiates an asynchronous API call to delete a specific user.
    ///
    /// # Arguments
    /// - `&self`: A reference to the current instance implementing the `IamService` trait.
    /// - `request`: A `DeleteUserReq` struct that contains the information needed to identify the user
    ///              to be deleted, usually the user ID.
    ///
    /// # Returns
    /// - A future that resolves to a `Result`. On success, it contains a `DeleteUserResp` struct
    ///   indicating that the user has been successfully deleted. On failure, it contains an `Error`
    ///   struct explaining the reason for the deletion failure, such as the user not existing,
    ///   the user having associated resources that prevent deletion, or insufficient permissions.
    fn new_delete_user(
        &self,
        request: volcengine_sdk_protobuf::protobuf::iam_user::DeleteUserReq,
    ) -> impl Future<
        Output = Result<volcengine_sdk_protobuf::protobuf::iam_user::DeleteUserResp, error::Error>,
    >;
}

// Define the Iam struct, which is used to encapsulate client - related information.
// This struct serves as a container for the client instance, allowing for better management
// and organization of the client - side operations related to the IAM (Identity and Access Management) service.
// The `Debug` derive trait enables easy debugging by providing a default implementation of the `fmt::Debug` trait.
// The `Clone` derive trait allows for creating copies of the `Iam` struct, which can be useful in scenarios
// where multiple parts of the code need independent access to the same IAM - related client information.
#[derive(Debug, Clone)]
pub struct Iam {
    // An instance of the `client::Client` struct that contains all the necessary client - side information.
    // This may include authentication details, connection settings, and other configurations required
    // to interact with the IAM service's APIs.
    client: client::Client,
}
