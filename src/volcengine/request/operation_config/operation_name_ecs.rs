/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-30 11:05:43
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-30 11:16:05
 * @Description: operation name ecs
 */
#[derive(Debug, Clone)]
pub enum OperationNameEcs {
    RunInstances,
    DescribeImages,
    DescribeInstances,
    DescribeRegions,
    DescribeZones,
    ModifyInstanceSpec,
    StopInstance,
    StopInstances,
}

impl ToString for OperationNameEcs {
    fn to_string(&self) -> String {
        match self {
            OperationNameEcs::RunInstances => "RunInstances",
            OperationNameEcs::DescribeImages => "DescribeImages",
            OperationNameEcs::DescribeInstances => "DescribeInstances",
            OperationNameEcs::DescribeRegions => "DescribeRegions",
            OperationNameEcs::DescribeZones => "DescribeZones",
            OperationNameEcs::ModifyInstanceSpec => "ModifyInstanceSpec",
            OperationNameEcs::StopInstance => "StopInstance",
            OperationNameEcs::StopInstances => "StopInstances",
        }
        .to_string()
    }
}
