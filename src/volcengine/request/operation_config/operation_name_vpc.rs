/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-30 10:56:49
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-30 11:03:30
 * @Description: operation name vpc
 */

#[derive(Debug, Clone)]
pub enum OperationNameVpc {
    DescribeVpcs,
    DescribeSubnets,
}

impl ToString for OperationNameVpc {
    fn to_string(&self) -> String {
        match self {
            OperationNameVpc::DescribeSubnets => "DescribeVpcs",
            OperationNameVpc::DescribeVpcs => "DescribeVpcs",
        }
        .to_string()
    }
}
