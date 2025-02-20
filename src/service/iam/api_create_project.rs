/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 14:39:42
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:35:14
 * @Description: api create project
 */
use crate::service::iam::Iam;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::iam_project;

/// Struct for creating a project in IAM.
/// This struct encapsulates the logic for making API requests to create a new project.
pub struct ApiCreateProjectIam;

/// Implementation of methods for the ApiCreateProjectIam struct.
/// These methods handle the logic for creating a project, including constructing requests and processing responses.
impl ApiCreateProjectIam {
    /// Creates a new API request to create a project in IAM.
    /// This method is the primary entry point for initiating the project creation request.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiCreateProjectIam`.
    /// - `iam`: A reference to the `Iam` struct, which contains the necessary IAM client information.
    /// - `request`: A `CreateProjectReq` struct containing the parameters to create a project.
    ///
    /// # Returns:
    /// A `Result` containing a `CreateProjectResp` on success or an `error::Error` on failure.
    pub async fn new_create_project(
        &self,
        iam: &Iam,
        request: iam_project::CreateProjectReq,
    ) -> Result<iam_project::CreateProjectResp, error::Error> {
        // Delegates the request creation and sending to the internal helper function.
        self.new_create_project_request(iam, request).await
    }

    /// Internal function to create and send the request to create a project in IAM.
    /// This function constructs the request operation and sends it using the Volcengine request system.
    ///
    /// # Parameters:
    /// - `&self`: A reference to the current instance of `ApiCreateProjectIam`.
    /// - `iam`: A reference to the `Iam` struct, which contains the necessary IAM client information.
    /// - `request`: A `CreateProjectReq` struct containing the parameters to create a project.
    ///
    /// # Returns:
    /// A `Result` containing a `CreateProjectResp` on success or an `error::Error` on failure.
    async fn new_create_project_request(
        &self,
        iam: &Iam,
        request: iam_project::CreateProjectReq,
    ) -> Result<iam_project::CreateProjectResp, error::Error> {
        // Define the request operation with necessary configurations.
        // The `operation_name` corresponds to the IAM "CreateProject" operation.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::IamOperation(
                    operation_config::operation_name_iam::OperationNameIam::CreateProject,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?; // Build the operation and return a Result

        // Set up the request using the client information from the `iam` struct.
        // The request is built with operation details and IAM client configuration.
        // After that, the request is sent using the `send` method and the response is processed.
        let response = request::Request::builder()
            .with_client_info(&iam.client.client_info) // Attach client information (e.g., credentials)
            .with_config(&iam.client.config) // Include configuration settings for the IAM client
            .with_handles(&iam.client.handles) // Use the appropriate IAM client handles for this operation
            .with_operation(&request_operation) // Attach the operation definition (CreateProject)
            .build()? // Build the request
            .send(request) // Send the request to IAM with the provided `CreateProjectReq`
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp = volcengine_sdk_protobuf::protobuf::iam_project::CreateProjectResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
