/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-07 10:51:34
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-07 10:53:05
 * @Description: operation name redis
 */
#[derive(Debug, Clone)]
pub enum OperationNameRedis {
    CreateDBInstance,
}

impl ToString for OperationNameRedis {
    fn to_string(&self) -> String {
        match self {
            OperationNameRedis::CreateDBInstance => "CreateDBInstance",
        }.to_string()
    }
}
