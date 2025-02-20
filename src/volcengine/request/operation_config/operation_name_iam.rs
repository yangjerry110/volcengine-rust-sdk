/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-30 11:03:46
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 16:41:48
 * @Description: operation name iam
 */

/// Enum representing various operation names for the Identity and Access Management (IAM) service.
/// This enum provides a type - safe way to define and reference different operations that can be
/// performed on IAM resources. The `Debug` derive allows for easy debugging by providing a default
/// implementation of the `fmt::Debug` trait, enabling enum variants to be printed in a human - readable
/// format. The `Clone` derive allows creating copies of the enum values.
#[derive(Debug, Clone)]
pub enum OperationNameIam {
    /// Retrieves information about a specific IAM user.
    /// This operation can be used to get details such as the user's name, status, and associated metadata.
    GetUser,
    /// Updates the information of an existing IAM user.
    /// For example, you can change the user's display name or other attributes.
    UpdateUser,
    /// Creates a new IAM user.
    /// This is used to add a new entity to the IAM system with its own set of permissions and access settings.
    CreateUser,
    /// Creates a login profile for an IAM user.
    /// A login profile allows the user to log in to the console using a password.
    CreateLoginProfile,
    /// Deletes the login profile of an IAM user.
    /// After deletion, the user will no longer be able to log in to the console using a password.
    DeleteLoginProfile,
    /// Updates the login profile of an IAM user.
    /// This can be used to change the user's password or other login - related settings.
    UpdateLoginProfile,
    /// Retrieves the login profile information of an IAM user.
    /// It can be used to check the current password status or other login - profile details.
    GetLoginProfile,
    /// Retrieves the security configuration settings of the IAM service.
    /// These settings may include password policies, multi - factor authentication requirements, etc.
    GetSecurityConfig,
    /// Sets the security configuration settings of the IAM service.
    /// This allows administrators to define and enforce security policies for the IAM system.
    SetSecurityConfig,
    /// Creates a new project in the IAM service.
    /// Projects can be used to group and manage resources and users within the IAM system.
    CreateProject,
    /// Retrieves information about a specific project in the IAM service.
    /// This can be used to get details such as the project's name, description, and associated users.
    GetProject,
    /// Creates a new IAM policy.
    /// Policies define the permissions that users or groups can have on various resources.
    CreatePolicy,
    /// Retrieves information about a specific IAM policy.
    /// This includes details about the policy's statements, which define the allowed or denied actions.
    GetPolicy,
    /// Lists all the IAM policies available in the system.
    /// This operation can be useful for administrators to manage and review the existing policies.
    ListPolicies,
    /// Updates an existing IAM policy.
    /// This can be used to modify the policy's statements to change the permissions it grants.
    UpdatePolicy,
    /// Deletes an existing IAM policy.
    /// After deletion, the policy will no longer be available, and any associated users or groups
    /// will lose the permissions defined by that policy.
    DeletePolicy,
    /// Attaches an IAM policy to a user.
    /// This grants the user the permissions defined in the policy.
    AttachUserPolicy,
    /// Lists all the policies attached to a specific IAM user.
    /// This helps in understanding the permissions that a user currently has.
    ListAttachedUserPolicies,
    /// Detaches an IAM policy from a user.
    /// This revokes the permissions that the user had due to the attached policy.
    DetachUserPolicy,
    /// Deletes an existing IAM user from the system.
    /// This removes the user and all associated permissions and settings.
    DeleteUser,
}

/// Implementation of the `ToString` trait for the `OperationNameIam` enum.
/// This implementation enables converting an `OperationNameIam` instance into a string representation.
/// It is useful when passing the operation name as a parameter in API calls or for logging purposes.
impl ToString for OperationNameIam {
    /// Converts an `OperationNameIam` instance into a string.
    ///
    /// # Returns
    /// - A `String` corresponding to the operation name. Each enum variant maps to a specific string
    ///   that represents the actual operation name used in the IAM API.
    fn to_string(&self) -> String {
        match self {
            OperationNameIam::GetUser => "GetUser",
            OperationNameIam::CreateUser => "CreateUser",
            OperationNameIam::UpdateUser => "UpdateUser",
            OperationNameIam::CreateLoginProfile => "CreateLoginProfile",
            OperationNameIam::UpdateLoginProfile => "UpdateLoginProfile",
            OperationNameIam::DeleteLoginProfile => "DeleteLoginProfile",
            OperationNameIam::GetLoginProfile => "GetLoginProfile",
            OperationNameIam::GetSecurityConfig => "GetSecurityConfig",
            OperationNameIam::SetSecurityConfig => "SetSecurityConfig",
            OperationNameIam::CreateProject => "CreateProject",
            OperationNameIam::GetProject => "GetProject",
            OperationNameIam::CreatePolicy => "CreatePolicy",
            OperationNameIam::GetPolicy => "GetPolicy",
            OperationNameIam::ListPolicies => "ListPolicies",
            OperationNameIam::UpdatePolicy => "UpdatePolicy",
            OperationNameIam::DeletePolicy => "DeletePolicy",
            OperationNameIam::AttachUserPolicy => "AttachUserPolicy",
            OperationNameIam::ListAttachedUserPolicies => "ListAttachedUserPolicies",
            OperationNameIam::DetachUserPolicy => "DetachUserPolicy",
            OperationNameIam::DeleteUser => "DeleteUser",
        }
        // Convert the string literal to a `String` type
        .to_string()
    }
}
