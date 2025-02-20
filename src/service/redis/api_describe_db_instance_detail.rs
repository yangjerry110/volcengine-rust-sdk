/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 10:47:33
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 14:30:42
 * @Description: API for describing the details of a Redis database instance.
 */
use crate::service::redis;
use crate::volcengine::error::error;
use crate::volcengine::request::operation;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::response::ApiResponse;
use volcengine_sdk_protobuf::protobuf::redis_instance;

/// A struct representing the API for describing the details of a Redis database instance.
/// This struct encapsulates the functionality required to send a request to the Volcengine Redis service
/// to retrieve detailed information about a specific Redis database instance.
pub struct ApiDescribeDBInstanceDetailRedis;

/// Implementation of methods for the `ApiDescribeDBInstanceDetailRedis` struct.
/// This implementation provides the necessary logic to construct and send a request to the Volcengine Redis service
/// to describe the details of a Redis database instance, as well as handle the response.
impl ApiDescribeDBInstanceDetailRedis {
    /// Public method to describe the details of a Redis database instance.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiDescribeDBInstanceDetailRedis`.
    /// - `redis`: Reference to a `Redis` instance, which contains client information, configuration, and handles.
    /// - `request`: A `DescribeDbInstanceDetailReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<redis_instance::DescribeDbInstanceDetailResp, error::Error>`: On success, returns a `DescribeDbInstanceDetailResp` structure containing the response from the Redis service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    pub async fn new_describe_db_instance(
        &self,
        redis: &redis::Redis,
        request: redis_instance::RedisDescribeDbInstanceDetailReq,
    ) -> Result<redis_instance::RedisDescribeDbInstanceDetailResp, error::Error> {
        // Delegate the request handling to the private method `new_describe_db_instance_request`.
        self.new_describe_db_instance_request(redis, request).await
    }

    /// Private method to handle the request to describe the details of a Redis database instance.
    ///
    /// This method constructs the request operation, builds the request, sends it to the Volcengine Redis service,
    /// and parses the response.
    ///
    /// # Arguments
    /// - `&self`: Reference to the current instance of `ApiDescribeDBInstanceDetailRedis`.
    /// - `redis`: Reference to a `Redis` instance, which contains client information, configuration, and handles.
    /// - `request`: A `DescribeDbInstanceDetailReq` structure containing the request parameters.
    ///
    /// # Returns
    /// - `Result<redis_instance::DescribeDbInstanceDetailResp, error::Error>`: On success, returns a `DescribeDbInstanceDetailResp` structure containing the response from the Redis service.
    ///   On failure, returns an `error::Error` indicating the cause of the failure.
    async fn new_describe_db_instance_request(
        &self,
        redis: &redis::Redis,
        request: redis_instance::RedisDescribeDbInstanceDetailReq,
    ) -> Result<redis_instance::RedisDescribeDbInstanceDetailResp, error::Error> {
        // Define the request operation with the specific operation name, HTTP method, and path.
        // The operation name corresponds to the "DescribeDBInstances" action in the Volcengine Redis service.
        let request_operation = operation::Operation::builder()
            .with_operation_name(
                operation_config::operation_name::OperationName::RedisOperation(
                    operation_config::operation_name_redis::OperationNameRedis::DescribeDBInstanceDetail,
                ),
            )
            .with_operation_http_method(
                operation_config::operation_http_method::OperationHttpMethod::GET,
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
        let mut resp = volcengine_sdk_protobuf::protobuf::redis_instance::RedisDescribeDbInstanceDetailResp::default();
        // Populate the response structure with data from the actual response.
        resp.to_struct(response).await?;
        // Return the structured response successfully.
        return Ok(resp);
    }
}
