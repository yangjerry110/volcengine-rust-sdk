/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-30 11:03:46
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-06 14:46:27
 * @Description: operation name iam
 */
#[derive(Debug, Clone)]
pub enum OperationNameIam {
    GetUser,
    UpdateUser,
    CreateUser,
    CreateLoginProfile,
    DeleteLoginProfile,
    UpdateLoginProfile,
    GetLoginProfile,
    GetSecurityConfig,
    SetSecurityConfig,
    CreateProject,
    GetProject,
    CreatePolicy,
    GetPolicy,
    ListPolicies,
    UpdatePolicy,
    DeletePolicy,
    AttachUserPolicy,
    ListAttachedUserPolicies,
    DetachUserPolicy,
}

impl ToString for OperationNameIam {
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
        }
        .to_string()
    }
}
