/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 15:59:33
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:31:42
 * @Description: API for modifying the allow list of a Redis database instance.
 */
use crate::service::redis;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::redis_allow;

/// A struct representing the API for modifying the allow list of a Redis database instance.
/// This struct encapsulates the functionality required to send a request to the Volcengine Redis service
/// to modify the allow list of a specific Redis database instance.
pub struct ApiModifyAllowListRedis;

/// Implementation of methods for the `ApiModifyAllowListRedis` struct.
/// This implementation provides the necessary logic to construct and send a request to the Volcengine Redis service
/// to modify the allow list of a Redis database instance, as well as handle the response.
impl ApiModifyAllowListRedis {
    /// Public method to modify the allow list of a Redis database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiModifyAllowListRedis`.
    /// - `redis`: Reference to a `Redis` instance, which contains client information, configuration, and handles.
    /// - `request`: A `ModifyAllowListReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<redis_allow::ModifyAllowListResp, error::Error>`: On success, returns a `ModifyAllowListResp` structure containing the response from the Redis service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    pub async fn new_modify_allow_list(
        &self,
        redis: &redis::Redis,
        request: redis_allow::RedisModifyAllowListReq,
    ) -> Result<redis_allow::RedisModifyAllowListResp, error::Error> {
        // Delegate the request handling to the private method `new_modify_allow_list_request`.
        self.new_modify_allow_list_request(redis, request).await
    }

    /// Private method to handle the request to modify the allow list of a Redis database instance.
    ///
    /// This method constructs the request operation, builds the request, sends it to the Volcengine Redis service,
    /// and parses the response.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiModifyAllowListRedis`.
    /// - `redis`: Reference to a `Redis` instance, which contains client information, configuration, and handles.
    /// - `request`: A `ModifyAllowListReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<redis_allow::ModifyAllowListResp, error::Error>`: On success, returns a `ModifyAllowListResp` structure containing the response from the Redis service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_modify_allow_list_request(
        &self,
        redis: &redis::Redis,
        request: redis_allow::RedisModifyAllowListReq,
    ) -> Result<redis_allow::RedisModifyAllowListResp, error::Error> {
        // Define the request operation with the specific operation name, HTTP method, and path.
        // The operation name corresponds to the "ModifyAllowList" action in the Volcengine Redis service.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::RedisOperation(
                    operation_config::operation_name_redis::OperationNameRedis::ModifyAllowList,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::POST,
            )
            .with_operation_http_path(
                operation_config::operation_http_path::OperationHttpPath::Default,
            )
            .build()?;

        // Build the Volcengine request using the client information, configuration, handles, and the defined operation.
        let response = request::Request::builder()
            .with_client_info(&redis.client.client_info)
            .with_config(&redis.client.config)
            .with_handles(&redis.client.handles)
            .with_operation(&request_operation)
            .build()?
            .send(request)
            .await?;

        // Convert the response into a structured format defined by the SDK's protobuf definitions.
        // Initialize a default response structure for load balancer descriptions.
        let mut resp =
            volcengine_sdk_protobuf::protobuf::redis_allow::RedisModifyAllowListResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
