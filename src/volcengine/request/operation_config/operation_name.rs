use crate::volcengine::request::operation_config::operation_name_ecs;
use crate::volcengine::request::operation_config::operation_name_iam;
use crate::volcengine::request::operation_config::operation_name_rds;
use crate::volcengine::request::operation_config::operation_name_vpc;
use crate::volcengine::request::operation_config::operation_name_redis;

/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-30 10:50:21
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-07 10:55:39
 * @Description:
 */
// Enum representing possible operation names, such as `GetUser`, `CreateUser`
#[derive(Debug, Clone)]
pub enum OperationName {
    // iam
    IamOperation(operation_name_iam::OperationNameIam),
    // ecs
    EcsOperation(operation_name_ecs::OperationNameEcs),
    // vpc
    VpcOperation(operation_name_vpc::OperationNameVpc),
    // rds
    RdsOperation(operation_name_rds::OperationNameRds),
    // redis
    RedisOperation(operation_name_redis::OperationNameRedis),
}

impl ToString for OperationName {
    fn to_string(&self) -> String {
        match self {
            OperationName::EcsOperation(operation_name_ecs) => operation_name_ecs.to_string(),
            OperationName::IamOperation(operation_name_iam) => operation_name_iam.to_string(),
            OperationName::VpcOperation(operation_name_vpc) => operation_name_vpc.to_string(),
            OperationName::RdsOperation(operation_name_rds) => operation_name_rds.to_string(),
            OperationName::RedisOperation(operation_name_redis) => operation_name_redis.to_string(),
        }
    }
}
