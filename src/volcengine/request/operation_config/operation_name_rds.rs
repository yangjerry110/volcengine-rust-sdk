/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:51:35
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-06 15:56:52
 * @Description: operation name rds
 */
#[derive(Debug, Clone)]
pub enum OperationNameRds {
    CreateDBInstance,
    DescribeDBInstanceDetail,
    CreateDBEndpoint,
    CreateDBAccount,
    CreateDatabase,
    ModifyAllowList,
}

impl ToString for OperationNameRds {
    fn to_string(&self) -> String {
        match self {
            OperationNameRds::CreateDBInstance => "CreateDBInstance",
            OperationNameRds::DescribeDBInstanceDetail => "DescribeDBInstanceDetail",
            OperationNameRds::CreateDBEndpoint => "CreateDBEndpoint",
            OperationNameRds::CreateDBAccount => "CreateDBAccount",
            OperationNameRds::CreateDatabase => "CreateDatabase",
            OperationNameRds::ModifyAllowList => "ModifyAllowList",
        }
        .to_string()
    }
}
