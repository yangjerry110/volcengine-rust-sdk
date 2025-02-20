/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-29 10:52:40
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 11:28:38
 * @Description: Describe ECS images via an API request.
 */
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use crate::{service::ecs, volcengine::error::error};
use volcengine_sdk_protobuf::protobuf::ecs_image;

/// Struct representing the API for describing ECS images.
///
/// This struct does not hold any data but defines the necessary methods
/// for interacting with the ECS service to describe images.
pub struct ApiDescribeImagesEcs;

/**
 * The `ApiDescribeImagesEcs` struct provides methods to interact with the ECS (Elastic Compute Service)
 * API for describing images. It contains methods for creating and sending requests to the ECS service to
 * retrieve image information, as well as handling the responses.
 *
 * This struct does not maintain state; rather, it provides stateless methods to interface with the ECS API.
 * The primary functionality is encapsulated in two methods:
 * 1. `new_describe_images`: A public method that initiates the request to describe ECS images.
 * 2. `new_describe_images_request`: A private method that constructs and sends the request to ECS,
 *    handling the response and any potential errors.
 *
 * The goal is to provide an abstraction layer over the ECS API, making it easier to work with image-related
 * operations in a cloud environment.
 */
impl ApiDescribeImagesEcs {
    /// Public method for describing ECS images using the DescribeImages API.
    ///
    /// This method initiates the request to the ECS API, passing in the required parameters
    /// contained in the `DescribeImagesReq` struct. It calls the private method `new_describe_images_request`
    /// to actually send the request and handle the response.
    ///
    /// The method returns a `Result`, which either contains:
    /// - `DescribeImagesResp` on success, or
    /// - `Error` on failure (if something goes wrong during the request or response processing).
    ///
    /// # Parameters
    /// - `&self`: The current instance of `ApiDescribeImagesEcs`.
    /// - `ecs`: A reference to an `Ecs` struct that holds the client configuration.
    /// - `request`: The `DescribeImagesReq` struct containing the request parameters.
    ///
    /// # Returns
    /// - `Result<ecs_image::DescribeImagesResp, error::Error>`:
    ///     - `Ok(DescribeImagesResp)` on success, or
    ///     - `Err(error::Error)` on failure.
    pub async fn new_describe_images(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_image::DescribeImagesReq,
    ) -> Result<ecs_image::DescribeImagesResp, error::Error> {
        self.new_describe_images_request(ecs, request).await
    }

    /// Private helper method for preparing and sending the DescribeImages request.
    ///
    /// This method constructs the request operation, specifying the operation name, HTTP method, and HTTP path.
    /// It then builds the request, sends it to the ECS service, and processes the response.
    ///
    /// The method uses a GET request to interact with the ECS service and handles any errors
    /// that may occur during the request building, sending, or response parsing.
    ///
    /// # Parameters
    /// - `&self`: The current instance of `ApiDescribeImagesEcs`.
    /// - `ecs`: A reference to an `Ecs` struct that holds client information and configuration.
    /// - `request`: The `DescribeImagesReq` struct containing the request parameters.
    ///
    /// # Returns
    /// - `Result<ecs_image::DescribeImagesResp, error::Error>`:
    ///     - `Ok(DescribeImagesResp)` if the request is successful.
    ///     - `Err(error::Error)` if any error occurs during the request or response processing.
    async fn new_describe_images_request(
        &self,
        ecs: &ecs::Ecs,
        request: ecs_image::DescribeImagesReq,
    ) -> Result<ecs_image::DescribeImagesResp, error::Error> {
        // Create the request operation object, specifying the ECS operation details.
        // This section constructs an `Operation` object, specifying the API endpoint details for querying ECS (Elastic Compute Service) images.
        let request_operation = operation::Operation::builder()
            // Set the operation name to "DescribeImages" which corresponds to the ECS API for describing images.
            .with_operation_name(
                operation_config::operation_name::OperationName::EcsOperation(
                    operation_config::operation_name_ecs::OperationNameEcs::DescribeImages,
                ),
            )
            // Specify the HTTP method for this operation. In this case, it's a GET request.
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
            )
            // Set the HTTP path for the operation. Here, the default path is used.
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            // Build the operation object, ensuring all configurations are properly set.
            .build()?;

        // Build and send the request
        // This section constructs the request object using the previously defined operation and client configurations, then sends the request.
        let response = request::Request::builder()
            // Include client information from the ECS client.
            .with_client_info(&ecs.client.client_info)
            // Include configuration details from the ECS client.
            .with_config(&ecs.client.config)
            // Include handles (e.g., for logging, error handling) from the ECS client.
            .with_handles(&ecs.client.handles)
            // Attach the previously defined operation to this request.
            .with_operation(&request_operation)
            // Build the request object.
            .build()?
            // Send the request and await the response.
            .send(request)
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for ECS image descriptions.
        let mut resp = volcengine_sdk_protobuf::protobuf::ecs_image::DescribeImagesResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
